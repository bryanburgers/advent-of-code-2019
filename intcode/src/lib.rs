//! Intcode processor that runs intcode for questions for multiple days
#![deny(missing_docs)]

use std::collections::VecDeque;

/// An error that can occur from running an intcode process
#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeError {
    /// The instruction found at <location> was unknown or unexpected
    UnknownInstruction(isize),
    /// Instruction 99 (halt and catch fire) was executed
    CatchFire,
    /// An instruction tried to access memory at <location> which is outside of the memory space
    Segfault(isize),
    /// The input instruction was executed, but no inputs were available
    NoInputAvailable,
}

/// The type of the input parameter
#[derive(Debug)]
enum InputParameter {
    /// Position mode means the parameter refers to a location in the memory space
    Position,
    /// Immediate mode means the parameter refers to the value that should be used
    Immediate,
    /// Like position mode, but relative to the relative offset register
    Relative,
}

/// The type of the output parameter
#[derive(Debug)]
enum OutputParameter {
    /// Position mode means the parameter refers to a location in the memory space
    Position,
    /// Like position mode, but relative to the relative offset register
    Relative,
}

#[derive(Debug)]
enum Instruction {
    Add(InputParameter, InputParameter, OutputParameter),
    Mul(InputParameter, InputParameter, OutputParameter),
    Input(OutputParameter),
    Output(InputParameter),
    JumpIfTrue(InputParameter, InputParameter),
    JumpIfFalse(InputParameter, InputParameter),
    LessThan(InputParameter, InputParameter, OutputParameter),
    Equals(InputParameter, InputParameter, OutputParameter),
    RelativeMode(InputParameter),
    Halt,
}

impl Instruction {
    pub fn decode(instruction: isize) -> Result<Self, ()> {
        let instruction = match instruction % 100 {
            1 => Instruction::Add(
                Self::decode_input_mode(instruction, 2)?,
                Self::decode_input_mode(instruction, 3)?,
                Self::decode_output_mode(instruction, 4)?,
            ),
            2 => Instruction::Mul(
                Self::decode_input_mode(instruction, 2)?,
                Self::decode_input_mode(instruction, 3)?,
                Self::decode_output_mode(instruction, 4)?,
            ),
            3 => Instruction::Input(Self::decode_output_mode(instruction, 2)?),
            4 => Instruction::Output(Self::decode_input_mode(instruction, 2)?),
            5 => Instruction::JumpIfTrue(
                Self::decode_input_mode(instruction, 2)?,
                Self::decode_input_mode(instruction, 3)?,
            ),
            6 => Instruction::JumpIfFalse(
                Self::decode_input_mode(instruction, 2)?,
                Self::decode_input_mode(instruction, 3)?,
            ),
            7 => Instruction::LessThan(
                Self::decode_input_mode(instruction, 2)?,
                Self::decode_input_mode(instruction, 3)?,
                Self::decode_output_mode(instruction, 4)?,
            ),
            8 => Instruction::Equals(
                Self::decode_input_mode(instruction, 2)?,
                Self::decode_input_mode(instruction, 3)?,
                Self::decode_output_mode(instruction, 4)?,
            ),
            9 => Instruction::RelativeMode(Self::decode_input_mode(instruction, 2)?),
            99 => Instruction::Halt,
            _ => Err(())?,
        };

        Ok(instruction)
    }

    pub fn encode(&self) -> isize {
        use Instruction::*;
        match self {
            Add(in2, in3, out4) => {
                1 + Self::encode_input_mode(in2, 2)
                    + Self::encode_input_mode(in3, 3)
                    + Self::encode_output_mode(out4, 4)
            }
            Mul(in2, in3, out4) => {
                2 + Self::encode_input_mode(in2, 2)
                    + Self::encode_input_mode(in3, 3)
                    + Self::encode_output_mode(out4, 4)
            }
            Input(out2) => 3 + Self::encode_output_mode(out2, 2),
            Output(in2) => 4 + Self::encode_input_mode(in2, 2),
            JumpIfTrue(in2, in3) => {
                5 + Self::encode_input_mode(in2, 2) + Self::encode_input_mode(in3, 3)
            }
            JumpIfFalse(in2, in3) => {
                6 + Self::encode_input_mode(in2, 2) + Self::encode_input_mode(in3, 3)
            }
            LessThan(in2, in3, out4) => {
                7 + Self::encode_input_mode(in2, 2)
                    + Self::encode_input_mode(in3, 3)
                    + Self::encode_output_mode(out4, 4)
            }
            Equals(in2, in3, out4) => {
                8 + Self::encode_input_mode(in2, 2)
                    + Self::encode_input_mode(in3, 3)
                    + Self::encode_output_mode(out4, 4)
            }
            RelativeMode(in2) => 9 + Self::encode_input_mode(in2, 2),
            Halt => 99,
        }
    }

