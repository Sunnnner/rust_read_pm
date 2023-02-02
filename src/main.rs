use std::{io::Write, ffi::c_void};

use windows::Win32::{System::Threading::{OpenProcess, PROCESS_ALL_ACCESS}, Foundation::{HANDLE, GetLastError}};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

fn read_line_address() -> u64 {
    let mut address = String::new();
    std::io::stdin().read_line(&mut address).unwrap();
    let address_target = address.trim().trim_start_matches("0x");
    return u64::from_str_radix(address_target, 16).unwrap();
}


fn read_exe<T: Default>(hand_process: HANDLE, address: u64) -> T {
    let mut read_target_ptr_void: T = Default::default();
    unsafe {
        ReadProcessMemory(
            hand_process,
            address as *const c_void,
            &mut read_target_ptr_void as *mut T as *mut c_void,
            std::mem::size_of::<T>() as usize,
            None
        );
        let b = GetLastError();

        println!("b {:?}", b)
    };
    return read_target_ptr_void;
}

// https://github.com/Whimfoome/rust_read_write_winpm/blob/master/read_mem/src/main.rs

#[warn(unused_unsafe)]
fn main() {
    let pid_target:u32;
    println!("Please input the pid:");
    std::io::stdout().flush().unwrap();
    let mut pid = String::new();
    std::io::stdin().read_line(&mut pid).unwrap();
    pid_target = pid.trim().parse::<u32>().unwrap();

    let hand_process = unsafe {
        OpenProcess(
            PROCESS_ALL_ACCESS, 
            false, 
            pid_target
        )
    };
    match hand_process {
        Ok(handle) => {
            unsafe{
                let address_target: u64;
                println!("Please input the address:");
                std::io::stdout().flush().unwrap();
                address_target = read_line_address();
                println!("address_target: 0x{:x}", address_target);
                let read_target = read_exe::<i32>(handle, address_target);
                println!("read_target: {}", read_target);
            }
        },
        Err(_) => println!("OpenProcess failed!"),
    }
        
    

 
    
}   
