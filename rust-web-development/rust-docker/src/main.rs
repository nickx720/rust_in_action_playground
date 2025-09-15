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
                // You’re *very* close. The behavior in your screenshot (seeing the host’s processes) almost always means one thing:

                //**`ps` is still reading the host’s `/proc`, not a `/proc` that’s tied to the new PID namespace.**
                //
                //A few focused hints to get you to the “three-PIDs only” view without giving away the full patch:
                //
                //* **Mount the `proc` FS *inside the child after it’s in the new PID namespace*.**
                //  With `CLONE_NEWPID`, the child becomes PID 1 *in that namespace*, but `/proc` keeps pointing at whatever was mounted in the parent (the initial PID ns). Unless you remount `proc` from inside the child, `ps` will show host PIDs.
                //
                //* **Do it after you’ve isolated mounts.**
                //  You’re creating a new mount namespace (`CLONE_NEWNS`) and you’re setting `/` to `MS_PRIVATE|MS_REC` (good—prevents propagation back to host). Now ensure you mount `proc` *in that private mount tree*, not the shared one inherited from the host.
                //
                //* **Mount point exists where `ps` expects it.**
                //  After your `chroot("/play")` + `chdir("/")`, make sure `"/proc"` exists under that root and mount `proc` there. If you mount before the `chroot`, you’ll miss the new root; if you mount after but the directory is missing, `ps` will fall back or fail.
                //
                //* **Unmount on exit.**
                //  When PID 1 inside the new PID namespace is exiting, lazily unmount `/proc` (e.g., a `MNT_DETACH` variant) so you don’t leave a dangling mount. This also ensures `mount | grep proc` on the host doesn’t show the container’s `/proc`.
                //
                //* **Order matters.**
                //  The sequence should be: enter new UTS/Mount/PID namespaces → make mounts private → `chroot` (or `pivot_root` if you later go that route) → **mount `proc`** within that root → `exec` your command. If you mount `proc` too early or in the wrong namespace/root, you’ll keep seeing host processes.
                //
                //* **Sanity check the PID ns.**
                //  Inside the child, `getpid()` should print `1` right before you `exec`. If it doesn’t, you’re not actually in the new PID namespace.
                //
                //* **Capabilities/permissions.**
                //  `sethostname` and mounts require the right caps (`CAP_SYS_ADMIN`) in the *current* namespace. If you’re running as non-root or without caps, some calls will “succeed” in confusing ways (or log errors you’re already printing).
                //
                //If you fix just the **“mount `/proc` in the child after the `chroot`, within the new (private) mount namespace”** detail, your `ps` should collapse to the expected three entries.
                //

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
                            mount(
                                Some(Path::new("proc")),
                                Path::new("/proc"),
                                Some(Path::new("proc")),
                                MsFlags::empty(),
                                None,
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
