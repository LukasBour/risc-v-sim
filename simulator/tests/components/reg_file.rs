use riscv_simulator::components::{RegFile, RegReadError, RegWriteError};

#[test]
fn all_registers_start_at_zero() {
    let reg_file = RegFile::default();

    for index in 0..32 {
        assert_eq!(reg_file.reg_value(index), Ok(0));
    }
}

#[test]
fn registers_x1_to_x31_can_be_written() {
    let mut reg_file = RegFile::default();

    for index in 1..32 {
        let value = index as u32 * 10;

        assert_eq!(reg_file.write_reg(index, value), Ok(()));
        assert_eq!(reg_file.reg_value(index), Ok(value));
    }
}

#[test]
fn x0_cannot_be_changed() {
    let mut reg_file = RegFile::default();

    assert_eq!(reg_file.write_reg(0, 42), Err(RegWriteError::ZeroRegister));
    assert_eq!(reg_file.reg_value(0), Ok(0));
}

#[test]
fn reading_invalid_index_returns_error() {
    let reg_file = RegFile::default();

    assert_eq!(reg_file.reg_value(32), Err(RegReadError::InvalidIndex(32)));
}

#[test]
fn reading_very_large_index_returns_error() {
    let reg_file = RegFile::default();

    assert_eq!(
        reg_file.reg_value(usize::MAX),
        Err(RegReadError::InvalidIndex(usize::MAX))
    );
}

#[test]
fn writing_invalid_index_returns_error() {
    let mut reg_file = RegFile::default();

    assert_eq!(
        reg_file.write_reg(32, 42),
        Err(RegWriteError::InvalidIndex(32))
    );
}

#[test]
fn register_value_can_be_overwritten() {
    let mut reg_file = RegFile::default();

    reg_file.write_reg(5, 10).unwrap();
    reg_file.write_reg(5, 20).unwrap();

    assert_eq!(reg_file.reg_value(5), Ok(20));
}

#[test]
fn writing_register_does_not_change_other_registers() {
    let mut reg_file = RegFile::default();

    reg_file.write_reg(5, 42).unwrap();

    assert_eq!(reg_file.reg_value(4), Ok(0));
    assert_eq!(reg_file.reg_value(5), Ok(42));
    assert_eq!(reg_file.reg_value(6), Ok(0));
}

#[test]
fn default_timings_are_correct() {
    let reg_file = RegFile::default();

    assert_eq!(reg_file.t_read_ps(), 100);
    assert_eq!(reg_file.t_write_ps(), 150);
}

#[test]
fn custom_timings_are_stored() {
    let reg_file = RegFile::new(80, 120);

    assert_eq!(reg_file.t_read_ps(), 80);
    assert_eq!(reg_file.t_write_ps(), 120);
}

#[test]
fn timings_can_be_changed() {
    let mut reg_file = RegFile::default();

    reg_file.set_t_read_ps(50);
    reg_file.set_t_write_ps(75);

    assert_eq!(reg_file.t_read_ps(), 50);
    assert_eq!(reg_file.t_write_ps(), 75);
}

#[test]
fn every_register_has_metadata() {
    for index in 0..32 {
        assert!(RegFile::reg_info(index).is_ok());
    }
}

#[test]
fn invalid_metadata_index_returns_error() {
    assert!(matches!(
        RegFile::reg_info(32),
        Err(RegReadError::InvalidIndex(32))
    ));
}

#[test]
fn important_register_names_are_correct() {
    assert_eq!(RegFile::reg_info(0).unwrap().name, "zero");
    assert_eq!(RegFile::reg_info(1).unwrap().name, "ra");
    assert_eq!(RegFile::reg_info(2).unwrap().name, "sp");
    assert_eq!(RegFile::reg_info(31).unwrap().name, "t6");
}
