use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CycleState {
    Starting,
    Executing,
    Ending,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cycle {
    tick: usize,
    state: CycleState,
}

impl Cycle {
    fn new() -> Cycle {
        Cycle {
            tick: 1,
            state: CycleState::Starting,
        }
    }

    fn next_state(&self) -> Cycle {
        match &self.state {
            CycleState::Starting => Cycle {
                tick: self.tick,
                state: CycleState::Executing,
            },
            CycleState::Executing => Cycle {
                tick: self.tick,
                state: CycleState::Ending,
            },
            CycleState::Ending => Cycle {
                tick: self.tick + 1,
                state: CycleState::Starting,
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ClockCircuit {
    current_cycle: Cycle,
}

impl ClockCircuit {
    pub fn new() -> ClockCircuit {
        ClockCircuit {
            current_cycle: Cycle::new(),
        }
    }
}

impl Iterator for ClockCircuit {
    type Item = Cycle;

    fn next(&mut self) -> Option<Self::Item> {
        let old_state = self.current_cycle;

        self.current_cycle = old_state.next_state();

        Some(old_state)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CpuInstruction {
    NoOp,
    Add(i32),
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
        match input {
            s if s.starts_with("noop") => CpuInstruction::NoOp,
            s if s.starts_with("addx") => CpuInstruction::Add(
                s[5..]
                    .parse()
                    .expect(&format!("Bad addx instruction: {}", s)),
            ),
            _ => unimplemented!("Have not implemented: {}", input),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CPU {
    registers: HashMap<char, i32>,
    instructions: VecDeque<CpuInstruction>,
    current_instruction: Option<CpuInstruction>,
    ticks_left_for_current_instruction: usize,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: HashMap::from([('X', 1)]),
            instructions: VecDeque::new(),
            current_instruction: None,
            ticks_left_for_current_instruction: 0,
        }
    }

    pub fn get_register_value(&self, register: char) -> Option<i32> {
        self.registers.get(&register).cloned()
    }

    fn add_instructions(&mut self, instructions: &[CpuInstruction]) {
        self.instructions.extend(instructions.iter());
    }

    pub fn run(&mut self, cycle: &Cycle) -> i32 {
        match &cycle.state {
            CycleState::Starting => {
                if self.current_instruction.is_none() {
                    self.current_instruction = self.instructions.pop_front();

                    self.ticks_left_for_current_instruction = self
                        .current_instruction
                        .map_or(0, |instruction| instruction.number_of_cycles_to_complete());
                }
            }
            CycleState::Executing => (),
            CycleState::Ending => {
                self.ticks_left_for_current_instruction =
                    self.ticks_left_for_current_instruction.saturating_sub(1);

                let should_execute_instruction = self.current_instruction.is_some()
                    && (self.ticks_left_for_current_instruction == 0);

                if should_execute_instruction {
                    self.execute(&self.current_instruction.unwrap());

                    self.current_instruction = None;
                }
            }
        }

        self.get_register_value('X').unwrap()
    }

    fn execute(&mut self, instruction: &CpuInstruction) {
        match &instruction {
            CpuInstruction::Add(value) => {
                *self.registers.get_mut(&'X').unwrap() += value;
            }
            CpuInstruction::NoOp => (),
        }
    }
}

#[derive(Debug, PartialEq)]
struct CRT {
    display: Vec<Vec<char>>,
    current_col: usize,
    current_row: usize,
}

impl CRT {
    fn new() -> CRT {
        CRT {
            display: vec![vec!['.'; 40]; 6],
            current_col: 0,
            current_row: 0,
        }
    }

    fn print(&self) {
        println!("{}", ['='; 46].iter().collect::<String>());

        for row in self.display.iter() {
            println!(
                "|| {} ||",
                row.iter()
                    .map(|&c| if c == '.' { ' ' } else { c })
                    .collect::<String>()
            );
        }

        println!("{}", ['='; 46].iter().collect::<String>());
    }

    fn run(&mut self, cycle: &Cycle, sprite_center_location: i32) {
        self.current_row = (cycle.tick.saturating_sub(1) / 40) % 6;
        self.current_col = cycle.tick.saturating_sub(1) % 40;

        if cycle.state == CycleState::Executing {
            self.draw_sprite(sprite_center_location);
        }
    }

    fn draw_sprite(&mut self, sprite_center_location: i32) {
        let distance_to_center = sprite_center_location - (self.current_col as i32);

        if distance_to_center.abs() <= 1 {
            self.display[self.current_row][self.current_col] = '#';
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VideoSystem {
    clock: ClockCircuit,
    cpu: CPU,
    crt: CRT,
}

impl VideoSystem {
    pub fn new() -> VideoSystem {
        VideoSystem {
            clock: ClockCircuit::new(),
            cpu: CPU::new(),
            crt: CRT::new(),
        }
    }

    pub fn print_display(&self) {
        self.crt.print();
    }

    pub fn get_display(&self) -> Vec<Vec<char>> {
        self.crt.display.clone()
    }

    pub fn add_cpu_instructions(&mut self, input: &[String]) {
        let instructions: Vec<CpuInstruction> = input
            .iter()
            .map(|line| CpuInstruction::from(line))
            .collect();

        self.cpu.add_instructions(&instructions);
    }

    pub fn get_cpu_register_signal_strengths_at(
        &mut self,
        register: char,
        ticks: &HashSet<usize>,
    ) -> Vec<Option<i32>> {
        let mut result = Vec::new();
        let temp_clock = self.clock;

        for cycle in temp_clock {
            if self.cpu.instructions.is_empty() && self.cpu.current_instruction.is_none() {
                break;
            }

            self.cpu.run(&cycle);

            let sprite_center_location = self.cpu.get_register_value('X').unwrap_or(-2);

            self.crt.run(&cycle, sprite_center_location);

            if ticks.contains(&cycle.tick) && (cycle.state == CycleState::Executing) {
                result.push(
                    self.cpu
                        .get_register_value(register)
                        .map(|reg| reg * (cycle.tick as i32)),
                );
            }
        }

        self.clock = temp_clock;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_next_state() {
        let cycle = Cycle::new();

        let expected = Cycle {
            tick: 1,
            state: CycleState::Executing,
        };

        let result = cycle.next_state();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_clock_circuit_iter() {
        let clock = ClockCircuit::new();

        let expected = vec![
            Cycle {
                tick: 1,
                state: CycleState::Starting,
            },
            Cycle {
                tick: 1,
                state: CycleState::Executing,
            },
            Cycle {
                tick: 1,
                state: CycleState::Ending,
            },
            Cycle {
                tick: 2,
                state: CycleState::Starting,
            },
        ];

        let result: Vec<Cycle> = clock.take(4).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cpu_instruction_from() {
        let input = vec![
            String::from("noop"),
            String::from("addx 3"),
            String::from("addx -5"),
        ];

        let expected = vec![
            CpuInstruction::NoOp,
            CpuInstruction::Add(3),
            CpuInstruction::Add(-5),
        ];

        let result: Vec<CpuInstruction> = input
            .iter()
            .map(|line| CpuInstruction::from(line))
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_cpu_execute_noop() {
        let mut cpu = CPU::new();

        cpu.execute(&CpuInstruction::NoOp);

        let expected = CPU {
            registers: HashMap::from([('X', 1)]),
            instructions: VecDeque::new(),
            current_instruction: None,
            ticks_left_for_current_instruction: 0,
        };

        assert_eq!(cpu, expected);
    }

    #[test]
    fn test_cpu_execute_add() {
        let mut cpu = CPU::new();

        let expected = CPU {
            registers: HashMap::from([('X', -1)]),
            instructions: VecDeque::new(),
            current_instruction: None,
            ticks_left_for_current_instruction: 0,
        };

        cpu.execute(&CpuInstruction::Add(3));
        cpu.execute(&CpuInstruction::Add(-5));

        assert_eq!(cpu, expected);
    }

    #[test]
    fn test_cpu_execute_run_first_cycle_starting() {
        let input = vec![
            CpuInstruction::NoOp,
            CpuInstruction::Add(3),
            CpuInstruction::Add(-5),
        ];

        let mut cpu = CPU::new();
        let clock = ClockCircuit::new();

        cpu.add_instructions(&input);

        let expected = CPU {
            registers: HashMap::from([('X', 1)]),
            instructions: VecDeque::from([CpuInstruction::Add(3), CpuInstruction::Add(-5)]),
            current_instruction: Some(CpuInstruction::NoOp),
            ticks_left_for_current_instruction: 1,
        };

        clock.take(1).for_each(|cycle| {
            cpu.run(&cycle);
        });

        assert_eq!(cpu, expected);
    }

    #[test]
    fn test_cpu_execute_run_cycle_2_executing() {
        let input = vec![
            CpuInstruction::NoOp,
            CpuInstruction::Add(3),
            CpuInstruction::Add(-5),
        ];

        let mut cpu = CPU::new();
        let clock = ClockCircuit::new();

        cpu.add_instructions(&input);

        let expected = CPU {
            registers: HashMap::from([('X', 1)]),
            instructions: VecDeque::from([CpuInstruction::Add(-5)]),
            current_instruction: Some(CpuInstruction::Add(3)),
            ticks_left_for_current_instruction: 2,
        };

        clock.take(5).for_each(|cycle| {
            cpu.run(&cycle);
        });

        assert_eq!(cpu, expected);
    }

    #[test]
    fn test_cpu_execute_run_cycle_3_ending() {
        let input = vec![
            CpuInstruction::NoOp,
            CpuInstruction::Add(3),
            CpuInstruction::Add(-5),
        ];

        let mut cpu = CPU::new();
        let clock = ClockCircuit::new();

        cpu.add_instructions(&input);

        let expected = CPU {
            registers: HashMap::from([('X', 4)]),
            instructions: VecDeque::from([CpuInstruction::Add(-5)]),
            current_instruction: None,
            ticks_left_for_current_instruction: 0,
        };

        clock.take(9).for_each(|cycle| {
            cpu.run(&cycle);
        });

        assert_eq!(cpu, expected);
    }

    #[test]
    fn test_cpu_execute_run_first_cycle_4_ending() {
        let input = vec![
            CpuInstruction::NoOp,
            CpuInstruction::Add(3),
            CpuInstruction::Add(-5),
        ];

        let mut cpu = CPU::new();
        let clock = ClockCircuit::new();

        cpu.add_instructions(&input);

        let expected = CPU {
            registers: HashMap::from([('X', 4)]),
            instructions: VecDeque::new(),
            current_instruction: Some(CpuInstruction::Add(-5)),
            ticks_left_for_current_instruction: 1,
        };

        clock.take(12).for_each(|cycle| {
            println!("{:?}", cycle);
            cpu.run(&cycle);
        });

        assert_eq!(cpu, expected);
    }

    #[test]
    fn test_cpu_execute_run_all_cycles() {
        let input = vec![
            CpuInstruction::NoOp,
            CpuInstruction::Add(3),
            CpuInstruction::Add(-5),
        ];

        let mut cpu = CPU::new();
        let clock = ClockCircuit::new();

        cpu.add_instructions(&input);

        let expected = CPU {
            registers: HashMap::from([('X', -1)]),
            instructions: VecDeque::new(),
            current_instruction: None,
            ticks_left_for_current_instruction: 0,
        };

        for cycle in clock {
            if cpu.instructions.is_empty() && cpu.current_instruction.is_none() {
                break;
            }

            cpu.run(&cycle);
        }

        assert_eq!(cpu, expected);
    }

    #[test]
    fn test_crt_draw_sprite_left_in_bounds() {
        let sprite_center = 1;

        let mut crt = CRT::new();

        let expected: Vec<char> = "#.......................................".chars().collect();

        crt.draw_sprite(sprite_center);

        assert_eq!(crt.display[0], expected);
    }

    #[test]
    fn test_crt_draw_sprite_center_in_bounds() {
        let sprite_center = 0;

        let mut crt = CRT::new();

        let expected: Vec<char> = "#.......................................".chars().collect();

        crt.draw_sprite(sprite_center);

        assert_eq!(crt.display[0], expected);
    }

    #[test]
    fn test_crt_draw_sprite_right_in_bounds() {
        let sprite_center = -1;

        let mut crt = CRT::new();

        let expected: Vec<char> = "#.......................................".chars().collect();

        crt.draw_sprite(sprite_center);

        assert_eq!(crt.display[0], expected);
    }

    #[test]
    fn test_crt_draw_sprite_left_out_of_bounds() {
        let sprite_center = -2;

        let mut crt = CRT::new();

        let expected: Vec<char> = "........................................".chars().collect();

        crt.draw_sprite(sprite_center);

        assert_eq!(crt.display[0], expected);
    }

    #[test]
    fn test_crt_draw_sprite_right_out_of_bounds() {
        let sprite_center = 2;

        let mut crt = CRT::new();

        let expected: Vec<char> = "........................................".chars().collect();

        crt.draw_sprite(sprite_center);

        assert_eq!(crt.display[0], expected);
    }

    #[test]
    fn test_crt_execute_run_cycle_3_ending() {
        let mut crt = CRT::new();
        let clock = ClockCircuit::new();

        let expected = CRT {
            display: vec![vec!['.'; 40]; 6],
            current_col: 2,
            current_row: 0,
        };

        clock.take(9).for_each(|cycle| {
            crt.run(&cycle, -2);
        });

        assert_eq!(crt, expected);
    }

    #[test]
    fn test_crt_execute_run_cycle_240_ending() {
        let mut crt = CRT::new();
        let clock = ClockCircuit::new();

        let expected = CRT {
            display: vec![vec!['.'; 40]; 6],
            current_col: 39,
            current_row: 5,
        };

        clock.take(720).for_each(|cycle| {
            crt.run(&cycle, -2);
        });

        assert_eq!(crt, expected);
    }
}
