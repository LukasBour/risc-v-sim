use crate::utils::extract_bits_32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ImmSrc {
    TypeI = 0b00,
    TypeS = 0b01,
    TypeB = 0b10,
    TypeJ = 0b11,
}

impl TryFrom<u32> for ImmSrc {
    type Error = ImmExtendError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(ImmSrc::TypeI),
            0b01 => Ok(ImmSrc::TypeS),
            0b10 => Ok(ImmSrc::TypeB),
            0b11 => Ok(ImmSrc::TypeJ),
            _ => Err(ImmExtendError::InvalidImmediateSourceError(value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImmExtendError {
    InvalidInstructionFormatError(u32),
    InvalidImmediateSourceError(u32),
}

pub struct ImmExtend {}

impl ImmExtend {
    pub fn extend(instruction31_7: u32, imm_src: u32) -> Result<u32, ImmExtendError> {
        Self::check_instruction_format(instruction31_7)?;
        let imm_src_type = ImmSrc::try_from(imm_src)?;

        let immediate = match imm_src_type {
            ImmSrc::TypeI => instruction31_7 >> 13,
            ImmSrc::TypeS => {
                extract_bits_32(instruction31_7, 24, 18) << 5
                    | extract_bits_32(instruction31_7, 4, 0)
            }
            ImmSrc::TypeB => {
                extract_bits_32(instruction31_7, 24, 24) << 12
                    | extract_bits_32(instruction31_7, 0, 0) << 11
                    | extract_bits_32(instruction31_7, 23, 18) << 5
                    | extract_bits_32(instruction31_7, 4, 1) << 1
            }
            ImmSrc::TypeJ => {
                extract_bits_32(instruction31_7, 24, 24) << 20
                    | extract_bits_32(instruction31_7, 12, 5) << 12
                    | extract_bits_32(instruction31_7, 13, 13) << 11
                    | extract_bits_32(instruction31_7, 23, 14) << 1
            }
        };

        Ok(Self::sign_extend_immediate(immediate, imm_src_type))
    }

    fn check_instruction_format(instruction31_7: u32) -> Result<(), ImmExtendError> {
        if instruction31_7 & (u32::MAX << 25) != 0 {
            Err(ImmExtendError::InvalidInstructionFormatError(
                instruction31_7,
            ))
        } else {
            Ok(())
        }
    }

    fn immediate_size_by_src(imm_src: ImmSrc) -> u8 {
        match imm_src {
            ImmSrc::TypeI | ImmSrc::TypeS => 12,
            ImmSrc::TypeB => 13,
            ImmSrc::TypeJ => 21,
        }
    }

    fn sign_extend_immediate(immediate: u32, imm_src: ImmSrc) -> u32 {
        let imm_size = Self::immediate_size_by_src(imm_src);
        debug_assert!(immediate & (u32::MAX << imm_size) == 0);

        if immediate & (0b1 << (imm_size - 1)) != 0 {
            immediate | (u32::MAX << imm_size)
        } else {
            immediate
        }
    }
}
