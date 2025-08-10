#![allow(dead_code, unused)]

use nix::unistd::{fork, ForkResult, Pid};
use nix::sys::{ptrace, wait::{waitpid, WaitStatus}};
use nix::sys::ptrace::Options;
use libc::ENOSYS;
use std::os::raw::c_int;
use exec;
use nix::sys::signal::Signal::SIGTRAP;

fn handle_syscall(child_pid: &Pid) {
    let regs = ptrace::getregs(*child_pid).unwrap();
    let meta_syscall = ptrace::syscall_info(*child_pid);
    if regs.rax == -ENOSYS as u64 {
        // it means that we are entering syscall so do nothing
        ();
    }
    else {
        println!("syscall - {}", regs.orig_rax);
    }
}

fn tracee_init() {
    let _ = ptrace::traceme().expect("failed to set TRACEME flag");
    let _ = exec::Command::new("ls")
        .arg("-la")
        .exec();
}

fn tracer_init(child_pid: &Pid) {
    ptrace::setoptions(*child_pid, Options::PTRACE_O_TRACESYSGOOD);

    loop {
        ptrace::syscall(*child_pid, None);
        match waitpid(*child_pid, None) {
            Ok(WaitStatus::Exited(_, _)) => {
                println!("process was finished!");
            },
            Ok(WaitStatus::Stopped(pid_t, sig_t)) => {
//                println!("process was stopped!");
                match sig_t {
                    SIGTRAP => handle_syscall(child_pid),
                    _ => (),
                }
            },
            _ => (),
        }
    }
    
}
    
fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent{child}) => {
            println!("i'm a parent!");
            tracer_init(&child);
        }
        Ok(ForkResult::Child) => {
            println!("i'm child!");
            tracee_init();
        }
        Err(_) => {
            println!("failed with fork");
        }
    }
}
