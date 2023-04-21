pub mod brainfuck {
    pub struct State {
        current_cell: usize,
        program_char: usize,
        cells: Vec<u8>,
        program: &'static str,
        opening_brackets: Vec<usize>,
        function_table: Vec<(&'static str, fn(state: &mut State, arguments: *const u8))>
    }

    impl State {
        pub fn new(program: &'static str) -> Self {
            Self {
                current_cell: 0,
                program_char: 0,
                cells: vec![0; 1024],
                program: crate::bindings::engine::read_file(program),
                opening_brackets: vec![],
                function_table: vec![]
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
            println!("{}", self.cells[self.current_cell] as char)
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
        pub fn get_cell(&self, cell: usize) -> u8 {
            self.cells[cell]
        }
        pub fn call_function(&mut self) {
            let mut function_name = String::from("");
            let mut i = self.current_cell;
            while self.cells[i] != '%' as u8 {
                function_name.push(self.cells[i] as char);
                i += 1;
            }
            i += 1;
            println!("{}", function_name);
            let mut args: Vec<u8> = vec![];
            while self.cells[i] != '%' as u8 {
                args.push(self.cells[i] as u8);
                i += 1;
            }
            let args_ptr = args[..].as_ptr();

            for i in 0..self.function_table.len() {
                if self.function_table[i].0 == function_name {
                    self.function_table[i].1(self, args_ptr);
                    break;
                } else {
                    println!("Undefined function {}", function_name);
                }
            }
        }
        pub fn register_function(&mut self, name: &'static str, ptr: fn(state: &mut State, arguments: *const u8)) {
            self.function_table.push((name, ptr));
        }
        pub fn cell_count(&self) -> usize {
            self.cells.len()
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
                '%' => self.call_function(),
                _   => {}
            }
            self.next_progchar();
        }

        pub fn arguments(args: *const u8, argcount: usize) -> Vec<u32> {
            let mut retval: Vec<u32> = vec![];
            for i in 0..argcount {
                let idx: usize = unsafe { *(args.add(i * 4) as *const u32) } as usize;
                retval.push(idx as u32);
            }
            retval
        }

        pub fn u32_from_idx(&self, index: usize) -> u32 {
            let mut retval: u32 = 0;
            unsafe { *(&mut retval as *mut u32 as *mut u8) = self.get_cell(index); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(1) = self.get_cell(index + 1); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(2) = self.get_cell(index + 2); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(3) = self.get_cell(index + 3); }
            retval
        }

        pub fn u64_from_idx(&self, index: usize) -> u32 {
            let mut retval: u32 = 0;
            unsafe { *(&mut retval as *mut u32 as *mut u8) = self.get_cell(index); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(1) = self.get_cell(index + 1); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(2) = self.get_cell(index + 2); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(3) = self.get_cell(index + 3); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(4) = self.get_cell(index + 4); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(5) = self.get_cell(index + 5); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(6) = self.get_cell(index + 6); }
            unsafe { *(&mut retval as *mut u32 as *mut u8).add(7) = self.get_cell(index + 7); }
            retval
        }

        pub fn get_string(&self, index: usize) -> String {
            let mut string = String::from("");
            let mut i = index;
            while self.get_cell(i) != '%' as u8 {
                string.push(self.get_cell(i) as char);
                i += 1
            }
            string
        }

        pub fn set_cell(&mut self, index: usize, value: u8) {
            self.cells[index] = value;
        }

        pub fn set_cell_u64(&mut self, index: usize, value: u64) {
            unsafe { *self.cells.as_mut_ptr().add(index) = *(&value as *const u64 as *const u8) };
            unsafe { *self.cells.as_mut_ptr().add(index + 1) = *((&value as *const u64 as *const u8).add(1)) };
            unsafe { *self.cells.as_mut_ptr().add(index + 2) = *((&value as *const u64 as *const u8).add(2)) };
            unsafe { *self.cells.as_mut_ptr().add(index + 3) = *((&value as *const u64 as *const u8).add(3)) };
            unsafe { *self.cells.as_mut_ptr().add(index + 4) = *((&value as *const u64 as *const u8).add(4)) };
            unsafe { *self.cells.as_mut_ptr().add(index + 5) = *((&value as *const u64 as *const u8).add(5)) };
            unsafe { *self.cells.as_mut_ptr().add(index + 6) = *((&value as *const u64 as *const u8).add(6)) };
            unsafe { *self.cells.as_mut_ptr().add(index + 7) = *((&value as *const u64 as *const u8).add(7)) };
        }
    }
}

mod aaascript {

}

