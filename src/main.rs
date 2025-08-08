#![allow(dead_code, unused)]

use nix::unistd::{fork, ForkResult, Pid};
use nix::sys::ptrace;
use exec;

fn tracee_init() {
    let _ = ptrace::traceme();
    let _ = exec::Command::new("ls")
        .arg("-la")
        .exec();
}

fn main() {
    match unsafe{fork()} {
        Ok(ForkResult::Child) => {
            println!("i'm child!");
            tracee_init();
        }
        Ok(ForkResult::Parent{child}) => {
            println!("i'm a parent!");
        }
        Err(_) => {
            println!("failed with fork");
        }
    }
}
