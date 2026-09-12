pub enum MemoryError {
    AddressAlignmentError { address: u32, alignment: u32 },
    InvalidAddressError(u32),
}

pub struct Memory {
    data: [u8; 1024],
}

impl Memory {
    pub fn new() -> Self {
        let data = [0; 1024];

        Self { data: data }
    }

    pub fn read_byte(&self, address: u32) -> Result<u8, MemoryError> {
        self.data
            .get(address as usize)
            .copied()
            .ok_or(MemoryError::InvalidAddressError(address))
    }

    pub fn write_byte(&mut self, address: u32, data: u8) -> Result<(), MemoryError> {
        let byte = self
            .data
            .get_mut(address as usize)
            .ok_or(MemoryError::InvalidAddressError(address))?;

        *byte = data;
        Ok(())
    }

    pub fn read_halfword(&self, address: u32) -> Result<u16, MemoryError> {
        Self::check_alignment(address, 2)?;

        let lsb: u8 = self.read_byte(address)?;
        let msb: u8 = self.read_byte(address + 1)?;
        Ok(u16::from_le_bytes([lsb, msb]))
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) -> Result<(), MemoryError> {
        Self::check_alignment(address, 2)?;

        let [lsb, msb] = value.to_le_bytes();
        self.write_byte(address, lsb)?;
        self.write_byte(address + 1, msb)?;
        Ok(())
    }

    pub fn read_word(&self, address: u32) -> Result<u32, MemoryError> {
        Self::check_alignment(address, 4)?;

        let mut bytes: [u8; 4] = [0; 4];
        for (offset, byte) in bytes.iter_mut().enumerate() {
            *byte = self.read_byte(address + offset as u32)?;
        }
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn write_word(&mut self, address: u32, value: u32) -> Result<(), MemoryError> {
        Self::check_alignment(address, 4)?;

        for (offset, byte) in value.to_le_bytes().into_iter().enumerate() {
            self.write_byte(address + offset as u32, byte)?;
        }
        Ok(())
    }

    fn check_alignment(address: u32, alignment: u32) -> Result<(), MemoryError> {
        debug_assert!(alignment != 0);

        if address % alignment != 0 {
            return Err(MemoryError::AddressAlignmentError { address, alignment });
        }
        Ok(())
    }
}
