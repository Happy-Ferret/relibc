use core::sync::atomic::AtomicBool;
use core::ptr;
use super::{constants, internal, BUFSIZ, FILE, UNGET};

#[allow(non_upper_case_globals)]
static mut default_stdin_buf: [u8; BUFSIZ as usize + UNGET] = [0; BUFSIZ as usize + UNGET];

#[allow(non_upper_case_globals)]
static mut default_stdin: FILE = FILE {
    flags: constants::F_PERM | constants::F_NOWR | constants::F_BADJ,
    rpos: ptr::null_mut(),
    rend: ptr::null_mut(),
    wend: ptr::null_mut(),
    wpos: ptr::null_mut(),
    wbase: ptr::null_mut(),
    fd: 0,
    buf: unsafe { &mut default_stdin_buf as *mut [u8] as *mut u8 },
    buf_size: BUFSIZ as usize,
    buf_char: -1,
    unget: UNGET,
    lock: AtomicBool::new(false),
};

#[allow(non_upper_case_globals)]
static mut default_stdout_buf: [u8; BUFSIZ as usize] = [0; BUFSIZ as usize];

#[allow(non_upper_case_globals)]
static mut default_stdout: FILE = FILE {
    flags: constants::F_PERM | constants::F_NORD | constants::F_BADJ,
    rpos: ptr::null_mut(),
    rend: ptr::null_mut(),
    wend: ptr::null_mut(),
    wpos: ptr::null_mut(),
    wbase: ptr::null_mut(),
    fd: 1,
    buf: unsafe { &mut default_stdout_buf } as *mut [u8] as *mut u8,
    buf_size: BUFSIZ as usize,
    buf_char: b'\n' as i8,
    unget: 0,
    lock: AtomicBool::new(false),
};

#[allow(non_upper_case_globals)]
static mut default_stderr_buf: [u8; BUFSIZ as usize] = [0; BUFSIZ as usize];

#[allow(non_upper_case_globals)]
static mut default_stderr: FILE = FILE {
    flags: constants::F_PERM | constants::F_NORD | constants::F_BADJ,
    rpos: ptr::null_mut(),
    rend: ptr::null_mut(),
    wend: ptr::null_mut(),
    wpos: ptr::null_mut(),
    wbase: ptr::null_mut(),
    fd: 2,
    buf: unsafe { &mut default_stderr_buf } as *mut [u8] as *mut u8,
    buf_size: BUFSIZ as usize,
    buf_char: -1,
    unget: 0,
    lock: AtomicBool::new(false),
};

#[no_mangle]
pub extern "C" fn __stdin() -> *mut FILE {
    unsafe { &mut default_stdin }
}

#[no_mangle]
pub extern "C" fn __stdout() -> *mut FILE {
    unsafe { &mut default_stdout }
}

#[no_mangle]
pub extern "C" fn __stderr() -> *mut FILE {
    unsafe { &mut default_stderr }
}
