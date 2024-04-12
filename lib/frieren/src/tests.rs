use core::mem;

use crate::*;

#[test]
// Just checking that we load tests
fn trivial() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

const HELLO_WORLD_ELF: &[u8] = include_bytes!("../test-files/test-elf.o");
const FILE_HEADER_SIZE: usize = 64;

#[test]
fn file_header_size() {
    // We are always using 64-bit, so the size of the file header is known
    assert_eq!(mem::size_of::<FileHeader>(), FILE_HEADER_SIZE);
}

#[test]
fn header_loading() {
    println!("First bytes: {:x?}", &HELLO_WORLD_ELF[0..4]);
    let ptr = HELLO_WORLD_ELF as *const [u8] as *const FileHeader;
    // Some of this should be extracted to a larger ELF struct at some point, but this works for now
    let file_header: Result<&FileHeader, ElfError> = unsafe { 
        FileHeader::try_from_raw(ptr)
    };
    println!("File Header Status: {:?}", file_header.as_ref().err());
    assert!(file_header.is_ok());
}

