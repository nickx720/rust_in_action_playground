//
//### 2. Mount Namespace Isolation
//
//• You use CLONE_NEWNS in clone.
//• This means the child process is in a new mount namespace.
//• However, you should also remount / as private (mount(NULL, "/", NULL, MS_REC|MS_PRIVATE, NULL))
//to prevent mount propagation to the host.
//• You do not currently mount /proc in the new namespace, which is required for ps to work as
//expected.
//
//### 3. Mounting /proc
//
//• Missing:
//You do not call mount("proc", "/proc", "proc", 0, NULL) in the child.
// • Without this, /proc in the container will be the host’s /proc, or may not be mounted at all
// (depending on chroot).
// • You must mount /proc after entering the new mount namespace and after chrooting (if you chroot).
//
//
//### 4. Unmounting /proc on Exit
//
//• Missing:
//You do not unmount /proc on container exit.
// • This is important for cleanup.
//
//
//### 5. Validation
//
//• If you add the missing /proc mount, running ps in the container will show only container
//processes, with your command as PID 1.
//• On the host, the container’s /proc will not be visible in mount | grep proc if you use a new
//mount namespace and remount / as private.
use std::{
    ffi::CString,
    io::{self, Write},
    path::Path,
    process::Command,
};

use clap::{Parser, Subcommand};
use nix::mount::{self, MsFlags, mount};
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
                // --- child process ---
                // https://docs.rs/nix/latest/nix/mount/fn.mount.html
                // TODO setup new process using CLONE_NEWPID
                // https://lwn.net/Articles/531419/

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
                                Some(Path::new("/play")),
                                Path::new("/play"),
                                None::<&str>,
                                MsFlags::MS_BIND | MsFlags::MS_REC,
                                None::<&str>,
                            )
                            .expect("Unable to run");
                            let result = chroot("/play").expect("Chroot failed");
                            chdir("/").expect("Unable to set directory");
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
                            //                            let cmd = Command::new(command).args(args).spawn();
                            //                            match cmd {
                            //                                Ok(val) => {
                            //                                    println!("Works");
                            //                                }
                            //                                Err(e) => {
                            //                                    eprintln!("Something went wrong {}", e)
                            //                                }
                            //                            }
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
