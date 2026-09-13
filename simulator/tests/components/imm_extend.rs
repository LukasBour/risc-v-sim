use riscv_simulator::components::ImmExtend;

const TYPE_I: u32 = 0b00;
const TYPE_S: u32 = 0b01;
const TYPE_B: u32 = 0b10;
const TYPE_J: u32 = 0b11;

fn expect_ok<T, E: core::fmt::Debug>(result: Result<T, E>) -> T {
    result.expect("expected immediate extension to succeed")
}

fn encode_i_immediate(immediate: u32) -> u32 {
    (immediate & 0x0fff) << 13
}

fn encode_s_immediate(immediate: u32) -> u32 {
    ((immediate >> 5) & 0x7f) << 18 | (immediate & 0x1f)
}

fn encode_b_immediate(immediate: u32) -> u32 {
    ((immediate >> 12) & 0x01) << 24
        | ((immediate >> 11) & 0x01)
        | ((immediate >> 5) & 0x3f) << 18
        | ((immediate >> 1) & 0x0f) << 1
}

fn encode_j_immediate(immediate: u32) -> u32 {
    ((immediate >> 20) & 0x01) << 24
        | ((immediate >> 12) & 0xff) << 5
        | ((immediate >> 11) & 0x01) << 13
        | ((immediate >> 1) & 0x03ff) << 14
}

#[test]
fn zero_is_extended_to_zero_for_every_immediate_type() {
    for imm_src in [TYPE_I, TYPE_S, TYPE_B, TYPE_J] {
        assert_eq!(expect_ok(ImmExtend::extend(0, imm_src)), 0);
    }
}

#[test]
fn type_i_positive_immediate_is_extracted() {
    let instruction31_7 = encode_i_immediate(0x5a3);

    assert_eq!(expect_ok(ImmExtend::extend(instruction31_7, TYPE_I)), 0x5a3);
}

#[test]
fn type_i_negative_immediate_is_sign_extended() {
    let instruction31_7 = encode_i_immediate(0x800);

    assert_eq!(
        expect_ok(ImmExtend::extend(instruction31_7, TYPE_I)),
        0xffff_f800
    );
}

#[test]
fn type_s_positive_immediate_is_reassembled() {
    let instruction31_7 = encode_s_immediate(0x5a3);

    assert_eq!(expect_ok(ImmExtend::extend(instruction31_7, TYPE_S)), 0x5a3);
}

#[test]
fn type_s_negative_immediate_is_reassembled_and_sign_extended() {
    let instruction31_7 = encode_s_immediate(0xfff);

    assert_eq!(
        expect_ok(ImmExtend::extend(instruction31_7, TYPE_S)),
        u32::MAX
    );
}

#[test]
fn type_b_positive_immediate_is_reassembled() {
    let instruction31_7 = encode_b_immediate(0x05aa);

    assert_eq!(
        expect_ok(ImmExtend::extend(instruction31_7, TYPE_B)),
        0x05aa
    );
}

#[test]
fn type_b_negative_immediate_is_reassembled_and_sign_extended() {
    let instruction31_7 = encode_b_immediate(0x1ffe);

    assert_eq!(
        expect_ok(ImmExtend::extend(instruction31_7, TYPE_B)),
        0xffff_fffe
    );
}

#[test]
fn type_b_result_is_always_even() {
    let instruction31_7 = encode_b_immediate(0x0aaa);

    assert_eq!(expect_ok(ImmExtend::extend(instruction31_7, TYPE_B)) & 1, 0);
}

#[test]
fn type_j_positive_immediate_is_reassembled() {
    let instruction31_7 = encode_j_immediate(0x0a_55aa);

    assert_eq!(
        expect_ok(ImmExtend::extend(instruction31_7, TYPE_J)),
        0x0a_55aa
    );
}

#[test]
fn type_j_negative_immediate_is_reassembled_and_sign_extended() {
    let instruction31_7 = encode_j_immediate(0x1f_fffe);

    assert_eq!(
        expect_ok(ImmExtend::extend(instruction31_7, TYPE_J)),
        0xffff_fffe
    );
}

#[test]
fn type_j_result_is_always_even() {
    let instruction31_7 = encode_j_immediate(0x0a_aaaa);

    assert_eq!(expect_ok(ImmExtend::extend(instruction31_7, TYPE_J)) & 1, 0);
}

#[test]
fn maximum_positive_immediates_are_not_sign_extended() {
    assert_eq!(
        expect_ok(ImmExtend::extend(encode_i_immediate(0x7ff), TYPE_I)),
        0x7ff
    );
    assert_eq!(
        expect_ok(ImmExtend::extend(encode_s_immediate(0x7ff), TYPE_S)),
        0x7ff
    );
    assert_eq!(
        expect_ok(ImmExtend::extend(encode_b_immediate(0x0ffe), TYPE_B)),
        0x0ffe
    );
    assert_eq!(
        expect_ok(ImmExtend::extend(encode_j_immediate(0x0f_fffe), TYPE_J)),
        0x0f_fffe
    );
}

#[test]
fn unused_instruction_bits_do_not_affect_type_i_immediate() {
    let instruction31_7 = encode_i_immediate(0x123) | 0x1fff;

    assert_eq!(expect_ok(ImmExtend::extend(instruction31_7, TYPE_I)), 0x123);
}

#[test]
fn value_larger_than_instruction_31_7_wire_is_rejected() {
    assert!(ImmExtend::extend(1 << 25, TYPE_I).is_err());
    assert!(ImmExtend::extend(u32::MAX, TYPE_I).is_err());
}

#[test]
fn invalid_immediate_source_is_rejected() {
    assert!(ImmExtend::extend(0, 0b100).is_err());
    assert!(ImmExtend::extend(0, u32::MAX).is_err());
}
