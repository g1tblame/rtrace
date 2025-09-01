use nix::unistd::Pid;
use nix::sys::ptrace;
use libc::{c_void, c_long};
use byteorder::{WriteBytesExt, LittleEndian};

impl SyscallBody {
    fn print(&self) {
        match self.args_count_flag {
            1 => {
                println!("{}({}) = {}", self.name, self.first_arg, self.ret);
            },
            2 => {
                println!("{}({}, {}) = {}", self.name, self.first_arg, self.second_arg, self.rax);
            },
            3 => {
                println!("{}({}, {}, {}) = {}", self.name, self.first_arg, self.second_arg, self.third_arg, self.ret);
            },
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct SyscallBody {
    pub rax: u64,
    pub rdi: u64, // first arg
    pub rsi: u64, // second arg
    pub rdx: u64, // third arg
    pub name: String,
    pub num: u64,
    pub args_count_flag: u64,
    pub first_arg: String, // preprocessed rdi
    pub second_arg: String, // preprocessed rsi
    pub third_arg: String, // preprocessed rdx
    pub ret: String, // preprocessed rax
}

fn read_stack_data(child_pid: &Pid, stack_addr: ptrace::AddressType) -> String {
     let mut words_count = 0;
     let word_size = 8;
     let mut stack_string = String::new();

     'read: loop {
         let mut raw_bytes: Vec<u8> = vec![];
         let stack_addr = unsafe {stack_addr.offset(words_count)};


         let mut stack_data: c_long = 0;
         match ptrace::read(*child_pid, stack_addr) {
             Ok(res) => stack_data = res,
             Err(_) => break 'read,
         }

          raw_bytes.write_i64::<LittleEndian>(stack_data).unwrap_or_else(|err| {
              panic!("Failed to write {} as i64 LittleEndian: {}", stack_data, err);
          });

          for b in raw_bytes {
              if b != 0 {
                  stack_string.push(b as char);
              } else {
                  break 'read;
              }
          }
          words_count += word_size;
     }

     stack_string
}

pub fn close_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    syscall.args_count_flag = 1;
    syscall.first_arg = format!("{}", syscall.rdi);
    syscall.ret = format!("{}", syscall.rax);
    syscall.print();
}

pub fn brk_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    syscall.args_count_flag = 1;
    if syscall.rdi == 0 {
        syscall.first_arg.push_str("NULL");
    }
    else {
        syscall.first_arg = format!("0x{:x}", syscall.rdi);
    }
    syscall.ret = format!("0x{:x}", syscall.rax);
    syscall.print();
}

pub fn openat_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
     let openat_addr = syscall.rsi as *mut c_void;
     syscall.second_arg = read_stack_data(child_pid, openat_addr);
     syscall.first_arg.push_str("AT_FDCWD");
     syscall.ret = format!("0x{:x}", syscall.rax);
     syscall.args_count_flag = 3;

     match syscall.rdx {
         0 => syscall.third_arg.push_str("O_RDONLY"),
         1 => syscall.third_arg.push_str("O_WRONLY"),
         2 => syscall.third_arg.push_str("O_RDWR"),
         64 => syscall.third_arg.push_str("O_RDONLY|O_CREAT"),
         65 => syscall.third_arg.push_str("O_WRONLY|O_CREAT"),
         66 => syscall.third_arg.push_str("O_RDWR|O_CREAT"),
         1024 => syscall.third_arg.push_str("O_RDONLY|O_APPEND"),
         1025 => syscall.third_arg.push_str("O_WRONLY|O_APPEND"),
         1026 => syscall.third_arg.push_str("O_RDWR|O_APPEND"),
         524288 => syscall.third_arg.push_str("O_RDONLY|O_CLOEXEC"),
         524289 => syscall.third_arg.push_str("O_WRONLY|O_CLOEXEC"),
         524290 => syscall.third_arg.push_str("O_RDWR|O_CLOEXEC"),
         _ => syscall.third_arg.push_str("unknown option yet"),
     }

    syscall.print();
     
}

pub fn access_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    let addr: ptrace::AddressType = syscall.rdi as *mut c_void;
    syscall.first_arg = read_stack_data(child_pid, addr);
    if(syscall.rsi == 4) {
        syscall.second_arg.push_str("R_OK");
    }
    syscall.args_count_flag = 2;
    syscall.print();
}

pub fn write_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
     syscall.args_count_flag = 3;
     let write_addr = syscall.rsi as *mut c_void;
     syscall.second_arg = read_stack_data(child_pid, write_addr);
     syscall.first_arg = format!("{}", syscall.rdi);
     syscall.third_arg = format!("{}", syscall.rdx);
     syscall.ret = format!("{}", syscall.rax);
     syscall.print();
}

pub fn mmap_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    syscall.args_count_flag = 3;
    if syscall.rdi == 0 {
        syscall.first_arg.push_str("NULL");
    }
    else {
        syscall.first_arg = format!("0x{:x}", syscall.rdi);
    }
    syscall.second_arg = syscall.rsi.to_string();

    match syscall.rdx {
        1 => syscall.third_arg.push_str("PROT_READ"),
        2 => syscall.third_arg.push_str("PROT_WRITE"),
        3 => syscall.third_arg.push_str("PROT_READ|PROT_WRITE"),
        4 => syscall.third_arg.push_str("PROT_EXEC"),
        5 => syscall.third_arg.push_str("PROT_READ|PROT_EXEC"),
        8 => syscall.third_arg.push_str("PROT_SEM"),
        _ => (),
    }

    syscall.ret = format!("0x{:x}", syscall.rax);
    syscall.print();
}

pub fn munmap_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    syscall.args_count_flag = 2;
    syscall.first_arg = format!("0x{:x}", syscall.rdi);
    syscall.second_arg = syscall.rsi.to_string();

    syscall.print();
}

pub fn read_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
//    let addr = syscall.rsi as *mut c_void;
//    syscall.second_arg = read_stack_data(child_pid, addr);
    syscall.args_count_flag = 3;
    syscall.first_arg = format!("0x{:x}", syscall.rdi);
    syscall.second_arg = format!("0x{:x}", syscall.rsi);
    syscall.third_arg = syscall.rdx.to_string();
    syscall.ret = format!("{}", syscall.rax);

    syscall.print();
}

pub fn prctl_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    syscall.args_count_flag = 1;
    match syscall.rdi {
        1 => syscall.first_arg.push_str("PR_SET_PDEATHSIG"),
        2 => syscall.first_arg.push_str("PR_GET_PDEATHSIG"),
        3 => syscall.first_arg.push_str("PR_GET_DUMPABLE"),
        4 => syscall.first_arg.push_str("PR_SET_DUMPABLE"),
        5 => syscall.first_arg.push_str("PR_GET_UNALIGN"),
        6 => syscall.first_arg.push_str("PR_SET_UNALIGN"),
        23 => syscall.first_arg.push_str("PR_CAPBSET_READ"),
        _ => (),
    }

    if syscall.rax == 18446744073709551594 {
        syscall.ret.push_str("-1 EINVAL (Invalid argument)");
    }
    else {
        syscall.ret = format!("{}", syscall.rax);
    }

    syscall.print();
}

pub fn execve_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
//   for some reason execve don't work yet
//    dbg!(syscall);
//    for some reason execve don't work yet
}


