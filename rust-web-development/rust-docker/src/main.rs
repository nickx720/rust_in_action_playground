use std::{
    ffi::CString,
    io::{self, Read, Write},
    os::fd::AsFd,
    path::Path,
    process::Command,
};

use clap::{Parser, Subcommand};
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

// Figure out how to isolate process running inside container from host
// https://www.man7.org/linux/man-pages/man7/user_namespaces.7.html
fn main() {
    let args = Args::parse();
    let (sync_r, sync_w) = pipe().unwrap();
    let mut stack = vec![0u8; 512 * 1024];

    // Show parent hostname
    let parent_hn = gethostname().unwrap().to_string_lossy().into_owned();
    println!("[parent] hostname before clone: {parent_hn}");

    // Create child in a NEW UTS namespace
    let child_pid: Pid = unsafe {
        clone(
            Box::new(move || {
                let mut buf = [0u8; 1];
                let _ = read(sync_r, &mut buf);
                let _ = close(sync_r);
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
    let ruid = getuid().as_raw();
    let rgid = getgid().as_raw();
    // Wait for child and report status
    match waitpid(child_pid, None).unwrap() {
        WaitStatus::Exited(pid, code) => println!("[parent] child {pid} exited with {code}"),
        other => println!("[parent] wait status: {other:?}"),
    }

    // Parent hostname is unchanged
    let parent_after = gethostname().unwrap().to_string_lossy().into_owned();
    println!("[parent] hostname after child: {parent_after}");
}
