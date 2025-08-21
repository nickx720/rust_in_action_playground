use std::{
    io::{self, Write},
    process::Command,
};

use clap::{Parser, Subcommand};
use nix::libc::SIGCHLD;
use nix::sched::{CloneFlags, clone};
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{Pid, gethostname, sethostname};

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
                if let Err(e) = sethostname("child-ns") {
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
                if let Some(arguments) = &args.run {
                    match arguments {
                        Commands::Run { command, args } => {
                            let output =
                                Command::new(command).arg(args.join(" ")).output().unwrap();
                            let _ = io::stdout().write_all(&output.stdout);
                            let _ = io::stderr().write_all(&output.stderr);
                        }
                    }
                }
                0 // child's exit status
            }),
            &mut stack,
            CloneFlags::CLONE_NEWUTS,
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
