use core::mem;

use crate::*;

#[test]
// Just checking that we load tests
fn trivial() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

const HELLO_WORLD_ELF: &[u8] = include_bytes!("../test-files/test-elf.o");

#[test]
fn check_header() {
    println!("First bytes: {:x?}", &HELLO_WORLD_ELF[0..4]);
    let ptr = HELLO_WORLD_ELF as *const [u8] as *const FileHeader;
    println!("Magic Bytes: {:x?}", unsafe { &*ptr }.magic_bytes);
    println!("Size: {:?}", {
        let size = unsafe { &*ptr }.size;
        size
    });
    // Some of this should be extracted to a larger ELF struct at some point, but this works for now
    let file_header: Result<&FileHeader, ElfError> = unsafe { 
        FileHeader::try_from_raw(ptr)
    };
    println!("File Header Status: {:?}", file_header.as_ref().err());
    assert!(file_header.is_ok());
}