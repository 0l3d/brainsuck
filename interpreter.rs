#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod crlib;
use crlib::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let panic_msg = b"PANIC occurred!\n\0";
        write(2, panic_msg.as_ptr(), panic_msg.len() - 1);
        quit(1);
    }
}

const MEMORY_SIZE: usize = 30000;

struct Brain {
    memory: [u8; MEMORY_SIZE],
    whereis: usize,
}

pub unsafe fn brainsucker() -> Brain {
    Brain {
        memory: [0; MEMORY_SIZE],
        whereis: 0,
    }
}

pub unsafe fn suck_in(br: &mut Brain) {
    br.memory[br.whereis] = br.memory[br.whereis].wrapping_add(1);
}

pub unsafe fn suck_de(br: &mut Brain) {
    br.memory[br.whereis] = br.memory[br.whereis].wrapping_sub(1);
}

pub unsafe fn suck_move_right(br: &mut Brain) {
    if br.whereis < MEMORY_SIZE - 1 {
        br.whereis += 1;
    }
}

pub unsafe fn suck_move_left(br: &mut Brain) {
    if br.whereis > 0 {
        br.whereis -= 1;
    }
}

pub unsafe fn get_current(br: &mut Brain) -> u8 {
    br.memory[br.whereis]
}

pub unsafe fn set_data(br: &mut Brain, data: u8) {
    br.memory[br.whereis] = data;
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: i32, argv: *const *const i8) -> i32 {
    if argc < 2 {
        printf(b"Usage: %s <brainfuck_file>\n\0".as_ptr() as PCChar);
        return 1;
    }
    
    let path = *argv.add(1);
    let mode = b"r\0";
    let file = fopen(path, mode.as_ptr() as PCChar);
    let mut brainsuck = brainsucker();
    
    if file.is_null() {
        printf(b"Error: Could not open file\n\0".as_ptr() as PCChar);
        return 1;
    }
    
    let mut code: [u8; 100000] = [0; 100000];
    let mut code_len = 0;
    let mut line_buf: [CChar; 256] = [0; 256];
    
    while !fgets(line_buf.as_mut_ptr(), 256, file).is_null() {
        let mut j = 0;
        while j < 256 && line_buf[j] != 0 && code_len < 99999 {
            code[code_len] = line_buf[j] as u8;
            code_len += 1;
            j += 1;
        }
    }
    
    fclose(file);
    
    let mut i = 0;
    while i < code_len {
        match code[i] as char {
            '+' => suck_in(&mut brainsuck),
            '-' => suck_de(&mut brainsuck),
            '>' => suck_move_right(&mut brainsuck),
            '<' => suck_move_left(&mut brainsuck),
            '.' => {
                let c = get_current(&mut brainsuck);
                write(1, &c as *const u8, 1);
            }
            ',' => {
                let input_char = getch();
                set_data(&mut brainsuck, input_char);
            }
            '[' => {
                if get_current(&mut brainsuck) == 0 {
                    let mut bracket_count = 1;
                    while bracket_count > 0 && i < code_len - 1 {
                        i += 1;
                        match code[i] as char {
                            '[' => bracket_count += 1,
                            ']' => bracket_count -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if get_current(&mut brainsuck) != 0 {
                    let mut bracket_count = 1;
                    while bracket_count > 0 && i > 0 {
                        i -= 1;
                        match code[i] as char {
                            ']' => bracket_count += 1,
                            '[' => bracket_count -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    0
}
