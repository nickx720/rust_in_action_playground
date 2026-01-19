// https://distribution.github.io/distribution/spec/api/
use std::{
    ffi::CString,
    fs::{self, File, OpenOptions},
    io::Write,
    os::{fd::AsFd, linux::fs::MetadataExt},
    path::{Component, Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
#[cfg(target_os = "linux")]
use nix::sched::{CloneFlags, clone};
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{
    Pid, chdir, chroot, close, execvp, getcwd, getgid, gethostname, getuid, pipe, read,
    sethostname, write,
};
use nix::{
    libc::SIGCHLD,
    mount::{self, MntFlags, MsFlags, mount, umount2},
};

use crate::fetch::get_docker_manifest;
mod fetch;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    run: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        #[arg(short, long)]
        command: String,
        #[arg(short, long,num_args=1..)]
        args: Vec<String>,
    },
}

pub fn write_file(path: &str, data: &str) -> std::io::Result<()> {
    let mut f = OpenOptions::new().write(true).create(true).open(path)?;
    f.write_all(data.as_bytes())
}

fn install_uid_gid_map_for_child(child_pid: Pid, ruid: u32, guid: u32) -> Result<()> {
    let setgroups_path = format!("/proc/{}/setgroups", child_pid);
    let _ = write_file(&setgroups_path, "deny\n");

    let uid_map_path = format!("/proc/{}/uid_map", child_pid);
    let gid_map_path = format!("/proc/{}/gid_map", child_pid);
    write_file(&uid_map_path, &format!("0 {} 1\n", ruid))
        .with_context(|| format!("writing {}", uid_map_path))?;
    write_file(&gid_map_path, &format!("0 {} 1\n", guid))
        .with_context(|| format!("writing {}", guid))?;
    Ok(())
}

fn setup_resources(child_pid: Pid, uid: u32, gid: u32) -> Result<()> {
    let cgroup_root = "/sys/fs/cgroup";
    if !Path::new(cgroup_root).exists() {
        return Err(anyhow::anyhow!(
            "Cgroup root {} does not exist. Ensure cgroup v2 is mounted.",
            cgroup_root
        ));
    }
    let path = "/sys/fs/cgroup/limited_mem";
    if !fs::metadata(path).is_ok() {
        fs::create_dir(path).context("Failed to create cgroup directory")?;
        println!("Created cgroup directory: {}", path);
    } else {
        println!("Cgroup directory already exists: {}", path);
    }
    let metadata = fs::metadata(path).context("Failed to get metadata for cgroup directory")?;
    let file_uid = metadata.st_uid();
    let file_gid = metadata.st_gid();
    if uid == file_uid || gid == file_gid {
        println!("Process has ownership access to cgroup directory.");
    } else {
        println!(
            "Warning: Process may not have ownership access. UID: {}, GID: {}, File UID: {}, File GID: {}",
            uid, gid, file_uid, file_gid
        );
    }
    let memory_max_path = format!("{}/memory.max", path);
    if Path::new(&memory_max_path).exists() {
        println!("File already exists, skipping the creation");
    } else {
        let mut file =
            File::create(&memory_max_path).context("Failed to create memory.max file")?;
        file.write_all(b"100M")
            .context("Failed to write to memory.max")?;
        println!("Set memory limit to 100M in {}", memory_max_path);
    }
    let cgroup_path = format!("{}/cgroup.procs", path);
    let mut file = File::create(cgroup_path)?;
    file.write_all(child_pid.as_raw().to_string().as_bytes())?;
    Ok(())
}