    fn decode_input_mode(instruction: isize, position: u32) -> Result<InputParameter, ()> {
        let position = 10_isize.pow(position);
        let value = instruction / position % 10;
        match value {
            0 => Ok(InputParameter::Position),
            1 => Ok(InputParameter::Immediate),
            2 => Ok(InputParameter::Relative),
            _ => Err(()),
        }
    }

    fn decode_output_mode(instruction: isize, position: u32) -> Result<OutputParameter, ()> {
        let position = 10_isize.pow(position);
        let value = instruction / position % 10;
        match value {
            0 => Ok(OutputParameter::Position),
            2 => Ok(OutputParameter::Relative),
            _ => Err(()),
        }
    }

    fn encode_input_mode(mode: &InputParameter, position: u32) -> isize {
        10_isize.pow(position)
            * match mode {
                InputParameter::Position => 0,
                InputParameter::Immediate => 1,
                InputParameter::Relative => 2,
            }
    }

    fn encode_output_mode(mode: &OutputParameter, position: u32) -> isize {
        10_isize.pow(position)
            * match mode {
                OutputParameter::Position => 0,
                OutputParameter::Relative => 2,
            }
    }
}

/// The root processor object that runs the intcode
pub struct IntcodeProcess {
    memory: Vec<isize>,
    instruction_counter: usize,
    relative_base: isize,
    inputs: VecDeque<isize>,
    outputs: Vec<isize>,
}

impl IntcodeProcess {
    /// Create a new process with the given memory
    pub fn from_vec(memory: Vec<isize>) -> Self {
        IntcodeProcess {
            memory,
            instruction_counter: 0,
            relative_base: 0,
            inputs: VecDeque::new(),
            outputs: Vec::new(),
        }
    }

    /// Get the current instruction counter
    pub fn instruction_counter(&self) -> usize {
        self.instruction_counter
    }

    /// Get the current state of the memory
    pub fn memory(&self) -> &[isize] {
        &self.memory[..]
    }

    /// Get the current relative base
    pub fn relative_base(&self) -> isize {
        self.relative_base
    }

    /// Retrieve a value from memory at the given address
    pub fn load(&self, address: isize) -> Result<isize, IntcodeError> {
        if address < 0 {
            Err(IntcodeError::Segfault(address))?;
        }
        let address_u = address as usize;
        if address_u >= self.memory.len() {
            Err(IntcodeError::Segfault(address))?;
        }

        Ok(self.memory[address_u])
    }

    /// Retrieve a value from  memory at the given address, resizing the address space if necessary
    fn load_with_resize(&mut self, address: isize) -> Result<isize, IntcodeError> {
        if address < 0 {
            Err(IntcodeError::Segfault(address))?;
        }
        let address_u = address as usize;
        if address_u >= self.memory.len() {
            self.memory.resize(address_u + 1, 0);
        }

        Ok(self.memory[address_u])
    }

    /// Put a value into memory at the given address
    pub fn store(&mut self, address: isize, value: isize) -> Result<(), IntcodeError> {
        if address < 0 {
            Err(IntcodeError::Segfault(address))?;
        }
        let address_u = address as usize;
        if address_u >= self.memory.len() {
            Err(IntcodeError::Segfault(address))?;
        }

        self.memory[address_u] = value;
        Ok(())
    }

    /// Put a value into memory at the given address
    fn store_with_resize(&mut self, address: isize, value: isize) -> Result<(), IntcodeError> {
        if address < 0 {
            Err(IntcodeError::Segfault(address))?;
        }
        let address_u = address as usize;
        if address_u >= self.memory.len() {
            self.memory.resize(address_u + 1, 0);
        }

        self.memory[address_u] = value;
        Ok(())
    }

    /// Add a parameter to the input to be used by the input instruction
    pub fn add_input(&mut self, value: isize) {
        self.inputs.push_back(value);
    }

