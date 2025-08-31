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
                println!("{}({:#x}) = {:#x}", self.name, self.rdi, self.ret);
            },
            2 => {
                println!("{}({}, {}) = {:#x}", self.name, self.first_arg, self.second_arg, self.ret);
            },
            3 => {
                println!("{}({}, {}, {}) = {:#x}", self.name, self.first_arg, self.second_arg, self.third_arg, self.ret);
            },
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct SyscallBody {
    pub ret: u64,
    pub rdi: u64, // first arg
    pub rsi: u64, // second arg
    pub rdx: u64, // third arg
    pub name: String,
    pub num: u64,
    pub args_count_flag: u64,
    pub first_arg: String,
    pub second_arg: String,
    pub third_arg: String,
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

//pub fn close_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
//    syscall.args_count_flag = 1;
//    syscall.print();
//}
//
//pub fn brk_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
//    if syscall.first_arg == 0 {
//        syscall.args_count_flag = 0;
//    } else {
//        syscall.args_count_flag = 1;
//    }
//    syscall.print();
//}

pub fn openat_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
     let openat_addr = syscall.rsi as *mut c_void;
     syscall.second_arg = read_stack_data(child_pid, openat_addr);
     syscall.first_arg.push_str("AT_FDCWD");
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

     //println!("{}({}, {}, {} rdx({})) = {}", syscall.name, syscall.first_arg, syscall.second_arg, syscall.third_arg, syscall.rdx, syscall.ret);
     syscall.print();
     
}

//pub fn access_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
//    let addr: ptrace::AddressType = syscall.first_arg as *mut c_void;
//    syscall.first_arg_string = read_stack_data(child_pid, addr);
//    println!("{}({}) = {:#x}", syscall.name, syscall.first_arg_string, syscall.ret);
//}
//
//pub fn write_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
//     let write_addr = syscall.second_arg as *mut c_void;
//     syscall.second_arg_string = read_stack_data(child_pid, write_addr);
//     println!("{}({}, [{}]) = {}", syscall.name, syscall.first_arg, syscall.second_arg_string, syscall.ret);
//}

pub fn execve_syscall(child_pid: &Pid, syscall: &mut SyscallBody) {
//   for some reason execve don't work yet
//    dbg!(syscall);
//    for some reason execve don't work yet
}


