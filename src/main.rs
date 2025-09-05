#![allow(dead_code, unused)]
#![allow(non_camel_case_types)]

mod syscalls;
mod prctl;

use nix::{
    unistd::{fork, ForkResult, Pid},
    sys::ptrace,
    sys::wait::{waitpid, WaitStatus},
    sys::signal::Signal::{SIGTRAP},
};
use libc::{SYS_openat, SYS_brk, SYS_close, SYS_access, SYS_execve, SYS_prctl, SYS_write, SYS_mmap, SYS_munmap, SYS_read, SYS_mprotect};
use libc::{ENOSYS, c_long, c_void};
use sysnames::Syscalls;
use exec;
use std::env;
use byteorder::{WriteBytesExt, ByteOrder, LittleEndian};
use crate::syscalls::SyscallBody;

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


fn match_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    match syscall.num as c_long {
        libc::SYS_openat => {syscalls::openat_syscall(child_pid, syscall);},
        libc::SYS_close => {syscalls::close_syscall(child_pid, syscall);},
        libc::SYS_brk => {syscalls::brk_syscall(child_pid, syscall);},
        libc::SYS_access => {syscalls::access_syscall(child_pid, syscall);},
        libc::SYS_write => {syscalls::write_syscall(child_pid, syscall);},
        libc::SYS_mmap => {syscalls::mmap_syscall(child_pid, syscall);},
        libc::SYS_munmap => {syscalls::munmap_syscall(child_pid, syscall);},
        libc::SYS_execve => {syscalls::execve_syscall(child_pid, syscall);},
        libc::SYS_read => {syscalls::read_syscall(child_pid, syscall);},
        libc::SYS_mprotect => {syscalls::mprotect_syscall(child_pid, syscall);},
        libc::SYS_prctl => {prctl::prctl_syscall(child_pid, syscall);},
        _ => {
            println!("{}({:#x})", syscall.name, syscall.rdi);
            ();
        },
    }

}


fn trace_syscall(child_pid: &Pid) {
    let regs = ptrace::getregs(*child_pid).unwrap();

    let mut syscall = SyscallBody {
        rax: regs.rax,
        rdi: regs.rdi,
        rsi: regs.rsi,
        rdx: regs.rdx,
        num: regs.orig_rax,
        name: Syscalls::name(regs.orig_rax).unwrap().to_uppercase(),
        args_count_flag: 0,
        first_arg: String::new(),
        second_arg: String::new(),
        third_arg: String::new(),
        ret: String::new(),
    };

    if syscall.rax == -ENOSYS as u64 {
        // it means that we are entering syscall so do nothing
        ();
    }
    else {
        match_syscall(&child_pid, &mut syscall);
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
                println!("===== finished successfully! =====");
                break;
            },
            Ok(WaitStatus::Stopped(pid_t, sig_t)) => {
                match sig_t {
                    SIGTRAP => trace_syscall(child_pid),
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
