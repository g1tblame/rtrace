#![allow(dead_code, unused)]

use nix::unistd::{fork, ForkResult, Pid};
use nix::sys::{ptrace, wait};
use nix::sys::ptrace::Options;
use libc::{WSTOPSIG, WIFSTOPPED, WIFEXITED};
use exec;

fn tracee_init() {
    let _ = ptrace::traceme();
    let _ = exec::Command::new("ls")
        .arg("-la")
        .exec();
}

fn tracer_init(child_pid: &Pid) {
    let status: i32;

    ptrace::setoptions(*child_pid, Options::PTRACE_O_TRACESYSGOOD);
    
    loop {
        ptrace::syscall(*child_pid, None);
        match wait::waitpid(*child_pid, None) {
            Ok(status) => {
                //need to castu status into the c_int
                println!("process has benn changed");
                let tmp_stat = status;
                println!("{:?}", tmp_stat);
            }
            Err(_) => {
                println!("some error");
            }
        }
    }
}

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Child) => {
            println!("i'm child!");
            tracee_init();
        }
        Ok(ForkResult::Parent{child}) => {
            println!("i'm a parent!");
            tracer_init(&child);
        }
        Err(_) => {
            println!("failed with fork");
        }
    }
}
