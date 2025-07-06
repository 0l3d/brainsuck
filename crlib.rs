#![no_std]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![allow(dead_code)]

pub type CChar = i8;
pub type PCChar = *const i8;
pub type PMutChar = *mut i8;
pub type Void = core::ffi::c_void;

#[derive(Clone, Copy)]
pub struct ExecResult {
    pub output: PCChar,
    pub status: i32,
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    // PRINT
    pub fn printf(fmt: PCChar, ...) -> i32;
    pub fn fprintf(stream: *mut Void, fmt: PCChar, ...) -> i32;
    pub fn sprintf(s: *mut u8, fmt: PCChar, ...) -> i32;
    pub fn snprintf(s: *mut u8, n: usize, fmt: PCChar, ...) -> i32;
    // SCAN
    pub fn scanf(fmt: PCChar, ...) -> i32;
    pub fn fscanf(stream: *mut Void, fmt: PCChar, ...) -> i32;
    pub fn sscanf(s: *const u8, fmt: PCChar, ...) -> i32;
    // FILE IO
    pub fn fopen(filename: PCChar, mode: PCChar) -> *mut FILE;
    pub fn fgets(buf: PMutChar, n: i32, stream: *mut FILE) -> PMutChar;
    pub fn fclose(stream: *mut FILE) -> i32;
    pub fn fputs(s: PCChar, stream: *mut FILE) -> i32;
    
    // STD IO
    pub fn write(fd: usize, buf: *const u8, count: usize) -> isize;
    pub fn read(fd: usize, buf: *mut u8, count: usize) -> isize;
    pub fn exit(code: i32) -> !;
    pub fn system(command: PCChar) -> i32;
    // FILE SYSTEM
    pub fn chdir(path: PCChar) -> i32;
    pub fn getcwd(buf: *mut i8, size: usize) -> PCChar;
    // MEMORY
    pub fn malloc(size: usize) -> *mut Void;
    pub fn realloc(ptr: *mut Void, size: usize) -> *mut Void;
    pub fn free(ptr: *mut Void);
    pub fn memset(ptr: *mut u8, value: i32, size: usize) -> *mut u8;
    pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
    pub fn memchr(s: *const u8, c: i32, n: usize) -> *mut u8;
    pub fn calloc(nmemb: usize, size: usize) -> *mut Void;
    // MATH
    pub fn sin(x: f64) -> f64;
    pub fn cos(x: f64) -> f64;
    pub fn tan(x: f64) -> f64;
    pub fn atan(x: f64) -> f64;
    pub fn atan2(y: f64, x: f64) -> f64;
    pub fn sqrt(x: f64) -> f64;
    pub fn pow(base: f64, exp: f64) -> f64;
    pub fn exp(x: f64) -> f64;
    pub fn log(x: f64) -> f64;
    pub fn log10(x: f64) -> f64;
    pub fn floor(x: f64) -> f64;
    pub fn ceil(x: f64) -> f64;
    pub fn fabs(x: f64) -> f64;
    // TIME
    pub fn time(t: *mut i64) -> i64;
    pub fn sleep(seconds: u32) -> u32;
    pub fn usleep(microseconds: u32) -> i32;
    // STRING
    pub fn strlen(s: PCChar) -> usize;
    pub fn strcmp(s1: PCChar, s2: PCChar) -> i32;
    pub fn strcpy(dest: PMutChar, src: PCChar) -> PMutChar;
    pub fn strncpy(dest: PMutChar, src: PCChar, n: usize) -> PMutChar;
}

pub unsafe fn print(s: PCChar) {
    if s.is_null() {
        return;
    }
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    write(1, s as *const u8, len);
}

pub unsafe fn pwd() -> PCChar {
    static mut BUF: [CChar; 256] = [0; 256];
    let ptr = getcwd(
        core::ptr::addr_of_mut!(BUF).cast::<i8>(),
        BUF.len()
    );
    if ptr.is_null() {
        return b"<unknown>\0".as_ptr() as PCChar;
    }
    core::ptr::addr_of!(BUF).cast::<i8>()
}

pub unsafe fn uprint(mut n: u32) {
    let mut buf = [0u8; 10];
    let mut i = 10;
    if n == 0 {
        let zero = b"0";
        write(1, zero.as_ptr(), 1);
        return;
    }
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    let len = 10 - i;
    write(1, buf[i..].as_ptr(), len);
}

pub unsafe fn quit(code: i32) -> ! {
    exit(code);
}

pub unsafe fn getch() -> u8 {
    let mut buf: u8 = 0;
    let n = read(0 as usize, &mut buf as *mut u8, 1);
    if n <= 0 {
        0
    } else {
        buf
    }
}

pub unsafe fn exec(command: PCChar) -> i32 {
    system(command)
}

pub unsafe fn input() -> [u8; 256] {
    let mut buffer = [0u8; 256];
    let _buf: isize = read(0, buffer.as_mut_ptr(), buffer.len() - 1);
    buffer
}

pub unsafe fn ensure_null_terminated(
    input: *const u8,
    input_len: usize,
    output: *mut u8,
    output_len: usize,
) -> PCChar {
    if input_len + 1 > output_len {
        return core::ptr::null();
    }
    for i in 0..input_len {
        *output.add(i) = *input.add(i);
    }
    if input_len == 0 || *input.add(input_len - 1) != 0 {
        *output.add(input_len) = 0;
    }
    output as PCChar
}

pub unsafe fn cd(path: *const u8) -> i32 {
    if path.is_null() {
        return -1;
    }

    let mut len = 0;
    while *path.add(len) != 0 {
        len += 1;
        if len >= 255 {
            break;
        }
    }

    let mut buffer = [0u8; 256];
    let c_path = ensure_null_terminated(path, len, buffer.as_mut_ptr(), buffer.len());
    if c_path.is_null() {
        return -1;
    }

    chdir(c_path)
}

#[derive(Clone, Copy)]
pub struct Token {
    pub word: *const u8,
    pub rest: *const u8,
}

pub unsafe fn splitft(mut input: *const u8) -> Token {
    static mut WORD: [u8; 64] = [0; 64];
    let mut i = 0;

    while *input != 0 && *input != b' ' && i < 63 {
        *core::ptr::addr_of_mut!(WORD[i]) = *input;
        input = input.add(1);
        i += 1;
    }
    *core::ptr::addr_of_mut!(WORD[i]) = 0;

    if *input == b' ' {
        input = input.add(1);
    }

    Token {
        word: core::ptr::addr_of!(WORD).cast::<u8>(),
        rest: input,
    }
}

pub unsafe fn matchpcchar(a: *const u8, b: *const u8) -> bool {
    let mut i = 0;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        i += 1;
    }
}