    /// Get a list of the outputs
    pub fn outputs(&self) -> &[isize] {
        &self.outputs[..]
    }

    /// Execute the next instruction
    ///
    /// If the command was an output, returns the value of the output. Otherwise returns nothing.
    /// This makes implementing `run_to_output` easier. It's not very generic, but not adding
    /// something generic until we need it.
    fn step(&mut self) -> Result<Option<isize>, IntcodeError> {
        let instruction = self.load_with_resize(self.instruction_counter as isize)?;
        let instruction_num = instruction;

        let instruction = Instruction::decode(instruction)
            .map_err(|_| IntcodeError::UnknownInstruction(instruction))?;

        match instruction {
            Instruction::Add(in0, in1, out) => self.add(in0, in1, out).map(|_| None),
            Instruction::Mul(in0, in1, out) => self.mul(in0, in1, out).map(|_| None),
            Instruction::Input(out) => self.input(out).map(|_| None),
            Instruction::Output(in0) => self.output(in0).map(|o| Some(o)),
            Instruction::JumpIfTrue(in0, in1) => self.jump_if_true(in0, in1).map(|_| None),
            Instruction::JumpIfFalse(in0, in1) => self.jump_if_false(in0, in1).map(|_| None),
            Instruction::LessThan(in0, in1, out) => self.less_than(in0, in1, out).map(|_| None),
            Instruction::Equals(in0, in1, out) => self.equals(in0, in1, out).map(|_| None),
            Instruction::RelativeMode(in0) => self.relative_mode(in0).map(|_| None),
            Instruction::Halt => self.halt().map(|_| None),
        }
    }

    /// Execute all remaining instructions until an error is reached
    pub fn run(&mut self) -> Result<(), IntcodeError> {
        loop {
            self.step()?;
        }
    }

    /// Execute instructions until we get an output
    pub fn run_to_output(&mut self) -> Result<isize, IntcodeError> {
        loop {
            let result = self.step()?;
            if let Some(output) = result {
                return Ok(output);
            }
        }
    }

    fn load_input(
        &mut self,
        mode: InputParameter,
        parameter_location: usize,
    ) -> Result<isize, IntcodeError> {
        let parameter = self.load_with_resize(parameter_location as isize)?;
        let val = match mode {
            InputParameter::Position => self.load_with_resize(parameter)?,
            InputParameter::Immediate => parameter,
            InputParameter::Relative => self.load_with_resize(parameter + self.relative_base)?,
        };
        Ok(val)
    }

    fn store_output(
        &mut self,
        mode: OutputParameter,
        parameter_location: usize,
        value: isize,
    ) -> Result<(), IntcodeError> {
        let parameter = self.load_with_resize(parameter_location as isize)?;
        match mode {
            OutputParameter::Position => self.store_with_resize(parameter, value)?,
            OutputParameter::Relative => {
                self.store_with_resize(parameter + self.relative_base, value)?
            }
        }

        Ok(())
    }

    fn add(
        &mut self,
        in0: InputParameter,
        in1: InputParameter,
        out: OutputParameter,
    ) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        let val1 = self.load_input(in1, self.instruction_counter + 2)?;
        self.store_output(out, self.instruction_counter + 3, val0 + val1)?;
        self.instruction_counter += 4;