// Figure out how to isolate process running inside container from host
// https://www.man7.org/linux/man-pages/man7/user_namespaces.7.html
fn main() {
    let args = Args::parse();
    let (sync_r, sync_w) = pipe().unwrap();
    let mut stack = vec![0u8; 512 * 1024];

    // Show parent hostname
    let parent_hn = gethostname().unwrap().to_string_lossy().into_owned();
    let src_path = Path::new("/mnt/hgfs/rust-docker/dist/output");
    get_docker_manifest(&src_path).expect("It failed");

    println!("[parent] hostname before clone: {parent_hn}");
    // Create child in a NEW UTS namespace
    let child_pid: Pid = unsafe {
        clone(
            Box::new(move || {
                //TODO : limit memory
                let mut buf = [0u8; 1];
                let _ = read(&sync_r, &mut buf);
                if let Err(e) = sethostname("container") {
                    eprintln!("[child] sethostname failed: {e} (need CAP_SYS_ADMIN in this ns)");
                }
                let h = match gethostname() {
                    Ok(s) => s.to_string_lossy().into_owned(),
                    Err(e) => {
                        eprintln!("[child] gethostname failed: {e}");
                        return 1; // non-zero exit from the child
                    }
                };
                println!("Child host name {}", h);
                // mounting a new mount namespace
                if let Some(arguments) = &args.run {
                    match arguments {
                        Commands::Run { command, args } => {
                            #[cfg(target_os = "linux")]
                            mount(
                                None::<&str>,
                                Path::new("/"),
                                None::<&str>,
                                MsFlags::MS_REC | MsFlags::MS_PRIVATE,
                                None::<&str>,
                            )
                            .expect("Unable to run");
                            #[cfg(target_os = "linux")]
                            mount(
                                Some(Path::new("/play")),
                                Path::new("/play"),
                                None::<&str>,
                                MsFlags::MS_BIND | MsFlags::MS_REC,
                                None::<&str>,
                            )
                            .expect("Unable to run");
                            // TODO
                            // The following will get the zip files
                            // get_docker_manifest(src_path).expect("It failed");
                            let target = Path::new("/play");
                            #[cfg(target_os = "linux")]
                            let mounted = mount(
                                Some(src_path),
                                target,
                                None::<&str>,
                                MsFlags::MS_BIND | MsFlags::MS_REC,
                                None::<&str>,
                            );
                            let _ = chroot("/play").expect("Chroot failed");
                            chdir("/").expect("Unable to set directory");
                            #[cfg(target_os = "linux")]
                            let _ = mount(
                                Some(Path::new("proc")),
                                Path::new("/proc"),
                                Some(Path::new("proc")),
                                MsFlags::empty(),
                                None::<&str>,
                            );
                            dbg!(getcwd().unwrap().display());
                            let shell = CString::new(command.to_string()).unwrap();
                            let arguments: Vec<CString>;
                            if !args.is_empty() {
                                let args = CString::new(args.join(" ").to_string()).unwrap();
                                arguments = vec![shell.clone(), args.clone()];
                            } else {
                                arguments = vec![shell.clone()];
                            }
                            println!("The pid before execvp {}", nix::unistd::getpid());
                            execvp(&shell, &arguments.to_owned()).expect("execvp failed");
                            #[cfg(target_os = "linux")]
                            umount2("/proc", MntFlags::MNT_DETACH).expect("Coudn't unmount");
                        }
                    }
                }
                0 // child's exit status
            }),
            &mut stack,
            CloneFlags::CLONE_NEWUTS
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWUSER,
            Some(SIGCHLD),
        )
        .unwrap()
    };
    let _ = write(sync_w.as_fd(), &[1u8]);
    let _ = close(sync_w);
    let uid = getuid().as_raw();
    let gid = getgid().as_raw();
    let _ = install_uid_gid_map_for_child(child_pid, uid, gid).expect("Didn't install uid or gid");
    if let Err(response) = setup_resources(child_pid, uid, gid) {
        eprintln!("Unable to run {}", response);
        std::process::exit(1);
    }
    // Wait for child and report status
    match waitpid(child_pid, None).unwrap() {
        WaitStatus::Exited(pid, code) => println!("[parent] child {pid} exited with {code}"),
        other => println!("[parent] wait status: {other:?}"),
    }

    // Parent hostname is unchanged
    let parent_after = gethostname().unwrap().to_string_lossy().into_owned();
    println!("[parent] hostname after child: {parent_after}");
}
