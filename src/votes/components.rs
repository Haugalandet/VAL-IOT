pub struct Vote {
    red: usize,
    blue: usize
}

impl Vote {
    pub fn vote_red(&mut self) {
        self.red += 1;
    }

    pub fn vote_blue(&mut self) {
        self.red += 1;
    }
}