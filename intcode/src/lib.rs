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
enum InputParameter {
    /// Position mode means the parameter refers to a location in the memory space
    Position,
    /// Immediate mode means the parameter refers to the value that should be used
    Immediate,
}

/// The type of the output parameter
enum OutputParameter {
    /// Position mode means the parameter refers to a location in the memory space
    Position,
}

enum Instruction {
    Add(InputParameter, InputParameter, OutputParameter),
    Mul(InputParameter, InputParameter, OutputParameter),
    Input(OutputParameter),
    Output(InputParameter),
    JumpIfTrue(InputParameter, InputParameter),
    JumpIfFalse(InputParameter, InputParameter),
    LessThan(InputParameter, InputParameter, OutputParameter),
    Equals(InputParameter, InputParameter, OutputParameter),
    Halt,
}

impl Instruction {
    pub fn decode(instruction: isize) -> Result<Self, ()> {
        let instruction = match instruction % 100 {
            1 => Instruction::Add(
                Self::input_mode(instruction, 2)?,
                Self::input_mode(instruction, 3)?,
                Self::output_mode(instruction, 4)?,
            ),
            2 => Instruction::Mul(
                Self::input_mode(instruction, 2)?,
                Self::input_mode(instruction, 3)?,
                Self::output_mode(instruction, 4)?,
            ),
            3 => Instruction::Input(Self::output_mode(instruction, 2)?),
            4 => Instruction::Output(Self::input_mode(instruction, 2)?),
            5 => Instruction::JumpIfTrue(
                Self::input_mode(instruction, 2)?,
                Self::input_mode(instruction, 3)?,
            ),
            6 => Instruction::JumpIfFalse(
                Self::input_mode(instruction, 2)?,
                Self::input_mode(instruction, 3)?,
            ),
            7 => Instruction::LessThan(
                Self::input_mode(instruction, 2)?,
                Self::input_mode(instruction, 3)?,
                Self::output_mode(instruction, 4)?,
            ),
            8 => Instruction::Equals(
                Self::input_mode(instruction, 2)?,
                Self::input_mode(instruction, 3)?,
                Self::output_mode(instruction, 4)?,
            ),
            99 => Instruction::Halt,
            _ => Err(())?,
        };

        Ok(instruction)
    }

    fn input_mode(instruction: isize, position: u32) -> Result<InputParameter, ()> {
        let position = 10_isize.pow(position);
        let value = instruction / position % 10;
        match value {
            0 => Ok(InputParameter::Position),
            1 => Ok(InputParameter::Immediate),
            _ => Err(()),
        }
    }

    fn output_mode(instruction: isize, position: u32) -> Result<OutputParameter, ()> {
        let position = 10_isize.pow(position);
        let value = instruction / position % 10;
        match value {
            0 => Ok(OutputParameter::Position),
            _ => Err(()),
        }
    }
}

/// The root processor object that runs the intcode
pub struct IntcodeProcess {
    memory: Vec<isize>,
    instruction_counter: usize,
    inputs: VecDeque<isize>,
    outputs: Vec<isize>,
}

impl IntcodeProcess {
    /// Create a new process with the given memory
    pub fn from_vec(memory: Vec<isize>) -> Self {
        IntcodeProcess {
            memory,
            instruction_counter: 0,
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

    /// Add a parameter to the input to be used by the input instruction
    pub fn add_input(&mut self, value: isize) {
        self.inputs.push_back(value);
    }

    /// Get a list of the outputs
    pub fn outputs(&self) -> &[isize] {
        &self.outputs[..]
    }

    /// Execute the next instruction
    pub fn step(&mut self) -> Result<(), IntcodeError> {
        let instruction = self.load(self.instruction_counter as isize)?;

        let instruction = Instruction::decode(instruction)
            .map_err(|_| IntcodeError::UnknownInstruction(instruction))?;

        match instruction {
            Instruction::Add(in0, in1, out) => self.add(in0, in1, out),
            Instruction::Mul(in0, in1, out) => self.mul(in0, in1, out),
            Instruction::Input(out) => self.input(out),
            Instruction::Output(in0) => self.output(in0),
            Instruction::JumpIfTrue(in0, in1) => self.jump_if_true(in0, in1),
            Instruction::JumpIfFalse(in0, in1) => self.jump_if_false(in0, in1),
            Instruction::LessThan(in0, in1, out) => self.less_than(in0, in1, out),
            Instruction::Equals(in0, in1, out) => self.equals(in0, in1, out),
            Instruction::Halt => self.halt(),
        }
    }

    /// Execute all remaining instructions until an error is reached
    pub fn run(&mut self) -> Result<(), IntcodeError> {
        loop {
            self.step()?;
        }
    }

    fn load_input(
        &mut self,
        mode: InputParameter,
        parameter_location: usize,
    ) -> Result<isize, IntcodeError> {
        let parameter = self.load(parameter_location as isize)?;
        let val = match mode {
            InputParameter::Position => self.load(parameter)?,
            InputParameter::Immediate => parameter,
        };
        Ok(val)
    }

    fn store_output(
        &mut self,
        mode: OutputParameter,
        parameter_location: usize,
        value: isize,
    ) -> Result<(), IntcodeError> {
        let parameter = self.load(parameter_location as isize)?;
        self.store(parameter, value)?;

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

    fn output(&mut self, in0: InputParameter) -> Result<(), IntcodeError> {
        let val0 = self.load_input(in0, self.instruction_counter + 1)?;
        self.outputs.push(val0);
        self.instruction_counter += 2;

        Ok(())
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

        assert_eq!(intcode.step(), Ok(()));
        assert_eq!(intcode.instruction_counter(), 4);
        assert_eq!(intcode.load(3), Ok(70));

        assert_eq!(intcode.step(), Ok(()));
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
}
