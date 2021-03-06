//! unistd implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/unistd.h.html

#![no_std]
#![cfg_attr(target_os = "redox", feature(alloc))]

#[cfg(target_os = "redox")] extern crate alloc;
extern crate platform;
extern crate stdio;
extern crate string;
extern crate sys_utsname;

pub use platform::types::*;
pub use getopt::*;

use core::ptr;

mod getopt;

pub const R_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const X_OK: c_int = 4;
pub const F_OK: c_int = 8;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const F_ULOCK: c_int = 0;
pub const F_LOCK: c_int = 1;
pub const F_TLOCK: c_int = 2;
pub const F_TEST: c_int = 3;

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

#[no_mangle]
pub static mut environ: *const *mut c_char = ptr::null();

#[no_mangle]
pub extern "C" fn _exit(status: c_int) {
    platform::exit(status)
}

#[no_mangle]
pub extern "C" fn access(path: *const c_char, amode: c_int) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn alarm(seconds: c_uint) -> c_uint {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn brk(addr: *mut c_void) -> c_int {
    platform::brk(addr)
}

#[no_mangle]
pub extern "C" fn chdir(path: *const c_char) -> c_int {
    platform::chdir(path)
}

#[no_mangle]
pub extern "C" fn chroot(path: *const c_char) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int {
    platform::chown(path, owner, group)
}

#[no_mangle]
pub extern "C" fn close(fildes: c_int) -> c_int {
    platform::close(fildes)
}

#[no_mangle]
pub extern "C" fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn crypt(key: *const c_char, salt: *const c_char) -> *mut c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn dup(fildes: c_int) -> c_int {
    platform::dup(fildes)
}

#[no_mangle]
pub extern "C" fn dup2(fildes: c_int, fildes2: c_int) -> c_int {
    platform::dup2(fildes, fildes2)
}

#[no_mangle]
pub extern "C" fn encrypt(block: [c_char; 64], edflag: c_int) {
    unimplemented!();
}

// #[no_mangle]
// pub extern "C" fn execl(path: *const c_char, args: *const *mut c_char) -> c_int {
//     unimplemented!();
// }

// #[no_mangle]
// pub extern "C" fn execle(
//   path: *const c_char,
//   args: *const *mut c_char,
//   envp: *const *mut c_char,
// ) -> c_int {
//     unimplemented!();
// }

// #[no_mangle]
// pub extern "C" fn execlp(file: *const c_char, args: *const *mut c_char) -> c_int {
//     unimplemented!();
// }

#[no_mangle]
pub unsafe extern "C" fn execv(path: *const c_char, argv: *const *mut c_char) -> c_int {
    execve(path, argv, environ)
}

#[no_mangle]
pub unsafe extern "C" fn execve(
    path: *const c_char,
    argv: *const *mut c_char,
    envp: *const *mut c_char,
) -> c_int {
    #[cfg(target_os = "linux")] {
        platform::execve(path, argv, envp)
    }
    #[cfg(target_os = "redox")] {
        use alloc::Vec;
        use platform::{c_str, e};
        use platform::syscall::flag::*;

        let mut env = envp;
        while !(*env).is_null() {
            let slice = c_str(*env);
            // Should always contain a =, but worth checking
            if let Some(sep) = slice.iter().position(|&c| c == b'=') {
                // If the environment variable has no name, do not attempt
                // to add it to the env.
                if sep > 0 {
                    let mut path = b"env:".to_vec();
                    path.extend_from_slice(&slice[..sep]);
                    match platform::syscall::open(&path, O_WRONLY | O_CREAT) {
                        Ok(fd) => {
                            // If the environment variable has no value, there
                            // is no need to write anything to the env scheme.
                            if sep + 1 < slice.len() {
                                let n = match platform::syscall::write(fd, &slice[sep + 1..]) {
                                    Ok(n) => n,
                                    err => {
                                        return e(err) as c_int;
                                    }
                                };
                            }
                            // Cleanup after adding the variable.
                            match platform::syscall::close(fd) {
                                Ok(_) => (),
                                err => {
                                    return e(err) as c_int;
                                }
                            }
                        }
                        err => {
                            return e(err) as c_int;
                        }
                    }
                }
            }
            env = env.offset(1);
        }

        let mut len = 0;
        for i in 0.. {
            if (*argv.offset(i)).is_null() {
                len = i;
                break;
            }
        }

        let mut args: Vec<[usize; 2]> = Vec::with_capacity(len as usize);
        let mut arg = argv;
        while !(*arg).is_null() {
            args.push([*arg as usize, c_str(*arg).len()]);
            arg = arg.offset(1);
        }

        e(platform::syscall::execve(c_str(path), &args)) as c_int
    }
}

