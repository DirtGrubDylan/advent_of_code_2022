#[derive(Debug, PartialEq, Clone)]
struct Ration {
    calories: u32,
}

impl Ration {
    fn new(calories: &str) -> Ration {
        Ration {
            calories: calories
                .parse()
                .expect(&format!("Could not parse: {}", calories)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Elf {
    rations: Vec<Ration>,
}

impl Elf {
    pub fn new(rations: &[String]) -> Elf {
        Elf {
            rations: rations
                .iter()
                .map(|calories| Ration::new(calories))
                .collect(),
        }
    }

    pub fn total_calories_from_rations(&self) -> u32 {
        self.rations
            .iter()
            .fold(0, |acc, ration| acc + ration.calories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATIONS: [Ration; 3] = [
        Ration { calories: 1_000 },
        Ration { calories: 2_000 },
        Ration { calories: 3_000 },
    ];

    #[test]
    fn test_ration_new() {
        let result = [
            Ration::new("1000"),
            Ration::new("2000"),
            Ration::new("3000"),
        ];

        assert_eq!(result, RATIONS)
    }

    #[test]
    fn test_elf_new() {
        let input = vec![
            String::from("1000"),
            String::from("2000"),
            String::from("3000"),
        ];

        let expected = Elf {
            rations: RATIONS.to_vec(),
        };

        let result = Elf::new(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_elf_total_calories_from_rations() {
        let elf = Elf {
            rations: RATIONS.to_vec(),
        };

        let expected = 6_000;

        let result = elf.total_calories_from_rations();

        assert_eq!(result, expected);
    }
}
