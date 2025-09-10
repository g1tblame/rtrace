# rtracer

tiny syscalls' tracer wich was created in rust just for fun.

achtung: project contains a lot of shitty code, don't use it for the real purposes!!

usage example for ls:

'''
~/rtrace [ rtrace ls                                                                                                                                                                               main * ] 9:29 PM
BRK(NULL) = 0x55bfbaa92000
ACCESS(/etc/ld.so.preload, R_OK) = 0xfffffffffffffffe
OPENAT(AT_FDCWD, /etc/ld.so.cache, O_RDONLY|O_CLOEXEC) = 0x3
FSTAT(0x3)
MMAP(NULL, 48855, PROT_READ) = 0x7f3f917be000
CLOSE(3) = 0
OPENAT(AT_FDCWD, /usr/lib/libcap.so.2, O_RDONLY|O_CLOEXEC) = 0x3
READ(0x3, 0x7ffc011754a8, 832) = 832
FSTAT(0x3)
MMAP(NULL, 8192, PROT_READ|PROT_WRITE) = 0x7f3f917bc000
MMAP(NULL, 45128, PROT_READ) = 0x7f3f917b0000
MMAP(0x7f3f917b3000, 20480, PROT_READ|PROT_EXEC) = 0x7f3f917b3000
MMAP(0x7f3f917b8000, 8192, PROT_READ) = 0x7f3f917b8000
MMAP(0x7f3f917ba000, 8192, PROT_READ|PROT_WRITE) = 0x7f3f917ba000
CLOSE(3) = 0
OPENAT(AT_FDCWD, /usr/lib/libc.so.6, O_RDONLY|O_CLOEXEC) = 0x3
READ(0x3, 0x7ffc01175488, 832) = 832
PREAD64(0x3)
FSTAT(0x3)
PREAD64(0x3)
MMAP(NULL, 2174000, PROT_READ) = 0x7f3f91400000
MMAP(0x7f3f91424000, 1515520, PROT_READ|PROT_EXEC) = 0x7f3f91424000
MMAP(0x7f3f91596000, 454656, PROT_READ) = 0x7f3f91596000
MMAP(0x7f3f91605000, 24576, PROT_READ|PROT_WRITE) = 0x7f3f91605000
MMAP(0x7f3f9160b000, 31792, PROT_READ|PROT_WRITE) = 0x7f3f9160b000
CLOSE(3) = 0
MMAP(NULL, 12288, PROT_READ|PROT_WRITE) = 0x7f3f917ad000
ARCH_PRCTL(0x1002)
SET_TID_ADDRESS(0x7f3f917ada10)
SET_ROBUST_LIST(0x7f3f917ada20)
RSEQ(0x7f3f917ad680)
MPROTECT(0x7f3f91605000, 0x4000) = 0
MPROTECT(0x7f3f917ba000, 0x1000) = 0
MPROTECT(0x55bf963e8000, 0x2000) = 0
MPROTECT(0x7f3f9180b000, 0x2000) = 0
PRLIMIT64(0x0)
GETRANDOM(0x7f3f91610200)
MUNMAP(0x7f3f917be000, 48855) = 0x0
PRCTL(PR_CAPBSET_READ, 0x20) = 0x1
PRCTL(PR_CAPBSET_READ, 0x30) = 0xffffffffffffffea
PRCTL(PR_CAPBSET_READ, 0x28) = 0x1
PRCTL(PR_CAPBSET_READ, 0x2c) = 0xffffffffffffffea
PRCTL(PR_CAPBSET_READ, 0x2a) = 0xffffffffffffffea
PRCTL(PR_CAPBSET_READ, 0x29) = 0xffffffffffffffea
BRK(NULL) = 0x55bfbaa92000
BRK(0x55bfbaab3000) = 0x55bfbaab3000
OPENAT(AT_FDCWD, /usr/lib/locale/locale-archive, O_RDONLY|O_CLOEXEC) = 0x3
FSTAT(0x3)
MMAP(NULL, 3063024, PROT_READ) = 0x7f3f91000000
CLOSE(3) = 0
IOCTL(0x1)
IOCTL(0x1)
OPENAT(AT_FDCWD, ., unknown option yet) = 0x3
FSTAT(0x3)
GETDENTS64(0x3)
GETDENTS64(0x3)
CLOSE(3) = 0
FSTAT(0x1)
Cargo.lock  Cargo.toml	demo.cast  Makefile  README.md	src  target  tests
WRITE(1, Cargo.lock  Cargo.toml	demo.cast  Makefile  README.md	src  target  tests
, 73) = 73
CLOSE(1) = 0
CLOSE(2) = 0
===== finished successfully! =====

'''
