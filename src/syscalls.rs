use crate::SyscallBody;
use nix::unistd::Pid;
use nix::sys::ptrace;
use libc::{c_void, c_long};

pub fn close_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
        println!("{}({}) = {}", syscall.name, syscall.first_arg, syscall.ret);
}

pub fn brk_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    if syscall.first_arg == 0 {
        println!("{}(NULL) = {:#x}", syscall.name, syscall.ret);
    }
    else {
        println!("{}({:#x}) = {:#x}", syscall.name, syscall.first_arg, syscall.ret);
    }
}

pub fn openat_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
     let openat_addr = syscall.second_arg as *mut c_void;
     let mut stack_data: c_long = 0;
     let mut hex_bytes: Vec<u8> = vec![];

     let mut string: String = String::new();
     match ptrace::read(*child_pid, openat_addr) {
         Ok(data) => {
             //let first_arg_string = String::from_utf8(data).expect("invalid UTF8");
             //println!("{:#x}", data);
             stack_data = data;
         },
         Err(_) => (),
     }
}