#[no_mangle]
pub extern "C" fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fchown(fildes: c_int, owner: uid_t, group: gid_t) -> c_int {
    platform::fchown(fildes, owner, group)
}

#[no_mangle]
pub extern "C" fn fchdir(fildes: c_int) -> c_int {
    platform::fchdir(fildes)
}

#[no_mangle]
pub extern "C" fn fdatasync(fildes: c_int) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fork() -> pid_t {
    platform::fork()
}

#[no_mangle]
pub extern "C" fn fpathconf(fildes: c_int, name: c_int) -> c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn fsync(fildes: c_int) -> c_int {
    platform::fsync(fildes)
}

#[no_mangle]
pub extern "C" fn ftruncate(fildes: c_int, length: off_t) -> c_int {
    platform::ftruncate(fildes, length)
}

#[no_mangle]
pub extern "C" fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
    platform::getcwd(buf, size)
}

#[no_mangle]
pub extern "C" fn getdtablesize() -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getegid() -> gid_t {
    platform::getegid()
}

#[no_mangle]
pub extern "C" fn geteuid() -> uid_t {
    platform::geteuid()
}

#[no_mangle]
pub extern "C" fn getgid() -> gid_t {
    platform::getgid()
}

#[no_mangle]
pub extern "C" fn getgroups(gidsetsize: c_int, grouplist: *mut gid_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn gethostid() -> c_long {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gethostname(mut name: *mut c_char, len: size_t) -> c_int {
    #[cfg(target_os = "linux")] {
        use core::mem;

        // len only needs to be mutable on linux
        let mut len = len;

        let mut uts: sys_utsname::utsname = mem::uninitialized();
        let err = sys_utsname::uname(&mut uts);
        if err < 0 {
            mem::forget(uts);
            return err;
        }
        for c in uts.nodename.iter() {
            if len == 0 { break; }
            len -= 1;

            *name = *c;

            if *name == 0 {
                // We do want to copy the zero also, so we check this after the copying.
                break;
            }

            name = name.offset(1);
        }
    }
    #[cfg(target_os = "redox")] {
        use platform::{e, FileReader, Read};
        use platform::syscall::flag::*;

        let fd = e(platform::syscall::open("/etc/hostname", O_RDONLY)) as i32;
        if fd < 0 {
            return fd;
        }
        let mut reader = FileReader(fd);
        for _ in 0..len {
            if !reader.read_u8(&mut *(name as *mut u8)) {
                *name = 0;
                break;
            }
            name = name.offset(1);
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn getlogin() -> *mut c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getlogin_r(name: *mut c_char, namesize: size_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getpagesize() -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getpass(prompt: *const c_char) -> *mut c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getpgid(pid: pid_t) -> pid_t {
    platform::getpgid(pid)
}

#[no_mangle]
pub extern "C" fn getpgrp() -> pid_t {
    platform::getpgid(platform::getpid())
}

#[no_mangle]
pub extern "C" fn getpid() -> pid_t {
    platform::getpid()
}

#[no_mangle]
pub extern "C" fn getppid() -> pid_t {
    platform::getppid()
}

#[no_mangle]
pub extern "C" fn getsid(pid: pid_t) -> pid_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn getuid() -> uid_t {
    platform::getuid()
}

#[no_mangle]
pub extern "C" fn getwd(path_name: *mut c_char) -> *mut c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn isatty(fildes: c_int) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn lchown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn link(path1: *const c_char, path2: *const c_char) -> c_int {
    platform::link(path1, path2)
}

#[no_mangle]
pub extern "C" fn lockf(fildes: c_int, function: c_int, size: off_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn lseek(fildes: c_int, offset: off_t, whence: c_int) -> off_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn nice(incr: c_int) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn pathconf(path: *const c_char, name: c_int) -> c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn pause() -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn pipe(fildes: [c_int; 2]) -> c_int {
    platform::pipe(fildes)
}

#[no_mangle]
pub extern "C" fn pread(fildes: c_int, buf: *mut c_void, nbyte: size_t, offset: off_t) -> ssize_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn pthread_atfork(
    prepare: Option<extern "C" fn()>,
    parent: Option<extern "C" fn()>,
    child: Option<extern "C" fn()>,
) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn pwrite(
    fildes: c_int,
    buf: *const c_void,
    nbyte: size_t,
    offset: off_t,
) -> ssize_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn read(fildes: c_int, buf: *const c_void, nbyte: size_t) -> ssize_t {
    use core::slice;
    let buf = unsafe { slice::from_raw_parts_mut(buf as *mut u8, nbyte as usize) };
    platform::read(fildes, buf)
}

#[no_mangle]
pub extern "C" fn readlink(path: *const c_char, buf: *mut c_char, bufsize: size_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn rmdir(path: *const c_char) -> c_int {
    platform::rmdir(path)
}

#[no_mangle]
pub extern "C" fn sbrk(incr: intptr_t) -> *mut c_void {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn setgid(gid: gid_t) -> c_int {
    platform::setregid(gid, gid)
}

#[no_mangle]
pub extern "C" fn setpgid(pid: pid_t, pgid: pid_t) -> c_int {
    platform::setpgid(pid, pgid)
}

#[no_mangle]
pub extern "C" fn setpgrp() -> pid_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn setregid(rgid: gid_t, egid: gid_t) -> c_int {
    platform::setregid(rgid, egid)
}

#[no_mangle]
pub extern "C" fn setreuid(ruid: uid_t, euid: uid_t) -> c_int {
    platform::setreuid(ruid, euid)
}

#[no_mangle]
pub extern "C" fn setsid() -> pid_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn setuid(uid: uid_t) -> c_int {
    platform::setreuid(uid, uid)
}

#[no_mangle]
pub extern "C" fn sleep(seconds: c_uint) -> c_uint {
    let rqtp = timespec {
        tv_sec: seconds as i64,
        tv_nsec: 0,
    };
    let rmtp = ptr::null_mut();
    platform::nanosleep(&rqtp, rmtp);
    0
}

#[no_mangle]
pub extern "C" fn swab(src: *const c_void, dest: *mut c_void, nbytes: ssize_t) {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn symlink(path1: *const c_char, path2: *const c_char) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn sync() {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn sysconf(name: c_int) -> c_long {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn tcgetpgrp() -> pid_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn tcsetpgrp(fildes: c_int, pgid_id: pid_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn truncate(path: *const c_char, length: off_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ttyname(fildes: c_int) -> *mut c_char {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ttyname_r(fildes: c_int, name: *mut c_char, namesize: size_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn ualarm(useconds: useconds_t, interval: useconds_t) -> useconds_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn unlink(path: *const c_char) -> c_int {
    platform::unlink(path)
}

#[no_mangle]
pub extern "C" fn usleep(useconds: useconds_t) -> c_int {
    let rqtp = timespec {
        tv_sec: 0,
        tv_nsec: (useconds * 1000).into(),
    };
    let rmtp = ptr::null_mut();
    platform::nanosleep(&rqtp, rmtp)
}

#[no_mangle]
pub extern "C" fn vfork() -> pid_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn write(fildes: c_int, buf: *const c_void, nbyte: size_t) -> ssize_t {
    use core::slice;

    let buf = unsafe { slice::from_raw_parts(buf as *const u8, nbyte as usize) };
    platform::write(fildes, buf)
}

/*
#[no_mangle]
pub extern "C" fn func(args) -> c_int {
    unimplemented!();
}
*/
