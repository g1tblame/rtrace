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
use std::env;

fn fork_init() {
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

//fn check_args_len() -> bool {
//    let cli_args: Vec<String> = env::args().collect();
//    match cli_args.len() {
//        1 => false,
//        2 => true,
//        _ => false,
//    }
//
//}

fn handle_syscall(child_pid: &Pid) {
    let regs = ptrace::getregs(*child_pid).unwrap();
//    println!("{:?}", regs);
    if regs.rax == -ENOSYS as u64 {
        // it means that we are entering syscall so do nothing
        ();
    }
    else {
        let syscall_name = Syscalls::name(regs.orig_rax).unwrap().to_uppercase();
        let ret = regs.rax;
        println!("{}({:#x}, {:#x}, {:#x}) = {}", syscall_name, regs.rdi, regs.rsi, regs.rdx, ret);
    }
}

fn tracee_init() {
    ptrace::traceme().expect("failed to set TRACEME flag");
    let bin: String = String::from("ls");
    let _ = exec::Command::new(bin)
//        .arg("-la")
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
        fork_init();
}