        Ok(())
    }

    fn mul(
        &mut self,
        in0: InputParameter,
        in1: InputParameter,
        out: OutputParameter,
    ) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        let val1 = self.load_input(in1, self.instruction_counter + 2)?;
        self.store_output(out, self.instruction_counter + 3, val0 * val1)?;
        self.instruction_counter += 4;

        Ok(())
    }

    fn input(&mut self, out: OutputParameter) -> Result<(), IntcodeError> {
        let input = self
            .inputs
            .pop_front()
            .ok_or(IntcodeError::NoInputAvailable)?;
        self.store_output(out, self.instruction_counter + 1, input)?;
        self.instruction_counter += 2;

        Ok(())
    }

    fn output(&mut self, in0: InputParameter) -> Result<isize, IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        self.outputs.push(val0);
        self.instruction_counter += 2;

        Ok(val0)
    }

    fn jump_if_true(
        &mut self,
        in0: InputParameter,
        in1: InputParameter,
    ) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        let val1 = self.load_input(in1, self.instruction_counter + 2)?;
        if val0 != 0 {
            self.instruction_counter = val1 as usize;
        } else {
            self.instruction_counter += 3;
        }

        Ok(())
    }

    fn jump_if_false(
        &mut self,
        in0: InputParameter,
        in1: InputParameter,
    ) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        let val1 = self.load_input(in1, self.instruction_counter + 2)?;
        if val0 == 0 {
            self.instruction_counter = val1 as usize;
        } else {
            self.instruction_counter += 3;
        }

        Ok(())
    }

    fn less_than(
        &mut self,
        in0: InputParameter,
        in1: InputParameter,
        out: OutputParameter,
    ) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        let val1 = self.load_input(in1, self.instruction_counter + 2)?;
        let out_val = match val0 < val1 {
            true => 1,
            false => 0,
        };
        self.store_output(out, self.instruction_counter + 3, out_val)?;
        self.instruction_counter += 4;

        Ok(())
    }

    fn equals(
        &mut self,
        in0: InputParameter,
        in1: InputParameter,
        out: OutputParameter,
    ) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        let val1 = self.load_input(in1, self.instruction_counter + 2)?;
        let out_val = match val0 == val1 {
            true => 1,
            false => 0,
        };
        self.store_output(out, self.instruction_counter + 3, out_val)?;
        self.instruction_counter += 4;

        Ok(())
    }

    fn relative_mode(&mut self, in0: InputParameter) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        self.relative_base += val0;
        self.instruction_counter += 2;

        Ok(())
    }

    fn halt(&mut self) -> Result<(), IntcodeError> {
        Err(IntcodeError::CatchFire)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load() {
        let intcode = IntcodeProcess::from_vec(vec![0, 2, 4, 6, 8]);

        assert_eq!(intcode.load(0), Ok(0));
        assert_eq!(intcode.load(1), Ok(2));
        assert_eq!(intcode.load(4), Ok(8));
        assert_eq!(intcode.load(5), Err(IntcodeError::Segfault(5)));
        assert_eq!(intcode.load(-1), Err(IntcodeError::Segfault(-1)));
    }

    #[test]
    fn test_store() {
        let mut intcode = IntcodeProcess::from_vec(vec![0, 0, 0, 0, 0]);

        assert_eq!(intcode.store(0, 0), Ok(()));
        assert_eq!(intcode.load(0), Ok(0));
        assert_eq!(intcode.store(1, 2), Ok(()));
        assert_eq!(intcode.load(1), Ok(2));
        assert_eq!(intcode.store(4, 8), Ok(()));
        assert_eq!(intcode.load(4), Ok(8));
        assert_eq!(intcode.store(5, 10), Err(IntcodeError::Segfault(5)));
        assert_eq!(intcode.store(-1, -2), Err(IntcodeError::Segfault(-1)));
    }

    #[test]
    fn test_step() {
        let mut intcode = IntcodeProcess::from_vec(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(intcode.instruction_counter(), 0);

        assert_eq!(intcode.step(), Ok(None));
        assert_eq!(intcode.instruction_counter(), 4);
        assert_eq!(intcode.load(3), Ok(70));

        assert_eq!(intcode.step(), Ok(None));
        assert_eq!(intcode.instruction_counter(), 8);
        assert_eq!(intcode.load(0), Ok(3500));

        assert_eq!(intcode.step(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.instruction_counter(), 8);
    }

    #[test]
    fn test_run() {
        let mut intcode = IntcodeProcess::from_vec(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.instruction_counter(), 8);
        assert_eq!(intcode.load(3), Ok(70));
        assert_eq!(intcode.load(0), Ok(3500));
    }

    #[test]
    fn test_cases() {
        let mut intcode = IntcodeProcess::from_vec(vec![1, 0, 0, 0, 99]);
        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.memory(), &[2, 0, 0, 0, 99]);

        let mut intcode = IntcodeProcess::from_vec(vec![2, 3, 0, 3, 99]);
        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.memory(), &[2, 3, 0, 6, 99]);

        let mut intcode = IntcodeProcess::from_vec(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.memory(), &[2, 4, 4, 5, 99, 9801]);

        let mut intcode = IntcodeProcess::from_vec(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.memory(), &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_day_2() {
        let input = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 9, 19, 1, 19, 5, 23, 2, 23, 13,
            27, 1, 10, 27, 31, 2, 31, 6, 35, 1, 5, 35, 39, 1, 39, 10, 43, 2, 9, 43, 47, 1, 47, 5,
            51, 2, 51, 9, 55, 1, 13, 55, 59, 1, 13, 59, 63, 1, 6, 63, 67, 2, 13, 67, 71, 1, 10, 71,
            75, 2, 13, 75, 79, 1, 5, 79, 83, 2, 83, 9, 87, 2, 87, 13, 91, 1, 91, 5, 95, 2, 9, 95,
            99, 1, 99, 5, 103, 1, 2, 103, 107, 1, 10, 107, 0, 99, 2, 14, 0, 0,
        ];
        let mut processor = IntcodeProcess::from_vec(input.clone());
        processor.store(1, 12).unwrap();
        processor.store(2, 2).unwrap();
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(processor.load(0), Ok(3895705));

        let mut processor = IntcodeProcess::from_vec(input.clone());
        processor.store(1, 64).unwrap();
        processor.store(2, 17).unwrap();
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(processor.load(0), Ok(19690720));
    }

    #[test]
    fn test_input_output() {
        let input = vec![3, 5, 4, 5, 99, 0];
        let mut processor = IntcodeProcess::from_vec(input);
        processor.add_input(421);
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(processor.load(5), Ok(421));
        assert_eq!(processor.outputs(), &[421]);

        let input = vec![3, 5, 4, 5, 99, 0];
        let mut processor = IntcodeProcess::from_vec(input);
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::NoInputAvailable));

        let input = vec![3, 9, 4, 9, 3, 10, 4, 10, 99, 0, 0];
        let mut processor = IntcodeProcess::from_vec(input);
        processor.add_input(421);
        processor.add_input(500);
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(processor.load(9), Ok(421));
        assert_eq!(processor.load(10), Ok(500));
        assert_eq!(processor.outputs(), &[421, 500]);
    }

    #[test]
    fn test_immediate_mode() {
        let input = vec![1101, 10, 20, 5, 99, 0];
        let mut processor = IntcodeProcess::from_vec(input);
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(processor.load(5), Ok(30));
    }

    #[test]
    fn test_run_to_output() {
        let input = vec![
            Instruction::Output(InputParameter::Immediate).encode(),
            1,
            Instruction::Output(InputParameter::Immediate).encode(),
            2,
            Instruction::Output(InputParameter::Immediate).encode(),
            3,
            Instruction::Halt.encode(),
        ];

        let mut program = IntcodeProcess::from_vec(input);
        assert_eq!(program.run_to_output(), Ok(1));
        assert_eq!(program.run_to_output(), Ok(2));
        assert_eq!(program.run_to_output(), Ok(3));
        assert_eq!(program.run_to_output(), Err(IntcodeError::CatchFire));
    }

    #[test]
    fn test_run_to_output_example() {
        let input = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];

        let mut program_a = IntcodeProcess::from_vec(input.clone());
        program_a.add_input(9);
        program_a.add_input(0);
        let mut program_b = IntcodeProcess::from_vec(input.clone());
        program_b.add_input(8);
        let mut program_c = IntcodeProcess::from_vec(input.clone());
        program_c.add_input(7);
        let mut program_d = IntcodeProcess::from_vec(input.clone());
        program_d.add_input(6);
        let mut program_e = IntcodeProcess::from_vec(input.clone());
        program_e.add_input(5);

        let result_a = program_a.run_to_output();
        assert!(result_a.is_ok());
    }

    #[test]
    fn test_relative_mode_input() {
        let input = vec![
            Instruction::RelativeMode(InputParameter::Immediate).encode(),
            9,
            Instruction::Output(InputParameter::Relative).encode(),
            0,
            Instruction::Output(InputParameter::Relative).encode(),
            1,
            Instruction::Output(InputParameter::Relative).encode(),
            -1,
            Instruction::Halt.encode(),
            100,
            101,
        ];

        let mut program = IntcodeProcess::from_vec(input);
        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(program.outputs(), &[100, 101, 99]);
        assert_eq!(program.relative_base(), 9);
    }

    #[test]
    fn test_relative_mode_output() {
        let input = vec![
            Instruction::RelativeMode(InputParameter::Immediate).encode(),
            15,
            Instruction::Add(
                InputParameter::Immediate,
                InputParameter::Immediate,
                OutputParameter::Relative,
            )
            .encode(),
            1000,
            1,
            0,
            Instruction::Add(
                InputParameter::Immediate,
                InputParameter::Immediate,
                OutputParameter::Relative,
            )
            .encode(),
            1000,
            2,
            1,
            Instruction::Add(
                InputParameter::Immediate,
                InputParameter::Immediate,
                OutputParameter::Relative,
            )
            .encode(),
            1000,
            3,
            -8,
            Instruction::Halt.encode(),
            0,
            0,
        ];

        let mut program = IntcodeProcess::from_vec(input);
        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(program.load(15 + 0), Ok(1001));
        assert_eq!(program.load(15 + 1), Ok(1002));
        assert_eq!(program.load(15 - 8), Ok(1003));
        assert_eq!(program.relative_base(), 15);

        let input = vec![
            Instruction::RelativeMode(InputParameter::Immediate).encode(),
            9,
            Instruction::Input(OutputParameter::Relative).encode(),
            0,
            Instruction::Input(OutputParameter::Relative).encode(),
            1,
            Instruction::Input(OutputParameter::Relative).encode(),
            -4,
            Instruction::Halt.encode(),
            0,
            0,
        ];

        let mut program = IntcodeProcess::from_vec(input);
        program.add_input(2001);
        program.add_input(2002);
        program.add_input(2003);
        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(program.load(9 + 0), Ok(2001));
        assert_eq!(program.load(9 + 1), Ok(2002));
        assert_eq!(program.load(9 - 4), Ok(2003));
        assert_eq!(program.relative_base(), 9);
    }

    #[test]
    fn test_moving_relative_mode() {
        let input = vec![
            Instruction::RelativeMode(InputParameter::Immediate).encode(),
            13,
            Instruction::RelativeMode(InputParameter::Immediate).encode(),
            2, // Relative mode gets altered by this value, not set to this value. So it should be 15 now.
            Instruction::Output(InputParameter::Relative).encode(),
            0,
            Instruction::RelativeMode(InputParameter::Position).encode(),
            18, // Increase the relative base by the value of memory space 16
            Instruction::Output(InputParameter::Relative).encode(),
            0,
            Instruction::RelativeMode(InputParameter::Relative).encode(),
            3, // Current relative base should be 14. Set it to be the value of the memory address that's three more, which should be memory address 17 (value 15).
            Instruction::Output(InputParameter::Relative).encode(),
            -1,
            Instruction::Halt.encode(),
            1015,
            1016,
            1017,
            1,
            2,
        ];

        let mut program = IntcodeProcess::from_vec(input);
        assert_eq!(program.run_to_output(), Ok(1015));
        assert_eq!(program.relative_base(), 15);
        assert_eq!(program.run_to_output(), Ok(1016));
        assert_eq!(program.relative_base(), 16);
        assert_eq!(program.run_to_output(), Ok(1017));
        assert_eq!(program.relative_base(), 18);
        assert_eq!(program.run(), Err(IntcodeError::CatchFire));
    }

    fn test_quine() {
        // A test from day 9
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut program = IntcodeProcess::from_vec(input.clone());

        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));

        assert_eq!(program.outputs(), &input[..]);
    }

    #[test]
    fn test_day_large_mult() {
        // A test from day 9
        let input = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut program = IntcodeProcess::from_vec(input);

        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));

        assert_eq!(program.outputs(), &[1219070632396864]);
    }

    #[test]
    fn test_large_numbers() {
        // A test from day 9
        let input = vec![104, 1125899906842624, 99];

        let mut program = IntcodeProcess::from_vec(input);

        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));

        assert_eq!(program.outputs(), &[1125899906842624]);
    }

    #[test]
    fn test_extra_space() {
        let input = vec![
            Instruction::Output(InputParameter::Position).encode(),
            1000,
            Instruction::Halt.encode(),
        ];

        let mut program = IntcodeProcess::from_vec(input);

        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));

        assert_eq!(program.outputs(), &[0]);

        let input = vec![
            Instruction::Add(
                InputParameter::Immediate,
                InputParameter::Immediate,
                OutputParameter::Position,
            )
            .encode(),
            1,
            2,
            1000,
            Instruction::Output(InputParameter::Position).encode(),
            1000,
            Instruction::Halt.encode(),
        ];

        let mut program = IntcodeProcess::from_vec(input);

        let result = program.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));

        assert_eq!(program.outputs(), &[3]);
    }
}
