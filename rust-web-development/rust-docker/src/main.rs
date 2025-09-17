use std::{
    ffi::CString,
    io::{self, Write},
    path::Path,
    process::Command,
};

use clap::{Parser, Subcommand};
use nix::mount::{self, MntFlags, MsFlags, mount, umount2};
use nix::unistd::{Pid, chroot, gethostname, sethostname};
use nix::{libc::SIGCHLD, unistd::chdir};
use nix::{
    sched::{CloneFlags, clone, unshare},
    unistd::getcwd,
};
use nix::{
    sys::wait::{WaitStatus, waitpid},
    unistd::execvp,
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
    let mut stack = vec![0u8; 512 * 1024];

    // Show parent hostname
    let parent_hn = gethostname().unwrap().to_string_lossy().into_owned();
    println!("[parent] hostname before clone: {parent_hn}");

    // Create child in a NEW UTS namespace
    let child_pid: Pid = unsafe {
        clone(
            Box::new(|| {
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
                            let result = chroot("/play").expect("Chroot failed");
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
                            let mut arguments: Vec<CString>;
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
            CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWPID,
            Some(SIGCHLD),
        )
        .unwrap()
    };

    // Wait for child and report status
    match waitpid(child_pid, None).unwrap() {
        WaitStatus::Exited(pid, code) => println!("[parent] child {pid} exited with {code}"),
        other => println!("[parent] wait status: {other:?}"),
    }

    // Parent hostname is unchanged
    let parent_after = gethostname().unwrap().to_string_lossy().into_owned();
    println!("[parent] hostname after child: {parent_after}");
}
