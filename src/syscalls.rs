use nix::unistd::Pid;
use nix::sys::ptrace;
use libc::{c_void, c_long};
use byteorder::{WriteBytesExt, LittleEndian};

impl SyscallBody {
    fn print(&self) {
        match self.args_count_flag {
            0 => {
                println!("{}(NULL) = {:#x}", self.name, self.ret);
            },
            1 => {
                println!("{}({:#x}) = {:#x}", self.name, self.first_arg, self.ret);
            },
            2 => {
                println!("{}({}, {}) = {:#x}", self.name, self.first_arg, self.second_arg_string, self.ret);
            },
            _ => (),
        }
    }
}

pub struct SyscallBody {
    pub ret: u64,
    pub first_arg: u64,
    pub second_arg: u64,
    pub third_arg: u64,
    pub name: String,
    pub num: u64,
    pub args_count_flag: u64,
    pub first_arg_string: String,
    pub second_arg_string: String,
}

pub fn close_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    syscall.args_count_flag = 1;
    syscall.print();
}

pub fn brk_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    if syscall.first_arg == 0 {
        syscall.args_count_flag = 0;
    } else {
        syscall.args_count_flag = 1;
    }
    syscall.print();
}

pub fn openat_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
     let openat_addr = syscall.second_arg as *mut c_void;
     let mut words_count = 0;
     let word_size = 8;
     let mut stack_string = String::new();

     'read: loop {
         let mut raw_bytes: Vec<u8> = vec![];
         let openat_addr = unsafe {openat_addr.offset(words_count)};


         let mut stack_data: c_long = 0;
         match ptrace::read(*child_pid, openat_addr) {
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
     syscall.second_arg_string = stack_string;
     syscall.args_count_flag = 2;
    // syscall.print();
     if(syscall.first_arg == 4294967196) {
         println!("{}(AT_FDCWD, {}, {:#x}) = {:#x}", syscall.name, syscall.second_arg_string, syscall.third_arg, syscall.ret);
     }
     
}

pub fn access_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
    let addr: ptrace::AddressType = syscall.first_arg as *mut c_void;
    syscall.first_arg_string = read_stack_data(child_pid, addr);
    println!("{}({}) = {:#x}", syscall.name, syscall.first_arg_string, syscall.ret);
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
