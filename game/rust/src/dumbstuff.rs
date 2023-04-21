
// For functions:
//                            (in hex, little endian)
//                            20 03 00 00 58 20 00 00 .... 
//      Function in memory:   800 600 hello!\0 window_create%0000 0400 0800%
//      to call the function, have the current cell be the value with the 'w' in window_create

pub mod brainfuck {
    pub struct State {
        current_cell: usize,
        program_char: usize,
        cells: Vec<u8>,
        program: &'static str,
        opening_brackets: Vec<usize>,
    }

    impl State {
        pub fn new(program: &'static str) -> Self {
            Self {
                current_cell: 0,
                program_char: 0,
                cells: vec![0; 1024],
                program: crate::bindings::engine::read_file(program),
                opening_brackets: vec![],
            }
        }
        pub fn delete(&self) {
            unsafe { crate::bindings::engine::free(self.program.as_ptr() as *mut std::ffi::c_void); }
        }
        pub fn next_cell(&mut self) {
            self.current_cell += 1;
        }
        pub fn prev_cell(&mut self) {
            self.current_cell -= 1;
        }
        pub fn inc_cell(&mut self) {
            self.cells[self.current_cell] = self.cells[self.current_cell].wrapping_add(1);
        }
        pub fn dec_cell(&mut self) {
            self.cells[self.current_cell] = self.cells[self.current_cell].wrapping_sub(1);
        }
        pub fn print_cell(&mut self) {
            println!("{} ({})", self.cells[self.current_cell] as char, self.cells[self.current_cell])
        }
        pub fn opening_bracket(&mut self) {
            self.opening_brackets.push(self.program_char);
        }
        pub fn closing_bracket(&mut self) {
            if self.cells[self.current_cell] != 0 {
                self.program_char = self.opening_brackets[self.opening_brackets.len() - 1];
            } else {
                if self.opening_brackets.pop() == None {
                    println!("Bad pop");
                }
            }
        }
        pub fn progchar(&self) -> char {
            if self.program_char >= self.program.len() {
                return '\0'
            }
            self.program.as_bytes()[self.program_char] as char
        }
        pub fn next_progchar(&mut self) {
            self.program_char += 1
        }
        pub fn execute(&mut self) {
            match self.progchar() {
                '>' => self.next_cell(),
                '<' => self.prev_cell(),
                '+' => self.inc_cell(),
                '-' => self.dec_cell(),
                '.' => self.print_cell(),
                '[' => self.opening_bracket(),
                ']' => self.closing_bracket(),
                _   => {}
            }
            self.next_progchar();
        }
    }
}
