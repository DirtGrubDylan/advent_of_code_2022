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

    fn next_stage(&self) -> ClockCircuit {
        unimplemented!()
    }

    fn next_cycle(&self) -> ClockCircuit {
        unimplemented!()
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

impl From<&String> for CpuInstruction {
    fn from(input: &String) -> CpuInstruction {
        unimplemented!()
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

    fn run(&mut self, cycle: &Cycle) -> isize {
        unimplemented!()
    }

    fn execute(&mut self, instruction: &CpuInstruction) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_next_stage() {
        unimplemented!()
    }

    #[test]
    fn test_cycle_next_tick() {
        unimplemented!()
    }

    #[test]
    fn test_clock_circuit_iter() {
        unimplemented!()
    }

    #[test]
    fn test_cpu_instruction_from() {
        unimplemented!()
    }
    #[test]
    fn test_cycle_next_stage() {
        unimplemented!()
    }

    #[test]
    fn test_cpu_execute_noop() {
        unimplemented!()
    }

    #[test]
    fn test_cpu_execute_add() {
        unimplemented!()
    }

    #[test]
    fn test_cpu_execute_run() {
        unimplemented!()
    }
}
