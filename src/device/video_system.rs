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

#[derive(Debug, PartialEq)]
struct ClockCircuit {
    current_cycle: Cycle,
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
            Self::Add(value) => 2,
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
