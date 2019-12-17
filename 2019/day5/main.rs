struct Computer {
    memory: Vec<isize>,
    ip: usize,
}

fn read_stdin() -> isize {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("STDIn read failed.");
    buffer.trim().parse::<isize>().unwrap()
}

fn write_stdout(number: isize) {
    println!("{}", number);
}


impl Computer {
   fn new(memory: Vec<isize>) -> Self {
        Self {memory, ip: 0}
   }

   // Read from a memory address
   fn get(& self, position: usize) -> isize {
        self.memory[position]
   }

   // Set to a memory address
   fn set(&mut self, position: usize, value: isize) {
        self.memory[position] = value;
   }

   fn arg(&self, offset: usize) -> isize {
        let idx = self.ip + offset;
        let mode: usize = (self.memory[self.ip] as usize) / 10 / 10_usize.pow(offset as u32) % 10_usize;
        match mode {
            0 => self.get(self.memory[idx] as usize), // position
            1 => self.get(idx), // immediate
            op => panic!("Unknown param mode {}", op),
        }
   }

   fn output_address(&self, offset: usize) -> usize {
       let idx = self.ip + offset;
       self.memory[idx] as usize
   }

   fn current_opcode(&self) -> isize {
       self.memory[self.ip] % 100
   }

   fn step(&mut self, size: usize) {
       self.ip += size;
   }

   fn compute(&mut self) {
       loop {
            match self.current_opcode() {
                1 => self.add(),
                2 => self.multiply(),
                3 => self.input(),
                4 => self.output(),
                5 => self.jump_if_true(),
                6 => self.jump_if_false(),
                7 => self.less_than(),
                8 => self.equal_to(),
                99 => break,
                invalid_opcode => panic!("Invalid opcode {}", invalid_opcode),
            }
        }
   }

   // op 1
   fn add(&mut self) {
       let in1 = self.arg(1);
       let in2 = self.arg(2);
       let out = self.output_address(3);
       self.set(out, in1 + in2);
       self.step(4);
   }

   // op 2
   fn multiply(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.output_address(3);

        self.set(out, in1 * in2);
        self.step(4);
   }

   // op 3
   fn input(&mut self) {
        let in1 = read_stdin();
        let out = self.output_address(1);

        self.set(out, in1);
        self.step(2);
   }

    // op4
    fn output(&mut self) {
        let in1 = self.arg(1);
        write_stdout(in1);
        self.step(2);
    }

    // op5
    fn jump_if_true(&mut self) {
        let in1 = self.arg(1);
        if in1 != 0 {
            let in2 = self.arg(2) as usize;
            self.ip = in2;
        } else {
            // Do Nothing
            self.step(3);
        }
    }

    // op6
    fn jump_if_false(&mut self) {
        let in1 = self.arg(1);
        if in1 == 0 {
            let in2 = self.arg(2) as usize;
            self.ip = in2;
        } else {
            // Do nothing
            self.step(3);
        }
    }

    // op7
    fn less_than(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.output_address(3);
        if in1 < in2 {
            self.set(out, 1);
        } else {
            self.set(out, 0);
        }
        self.step(4);
    }

    // op 8
    fn equal_to(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.output_address(3);
        if in1 == in2 {
            self.set(out, 1);
        } else {
            self.set(out, 0);
        }
        self.step(4);
    }

}

fn main() {
  let input: Vec<isize> = std::fs::read_to_string("input.txt").expect("Error Reading File")
  .split(",")
  .filter_map(|s| s.parse::<isize>().ok())
  .collect();

  let mut computer = Computer::new(input);
  computer.compute();
}