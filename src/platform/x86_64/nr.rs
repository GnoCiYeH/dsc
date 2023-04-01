pub const SYS_PUTSTRING: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_READ: usize = 4;
pub const SYS_WRITE: usize = 5;
pub const SYS_LSEEK: usize = 6;
pub const SYS_FORK: usize = 7;
pub const SYS_VFORK: usize = 8;
pub const SYS_BRK: usize = 9;
pub const SYS_SBRK: usize = 10;

pub const SYS_REBOOT: usize = 11;
pub const SYS_CHDIR: usize = 12;
pub const SYS_GET_DENTS: usize = 13;
pub const SYS_EXECVE: usize = 14;
pub const SYS_WAIT4: usize = 15;
pub const SYS_EXIT: usize = 16;
pub const SYS_MKDIR: usize = 17;
pub const SYS_NANOSLEEP: usize = 18;
pub const SYS_CLOCK: usize = 19;
pub const SYS_PIPE: usize = 20;

pub const SYS_MSTAT: usize = 21;
pub const SYS_UNLINK_AT: usize = 22;
pub const SYS_KILL: usize = 23;
pub const SYS_SIGACTION: usize = 24;
pub const SYS_RT_SIGRETURN: usize = 25;
pub const SYS_GETPID: usize = 26;