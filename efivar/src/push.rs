pub trait PushVecU8 {
    fn push_u8(&mut self, value: u8);
    fn push_u16(&mut self, value: u16);
    fn push_u32(&mut self, value: u32);
    fn push_u64(&mut self, value: u64);
}
impl PushVecU8 for Vec<u8> {
    fn push_u8(&mut self, value: u8) {
        self.push(value)
    }

    fn push_u16(&mut self, value: u16) {
        self.append(&mut value.to_le_bytes().to_vec())
    }

    fn push_u32(&mut self, value: u32) {
        self.append(&mut value.to_le_bytes().to_vec())
    }

    fn push_u64(&mut self, value: u64) {
        self.append(&mut value.to_le_bytes().to_vec())
    }
}
