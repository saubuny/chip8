pub const STACK_SIZE: usize = 16;

pub struct Stack {
    pub data: [u16; STACK_SIZE],
    pub sp: u16,
}

impl Stack {
    pub fn push(&mut self, val: u16) {
        self.data[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.data[self.sp as usize]
    }
}
