use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
enum CycleState {
    Starting,
    Executing,
    Ending,
}

#[derive(Debug, PartialEq)]
struct Cycle {
    tick: usize,
    state: CycleState,
}

impl Cycle {
    fn new() -> Cycle {
        Cycle {
            tick: 0,
            state: CycleState::Starting,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ClockCircuit {
    current_cycle: Cycle,
}

impl ClockCircuit {
    fn new() -> ClockCircuit {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
pub enum CpuInstruction {
    NoOp,
    Add(isize),
}

impl CpuInstruction {
    fn number_of_cycles_to_complete(&self) -> usize {
        match &self {
            Self::NoOp => 1,
            Self::Add(_) => 2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Cpu {
    registers: HashMap<char, isize>,
    instructions: VecDeque<CpuInstruction>,
    current_cycle: Cycle,
    current_instruction: CpuInstruction,
}

impl struct Cpu {
    fn new() -> Cpu {
        unimplemented!()
    }

    fn execute(&mut self) {
    }
}
