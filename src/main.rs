#![allow(dead_code, unused)]

use nix::{
    unistd::{fork, ForkResult, Pid},
    sys::ptrace,
    sys::wait::{waitpid, WaitStatus},
    sys::signal::Signal::{SIGTRAP},
};
use libc::ENOSYS;
use sysnames::Syscalls;
use exec;

fn handle_syscall(child_pid: &Pid) {
    let regs = ptrace::getregs(*child_pid).unwrap();
    let meta_syscall = ptrace::syscall_info(*child_pid);
    if regs.rax == -ENOSYS as u64 {
        // it means that we are entering syscall so do nothing
        ();
    }
    else {
        let syscall_name = Syscalls::name(regs.orig_rax).unwrap();
        println!("{}({})", syscall_name, regs.orig_rax);
    }
}

fn tracee_init() {
    let _ = ptrace::traceme().expect("failed to set TRACEME flag");
    let _ = exec::Command::new("ls")
        .arg("-la")
        .exec();
}

fn tracer_init(child_pid: &Pid) {
    ptrace::setoptions(*child_pid, ptrace::Options::PTRACE_O_TRACESYSGOOD);

    loop {
        ptrace::syscall(*child_pid, None);
        match waitpid(*child_pid, None) {
            Ok(WaitStatus::Exited(_, _)) => {
                println!("process was finished!");
                break;
            },
            Ok(WaitStatus::Stopped(pid_t, sig_t)) => {
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
            tracer_init(&child);
        }
        Ok(ForkResult::Child) => {
            tracee_init();
        }
        Err(_) => {
            eprintln!("failed with fork");
        }
    }
}
