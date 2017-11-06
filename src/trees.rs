pub struct MutableInt {
    i: usize
}

pub struct Origin {
    line: Option<usize>,
    start_position: Option<usize>
}

impl Origin {
    pub fn new() -> Self {
        Origin { line: None, start_position: None }
    }

    pub fn set_position(&mut self, line: usize, start: usize) {
        self.line = Some(line);
        self.start_position = Some(start);
    }
}
