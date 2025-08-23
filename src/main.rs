#![allow(dead_code, unused)]

use nix::{
    unistd::{fork, ForkResult, Pid},
    sys::ptrace,
    sys::wait::{waitpid, WaitStatus},
    sys::signal::Signal::{SIGTRAP},
};
use libc::{SYS_openat, SYS_brk};
use libc::{ENOSYS, c_long, c_void};
use sysnames::Syscalls;
use exec;
use std::env;
use byteorder::{WriteBytesExt, ByteOrder, LittleEndian};

struct SyscallBody {
    ret: u64,
    first_arg: u64,
    second_arg: u64,
    third_arg: u64,
    name: String,
    num: u64,
}

impl SyscallBody {
    fn print(&self) {
        println!("{}({:#x}, {:#x}, {:#x}) = {}", self.name, self.first_arg, self.second_arg, self.third_arg, self.ret);
    }
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

fn openat_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
        let openat_addr = syscall.second_arg as *mut c_void;
        let mut stack_data: c_long = 0;
        let mut hex_bytes: Vec<u8> = vec![];

        let mut string: String = String::new();
        match ptrace::read(*child_pid, openat_addr) {
            Ok(data) => {
                //let first_arg_string = String::from_utf8(data).expect("invalid UTF8");
                println!("{:#x}", data);
                stack_data = data;
            },
            Err(_) => (),
        }

        hex_bytes.write_i64::<LittleEndian>(stack_data).unwrap_or_else(|err| {
            panic!("Failed to write {} as i64 LittleEndian: {}", stack_data, err);
        });
        println!("STRING HERE - {:?}", hex_bytes);

        syscall.print();
}


fn match_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    match syscall.num as c_long {
        libc::SYS_openat => {openat_syscall(child_pid, syscall);},
        _ => (),
    }
    //syscall.print();

}


fn trace_syscall(child_pid: &Pid) {
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
                println!("child process was finished!");
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
