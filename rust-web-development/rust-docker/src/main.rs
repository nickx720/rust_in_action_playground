// To limit resources (memory/CPU) for the cloned child process in your Rust code using Nix, you can modify
//cgroup controller files after cloning but before executing the child process. Here's a high-level
//explanation:
//
//1. Set up cgroups: After cloning (around line 72-146), create or join a cgroup for the child PID (e.g.,
//via /sys/fs/cgroup/memory/ and /sys/fs/cgroup/cpu/). Write limits to files like memory.limit_in_bytes (e.g.,
//"100M" for 100MB) and cpu.cfs_quota_us (e.g., "50000" for 50% CPU).
//2. Modify controller files: Use Nix's file I/O functions (e.g., std::fs::write) to set values, ensuring
//the cgroup exists first. For example, add code to write to /sys/fs/cgroup/memory/mygroup/memory.
//limit_in_bytes.
//3. Enable/check controllers: Cgroup controllers must be mounted and enabled system-wide (not in code).
//Check if mounted with mount | grep cgroup or enable via /proc/cgroups. To enable, run sudo mount -t
//cgroup2 none /sys/fs/cgroup if using cgroup v2, or configure in /etc/fstab for persistence. Your code
//can't enable them directly; it assumes they're set up externally.
//
//
//
//## 🧠 Primer: cgroup v2 Basics
//
//In cgroup v2, resource limits (memory, CPU, IO, etc.) are controlled through **unified hierarchy** under `/sys/fs/cgroup/`.
//
//Each control file (like `memory.max`, `cpu.max`, etc.) lives inside a cgroup directory, and processes are attached to that cgroup by writing their PIDs into the `cgroup.procs` file.
//
//---
//
//## 🧩 Example: Limit memory for a child PID
//
//Let’s say you want to limit a child process (say, PID `1234`) to **100 MB** of memory.
//
//### 1. Confirm cgroup v2 is active
//
//```bash
//mount | grep cgroup2
//```
//
//If you see something like:
//
//```
//cgroup2 on /sys/fs/cgroup type cgroup2 (rw,nosuid,nodev,noexec,relatime)
//```
//
//then you’re good.
//If not, you can remount:
//
//```bash
//mount -t cgroup2 none /sys/fs/cgroup
//```
//
//---
//
//### 2. Create a new cgroup
//
//```bash
//mkdir /sys/fs/cgroup/limited_mem
//```
//
//This directory represents a new cgroup named `limited_mem`.
//
//---
//
//### 3. Set the memory limit
//
//```bash
//echo 100M > /sys/fs/cgroup/limited_mem/memory.max
//```
//
//You can verify:
//
//```bash
//cat /sys/fs/cgroup/limited_mem/memory.max
//# Output: 104857600
//```
//
//---
//
//### 4. Attach the process (child PID)
//
//If your child PID is `1234`:
//
//```bash
//echo 1234 > /sys/fs/cgroup/limited_mem/cgroup.procs
//```
//
//This moves the process (and its threads) into this new memory-limited cgroup.
//
//---
//
//### 5. Verify it’s applied
//
//Check which cgroup the process belongs to:
//
//```bash
//cat /proc/1234/cgroup
//```
//
//You should see something like:
//
//```
//0::/limited_mem
//```
//
//---
//
//## 🧪 Optional: Launch a process directly in a cgroup
//
//Instead of moving an existing process, you can start one directly:
//
//```bash
//mkdir /sys/fs/cgroup/test_mem
//echo 200M > /sys/fs/cgroup/test_mem/memory.max
//
//# Run a process and add its PID
//sh -c "echo $$ > /sys/fs/cgroup/test_mem/cgroup.procs; exec stress --vm 1 --vm-bytes 300M --vm-keep"
//```
//
//This will get OOM-killed once it exceeds 200 MB.
//
//---
//
//## ⚙️ Troubleshooting on Alpine
//
//1. Alpine minimal images sometimes **don’t mount cgroup2** automatically.
//   You can add to `/etc/fstab`:
//
//   ```
//   none /sys/fs/cgroup cgroup2 defaults 0 0
//   ```
//
//2. Make sure the kernel boot line includes:
//
//   ```
//   systemd.unified_cgroup_hierarchy=1
//   ```
//
//   (or `cgroup_no_v1=all` if you want only v2)
//
//3. Some older Alpine kernels (<5.10) had incomplete memory controller support in cgroup v2 — verify with:
//
//   ```bash
//   cat /sys/fs/cgroup/cgroup.controllers
//   ```
//
//   You should see:
//
//   ```
//   cpuset cpu io memory pids
//   ```
//
//   If `memory` isn’t there, your kernel is missing that controller.
//
//---
//
//## 🧰 Summary Cheat Sheet
//
//| Action              | File                     | Example Value  |
//| ------------------- | ------------------------ | -------------- |
//| Set memory limit    | `memory.max`             | `100M`         |
//| Set swap limit      | `memory.swap.max`        | `50M`          |
//| Check current usage | `memory.current`         | *(bytes)*      |
//| Move process        | `cgroup.procs`           | `echo <pid>`   |
//| Check child cgroups | `cgroup.subtree_control` | `+memory +cpu` |
//
//---
//
//Would you like me to show the **Rust or shell script version** that dynamically creates a child process and places it under a cgroup2 memory limit (so you can use it programmatically)?
//
use std::{
    ffi::CString,
    fmt::format,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    os::fd::AsFd,
    path::Path,
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

fn write_file(path: &str, data: &str) -> std::io::Result<()> {
    let mut f = OpenOptions::new().write(true).open(path)?;
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

fn setup_resources() -> Result<()> {
    let path = "/sys/fs/cgroup/limited_mem";
    if !fs::metadata(path).is_ok() {
        fs::create_dir(path).expect("Creation error")
    }
    let mut file = File::create(format!("{}/limited_mem/memory.max", path))?;
    file.write_all(b"100M");
    dbg!(file);
    Ok(())
}

// Figure out how to isolate process running inside container from host
// https://www.man7.org/linux/man-pages/man7/user_namespaces.7.html
fn main() {
    let args = Args::parse();
    let (sync_r, sync_w) = pipe().unwrap();
    setup_resources();
    let mut stack = vec![0u8; 512 * 1024];

    // Show parent hostname
    let parent_hn = gethostname().unwrap().to_string_lossy().into_owned();
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
    let guid = getgid().as_raw();
    install_uid_gid_map_for_child(child_pid, ruid, guid);
    // Wait for child and report status
    match waitpid(child_pid, None).unwrap() {
        WaitStatus::Exited(pid, code) => println!("[parent] child {pid} exited with {code}"),
        other => println!("[parent] wait status: {other:?}"),
    }

    // Parent hostname is unchanged
    let parent_after = gethostname().unwrap().to_string_lossy().into_owned();
    println!("[parent] hostname after child: {parent_after}");
}
