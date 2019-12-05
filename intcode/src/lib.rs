pub struct IntcodeProcessor {
    memory: Vec<isize>,
    instruction_counter: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeError {
    UnknownInstruction(isize),
    CatchFire,
    Segfault(isize),
}

impl IntcodeProcessor {
    pub fn from_vec(memory: Vec<isize>) -> Self {
        IntcodeProcessor {
            memory,
            instruction_counter: 0,
        }
    }

    #[allow(dead_code)]
    pub fn ic(&self) -> usize {
        self.instruction_counter
    }

    #[allow(dead_code)]
    pub fn memory(&self) -> &[isize] {
        &self.memory[..]
    }

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

    pub fn step(&mut self) -> Result<(), IntcodeError> {
        let instruction = self.load(self.instruction_counter as isize)?;

        match instruction {
            1 => self.add(),
            2 => self.mul(),
            99 => Err(IntcodeError::CatchFire),
            _ => Err(IntcodeError::UnknownInstruction(instruction)),
        }
    }

    pub fn run(&mut self) -> Result<(), IntcodeError> {
        loop {
            self.step()?;
        }
    }

    fn add(&mut self) -> Result<(), IntcodeError> {
        let op1 = self.load((self.instruction_counter + 1) as isize)?;
        let op2 = self.load((self.instruction_counter + 2) as isize)?;
        let op3 = self.load((self.instruction_counter + 3) as isize)?;
        let val1 = self.load(op1)?;
        let val2 = self.load(op2)?;
        self.store(op3, val1 + val2)?;
        self.instruction_counter += 4;

        Ok(())
    }

    fn mul(&mut self) -> Result<(), IntcodeError> {
        let op1 = self.load((self.instruction_counter + 1) as isize)?;
        let op2 = self.load((self.instruction_counter + 2) as isize)?;
        let op3 = self.load((self.instruction_counter + 3) as isize)?;
        let val1 = self.load(op1)?;
        let val2 = self.load(op2)?;
        self.store(op3, val1 * val2)?;
        self.instruction_counter += 4;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load() {
        let intcode = IntcodeProcessor::from_vec(vec![0, 2, 4, 6, 8]);

        assert_eq!(intcode.load(0), Ok(0));
        assert_eq!(intcode.load(1), Ok(2));
        assert_eq!(intcode.load(4), Ok(8));
        assert_eq!(intcode.load(5), Err(IntcodeError::Segfault(5)));
        assert_eq!(intcode.load(-1), Err(IntcodeError::Segfault(-1)));
    }

    #[test]
    fn test_store() {
        let mut intcode = IntcodeProcessor::from_vec(vec![0, 0, 0, 0, 0]);

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
        let mut intcode =
            IntcodeProcessor::from_vec(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(intcode.ic(), 0);

        assert_eq!(intcode.step(), Ok(()));
        assert_eq!(intcode.ic(), 4);
        assert_eq!(intcode.load(3), Ok(70));

        assert_eq!(intcode.step(), Ok(()));
        assert_eq!(intcode.ic(), 8);
        assert_eq!(intcode.load(0), Ok(3500));

        assert_eq!(intcode.step(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.ic(), 8);
    }

    #[test]
    fn test_run() {
        let mut intcode =
            IntcodeProcessor::from_vec(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);

        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.ic(), 8);
        assert_eq!(intcode.load(3), Ok(70));
        assert_eq!(intcode.load(0), Ok(3500));
    }

    #[test]
    fn test_cases() {
        let mut intcode = IntcodeProcessor::from_vec(vec![1, 0, 0, 0, 99]);
        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.memory(), &[2, 0, 0, 0, 99]);

        let mut intcode = IntcodeProcessor::from_vec(vec![2, 3, 0, 3, 99]);
        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.memory(), &[2, 3, 0, 6, 99]);

        let mut intcode = IntcodeProcessor::from_vec(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(intcode.run(), Err(IntcodeError::CatchFire));
        assert_eq!(intcode.memory(), &[2, 4, 4, 5, 99, 9801]);

        let mut intcode = IntcodeProcessor::from_vec(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
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
        let mut processor = IntcodeProcessor::from_vec(input.clone());
        processor.store(1, 12).unwrap();
        processor.store(2, 2).unwrap();
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(processor.load(0), Ok(3895705));

        let mut processor = IntcodeProcessor::from_vec(input.clone());
        processor.store(1, 64).unwrap();
        processor.store(2, 17).unwrap();
        let result = processor.run();
        assert_eq!(result, Err(IntcodeError::CatchFire));
        assert_eq!(processor.load(0), Ok(19690720));
    }
}
