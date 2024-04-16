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
    let ptr = HELLO_WORLD_ELF.as_ptr() as *const FileHeader;
    // Some of this should be extracted to a larger ELF struct at some point, but this works for now
    let file_header: Result<&FileHeader, ElfError> = unsafe { 
        FileHeader::try_from_raw(ptr)
    };
    println!("File Header Status: {:?}", file_header.as_ref().err());
    assert!(file_header.is_ok());
    let file_header = file_header.unwrap();
    // Assert that the read in size for this header is the same as our struct
    println!("Checking File Header size");
    assert_eq!(mem::size_of::<FileHeader>(), file_header.size.into());

    println!("Checking Program Header size");
    // Assert that the read in size of a program header is the same as our struct
    assert_eq!(mem::size_of::<ProgramHeader>(), file_header.program_header_size.into());

    println!("Checking Section Header size");
    // Assert that the read in size of a section header is the same as our struct
    assert_eq!(mem::size_of::<SectionHeader>(), file_header.section_header_size.into());

    let entries = file_header.program_table_entries as usize;
    let offset = file_header.program_table_offset;
    let possible = (HELLO_WORLD_ELF.len() - offset as usize) / mem::size_of::<ProgramHeader>();
    println!("Program headers: ");
    let mut program_headers = vec![];
    for entry in 0..entries {
        let header = unsafe { ProgramHeader::from_ptr(
            HELLO_WORLD_ELF.as_ptr()
            .add(file_header.program_table_offset as usize)
            .add(entry * mem::size_of::<ProgramHeader>())
        )};

        program_headers.push(header);
        println!("{:x?}", program_headers[entry]);
    }
}

