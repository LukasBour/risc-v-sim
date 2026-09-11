#[derive(Debug, PartialEq, Eq)]
pub enum RegWriteError {
    ZeroRegister,
    InvalidIndex(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RegReadError {
    InvalidIndex(usize),
}

pub struct RegFile {
    values: [u32; 32],
    t_read_ps: u32,
    t_write_ps: u32,
}

impl Default for RegFile {
    fn default() -> Self {
        Self::new(100, 150)
    }
}

impl RegFile {
    pub fn new(t_read_ps: u32, t_write_ps: u32) -> Self {
        Self {
            values: [0; 32],
            t_read_ps: t_read_ps,
            t_write_ps: t_write_ps,
        }
    }

    pub fn reg_value(&self, index: usize) -> Result<u32, RegReadError> {
        self.values
            .get(index)
            .copied()
            .ok_or(RegReadError::InvalidIndex(index))
    }

    pub fn write_reg(&mut self, index: usize, value: u32) -> Result<(), RegWriteError> {
        if index == 0 {
            return Err(RegWriteError::ZeroRegister);
        }

        let register = self
            .values
            .get_mut(index)
            .ok_or(RegWriteError::InvalidIndex(index))?;

        *register = value;
        Ok(())
    }

    pub fn reg_info(index: usize) -> Result<&'static RegisterInfo, RegReadError> {
        REGISTER_INFO
            .get(index)
            .ok_or(RegReadError::InvalidIndex(index))
    }

    pub fn t_read_ps(&self) -> u32 {
        self.t_read_ps
    }

    pub fn t_write_ps(&self) -> u32 {
        self.t_write_ps
    }

    pub fn set_t_write_ps(&mut self, t_write_ps: u32) {
        self.t_write_ps = t_write_ps
    }

    pub fn set_t_read_ps(&mut self, t_read_ps: u32) {
        self.t_read_ps = t_read_ps
    }
}

pub struct RegisterInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub additional_info: &'static str,
}

pub const REGISTER_INFO: [RegisterInfo; 32] = [
    RegisterInfo {
        name: "zero",
        description: "Constant value 0",
        additional_info: "Immutable",
    },
    RegisterInfo {
        name: "ra",
        description: "Return address",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "sp",
        description: "Stack pointer",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "gp",
        description: "Global pointer",
        additional_info: "Unallocatable",
    },
    RegisterInfo {
        name: "tp",
        description: "Thread pointer",
        additional_info: "Unallocatable",
    },
    RegisterInfo {
        name: "t0",
        description: "Temporary Register 0",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "t1",
        description: "Temporary Register 1",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "t2",
        description: "Temporary Register 2",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "s0",
        description: "Callee-saved register 0",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s1",
        description: "Callee-saved register 1",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "a0",
        description: "Argument regsiter 0",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "a1",
        description: "Argument regsiter 1",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "a2",
        description: "Argument regsiter 2",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "a3",
        description: "Argument regsiter 3",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "a4",
        description: "Argument regsiter 4",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "a5",
        description: "Argument regsiter 5",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "a6",
        description: "Argument regsiter 6",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "a7",
        description: "Argument regsiter 7",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "s2",
        description: "Callee-saved register 2",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s3",
        description: "Callee-saved register 3",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s4",
        description: "Callee-saved register 4",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s5",
        description: "Callee-saved register 5",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s6",
        description: "Callee-saved register 6",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s7",
        description: "Callee-saved register 7",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s8",
        description: "Callee-saved register 8",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s9",
        description: "Callee-saved register 9",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s10",
        description: "Callee-saved register 10",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "s11",
        description: "Callee-saved register 11",
        additional_info: "Callee saved",
    },
    RegisterInfo {
        name: "t3",
        description: "Temporary register 3",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "t4",
        description: "Temporary register 4",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "t5",
        description: "Temporary regsiter 5",
        additional_info: "Caller saved",
    },
    RegisterInfo {
        name: "t6",
        description: "Temporary Register 6",
        additional_info: "Caller saved",
    },
];
