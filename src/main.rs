#![allow(dead_code, unused)]

use nix::{
    unistd::{fork, ForkResult, Pid},
    sys::ptrace,
    sys::wait::{waitpid, WaitStatus},
    sys::signal::Signal::{SIGTRAP},
};
use libc::{ENOSYS, SYS_openat, c_long, c_void};
use sysnames::Syscalls;
use exec;
use std::env;

struct SyscallBody {
    ret: u64,
    first_arg: u64,
    second_arg: u64,
    third_arg: u64,
    name: String,
    num: u64,
}

fn fork_init() {
    match unsafe{fork()} {
        Ok(ForkResult::Parent{child}) => {
            tracer_init(&child);
        }
        Ok(ForkResult::Child) => {
            tracee_init();
        }
        Err(_) => {
            eprintln!("fork failed");
        }
    }
}

fn check_args_len(exec_args: usize) -> bool {
    match exec_args {
        1 => false,
        2 => true,
        _ => false,
    }
}

fn print_syscall(syscall: &SyscallBody) {
    println!("{}({:#x}, {:#x}, {:#x}) = {}", syscall.name, syscall.first_arg, syscall.second_arg, syscall.third_arg, syscall.ret);
}

fn preprocess_syscall_args(child_pid: &Pid, syscall: &mut SyscallBody) {
    match syscall.num as c_long {
        SYS_openat => {
            let openat_addr = syscall.second_arg as *mut c_void;
            match ptrace::read(*child_pid, openat_addr) {
                Ok(data) => {println!("OPENAT DATA HERE - {:#x}", data);},
                Err(_) => (),
            }
        },
        _ => (),
    }
    print_syscall(&syscall);
}


fn handle_syscall(child_pid: &Pid) {
    let regs = ptrace::getregs(*child_pid).unwrap();

    let mut syscall = SyscallBody {
        ret: regs.rax,
        first_arg: regs.rdi,
        second_arg: regs.rsi,
        third_arg: regs.rdx,
        num: regs.orig_rax,
        name: Syscalls::name(regs.orig_rax).unwrap().to_uppercase(),
    };

    if syscall.ret == -ENOSYS as u64 {
        // it means that we are entering syscall so do nothing
        ();
    }
    else {
        preprocess_syscall_args(&child_pid, &mut syscall);
    }
}

fn tracee_init() {
    ptrace::traceme().expect("failed to set TRACEME flag");
    let cli_args: Vec<String> = env::args().collect();
    let _ = exec::Command::new(cli_args[1].clone()).exec();
}

fn tracer_init(child_pid: &Pid) {
    ptrace::setoptions(*child_pid, ptrace::Options::PTRACE_O_TRACESYSGOOD);

    loop {
        ptrace::syscall(*child_pid, None);
        match waitpid(*child_pid, None) {
            Ok(WaitStatus::Exited(_, _)) => {
                println!("child process was finished!");
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
    let cli_args: Vec<String> = env::args().collect();
    match check_args_len(cli_args.len()) {
        false => {eprintln!("to few arguments!");},
        true => {fork_init();},
    }
}
