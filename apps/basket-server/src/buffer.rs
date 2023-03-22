pub type BufferCapacity = u16;

pub const BUFFER_CAPACITY: BufferCapacity = BufferCapacity::MAX;
pub type Buffer = [u8; BUFFER_CAPACITY as usize];
