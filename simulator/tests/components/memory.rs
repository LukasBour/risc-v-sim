use riscv_simulator::components::{Memory, MemoryError};

const MEMORY_SIZE: u32 = 1024;

fn expect_ok<T>(result: Result<T, MemoryError>) -> T {
    match result {
        Ok(value) => value,
        Err(_) => panic!("expected memory operation to succeed"),
    }
}

#[test]
fn memory_starts_zeroed() {
    let memory = Memory::new();

    for address in 0..MEMORY_SIZE {
        assert_eq!(expect_ok(memory.read_byte(address)), 0);
    }
}

#[test]
fn byte_can_be_written_and_read() {
    let mut memory = Memory::new();

    assert!(memory.write_byte(123, 0xab).is_ok());
    assert_eq!(expect_ok(memory.read_byte(123)), 0xab);
}

#[test]
fn byte_can_be_written_at_memory_boundaries() {
    let mut memory = Memory::new();

    assert!(memory.write_byte(0, 0x12).is_ok());
    assert!(memory.write_byte(MEMORY_SIZE - 1, 0x34).is_ok());

    assert_eq!(expect_ok(memory.read_byte(0)), 0x12);
    assert_eq!(expect_ok(memory.read_byte(MEMORY_SIZE - 1)), 0x34);
}

#[test]
fn byte_access_outside_memory_returns_error() {
    let mut memory = Memory::new();

    assert!(matches!(
        memory.read_byte(MEMORY_SIZE),
        Err(MemoryError::InvalidAddressError(address)) if address == MEMORY_SIZE
    ));
    assert!(matches!(
        memory.write_byte(MEMORY_SIZE, 0xff),
        Err(MemoryError::InvalidAddressError(address)) if address == MEMORY_SIZE
    ));
    assert!(matches!(
        memory.read_byte(u32::MAX),
        Err(MemoryError::InvalidAddressError(address)) if address == u32::MAX
    ));
}

#[test]
fn halfword_can_be_written_and_read() {
    let mut memory = Memory::new();

    assert!(memory.write_halfword(100, 0x1234).is_ok());
    assert_eq!(expect_ok(memory.read_halfword(100)), 0x1234);
}

#[test]
fn halfword_is_stored_in_little_endian_order() {
    let mut memory = Memory::new();

    assert!(memory.write_halfword(100, 0x1234).is_ok());

    assert_eq!(expect_ok(memory.read_byte(100)), 0x34);
    assert_eq!(expect_ok(memory.read_byte(101)), 0x12);
}

#[test]
fn halfword_can_be_accessed_at_last_aligned_address() {
    let mut memory = Memory::new();
    let address = MEMORY_SIZE - 2;

    assert!(memory.write_halfword(address, 0xabcd).is_ok());
    assert_eq!(expect_ok(memory.read_halfword(address)), 0xabcd);
}

#[test]
fn unaligned_halfword_access_returns_alignment_error() {
    let mut memory = Memory::new();

    assert!(matches!(
        memory.read_halfword(3),
        Err(MemoryError::AddressAlignmentError {
            address: 3,
            alignment: 2
        })
    ));
    assert!(matches!(
        memory.write_halfword(3, 0x1234),
        Err(MemoryError::AddressAlignmentError {
            address: 3,
            alignment: 2
        })
    ));
}

#[test]
fn aligned_halfword_access_outside_memory_returns_error() {
    let mut memory = Memory::new();

    assert!(matches!(
        memory.read_halfword(MEMORY_SIZE),
        Err(MemoryError::InvalidAddressError(address)) if address == MEMORY_SIZE
    ));
    assert!(matches!(
        memory.write_halfword(MEMORY_SIZE, 0x1234),
        Err(MemoryError::InvalidAddressError(address)) if address == MEMORY_SIZE
    ));
}

#[test]
fn word_can_be_written_and_read() {
    let mut memory = Memory::new();

    assert!(memory.write_word(200, 0x1234_5678).is_ok());
    assert_eq!(expect_ok(memory.read_word(200)), 0x1234_5678);
}

#[test]
fn word_is_stored_in_little_endian_order() {
    let mut memory = Memory::new();

    assert!(memory.write_word(200, 0x1234_5678).is_ok());

    assert_eq!(expect_ok(memory.read_byte(200)), 0x78);
    assert_eq!(expect_ok(memory.read_byte(201)), 0x56);
    assert_eq!(expect_ok(memory.read_byte(202)), 0x34);
    assert_eq!(expect_ok(memory.read_byte(203)), 0x12);
}

#[test]
fn word_can_be_accessed_at_last_aligned_address() {
    let mut memory = Memory::new();
    let address = MEMORY_SIZE - 4;

    assert!(memory.write_word(address, 0xdead_beef).is_ok());
    assert_eq!(expect_ok(memory.read_word(address)), 0xdead_beef);
}

#[test]
fn unaligned_word_access_returns_alignment_error() {
    let mut memory = Memory::new();

    for address in [1, 2, 3] {
        assert!(matches!(
            memory.read_word(address),
            Err(MemoryError::AddressAlignmentError {
                address: error_address,
                alignment: 4
            }) if error_address == address
        ));
        assert!(matches!(
            memory.write_word(address, 0x1234_5678),
            Err(MemoryError::AddressAlignmentError {
                address: error_address,
                alignment: 4
            }) if error_address == address
        ));
    }
}

#[test]
fn aligned_word_access_outside_memory_returns_error() {
    let mut memory = Memory::new();

    assert!(matches!(
        memory.read_word(MEMORY_SIZE),
        Err(MemoryError::InvalidAddressError(address)) if address == MEMORY_SIZE
    ));
    assert!(matches!(
        memory.write_word(MEMORY_SIZE, 0x1234_5678),
        Err(MemoryError::InvalidAddressError(address)) if address == MEMORY_SIZE
    ));
}

#[test]
fn word_and_halfword_accesses_use_the_same_bytes() {
    let mut memory = Memory::new();

    assert!(memory.write_word(0, 0x1234_5678).is_ok());

    assert_eq!(expect_ok(memory.read_halfword(0)), 0x5678);
    assert_eq!(expect_ok(memory.read_halfword(2)), 0x1234);
}

#[test]
fn writing_one_location_does_not_change_neighbouring_bytes() {
    let mut memory = Memory::new();

    assert!(memory.write_byte(100, 0xaa).is_ok());

    assert_eq!(expect_ok(memory.read_byte(99)), 0);
    assert_eq!(expect_ok(memory.read_byte(100)), 0xaa);
    assert_eq!(expect_ok(memory.read_byte(101)), 0);
}

#[test]
fn rejected_unaligned_write_does_not_modify_memory() {
    let mut memory = Memory::new();

    assert!(memory.write_word(1, 0x1234_5678).is_err());

    for address in 0..5 {
        assert_eq!(expect_ok(memory.read_byte(address)), 0);
    }
}
