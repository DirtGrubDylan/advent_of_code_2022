use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Item {
    id: char,
    priority: u32,
}

impl Item {
    fn get_priority(id: char) -> u32 {
        let id_value = id as u32;
        let starting_value = if id_value < 97 { 26 } else { 0 };

        starting_value + (id_value % 32)
    }
}

impl From<char> for Item {
    fn from(id: char) -> Item {
        let priority = Item::get_priority(id);

        Item { id, priority }
    }
}

#[derive(Debug, PartialEq)]
pub struct RuckSack {
    items: Vec<Item>,
    number_of_compartments: usize,
}

impl RuckSack {
    pub fn new(input: &str) -> RuckSack {
        let items: Vec<Item> = input.chars().map(|id| Item::from(id)).collect();
        let number_of_compartments = 2;

        RuckSack {
            items,
            number_of_compartments,
        }
    }

    pub fn get_shared_item_priorities(sacks: &[RuckSack]) -> HashSet<u32> {
        RuckSack::get_shared_items(sacks)
            .into_iter()
            .map(|item| item.priority)
            .collect()
    }

    fn get_shared_items(sacks: &[RuckSack]) -> HashSet<Item> {
        let first_sack_items = sacks
            .get(0)
            .map_or(HashSet::new(), |sack| sack.items.iter().cloned().collect());

        sacks.iter().fold(first_sack_items, |acc, sack| {
            acc.intersection(&sack.items.iter().cloned().collect())
                .cloned()
                .collect()
        })
    }
}

impl RuckSack {
    pub fn get_missorted_item_priorities(&self) -> HashSet<u32> {
        self.get_missorted_items()
            .iter()
            .map(|item| item.priority)
            .collect()
    }

    fn get_missorted_items(&self) -> HashSet<Item> {
        let compartment_size = self.items.len() / self.number_of_compartments;

        let first_compartment_items = self
            .items
            .chunks(compartment_size)
            .nth(0)
            .map_or(HashSet::new(), |compartment| {
                compartment.iter().cloned().collect()
            });

        self.items
            .chunks(compartment_size)
            .fold(first_compartment_items, |acc, compartment| {
                acc.intersection(&HashSet::from_iter(compartment.iter().cloned()))
                    .cloned()
                    .collect()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_get_priority() {
        let input: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();

        let expected: Vec<u32> = (1..=52).collect();

        let result: Vec<u32> = input.iter().map(|id| Item::get_priority(*id)).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_missorted_items() {
        let sacks = vec![
            RuckSack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            RuckSack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            RuckSack::new("PmmdzqPrVvPwwTWBwg"),
            RuckSack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            RuckSack::new("ttgJtRGJQctTZtZT"),
            RuckSack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];

        let expected = vec![
            HashSet::from([Item::from('p')]),
            HashSet::from([Item::from('L')]),
            HashSet::from([Item::from('P')]),
            HashSet::from([Item::from('v')]),
            HashSet::from([Item::from('t')]),
            HashSet::from([Item::from('s')]),
        ];

        let result: Vec<HashSet<Item>> = sacks
            .into_iter()
            .map(|sack| sack.get_missorted_items())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_shared_items() {
        let sacks_groups = vec![
            vec![
                RuckSack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
                RuckSack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                RuckSack::new("PmmdzqPrVvPwwTWBwg"),
            ],
            vec![
                RuckSack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                RuckSack::new("ttgJtRGJQctTZtZT"),
                RuckSack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
            ],
        ];

        let expected = vec![
            HashSet::from([Item::from('r')]),
            HashSet::from([Item::from('Z')]),
        ];

        let result: Vec<HashSet<Item>> = sacks_groups
            .into_iter()
            .map(|sacks| RuckSack::get_shared_items(&sacks))
            .collect();

        assert_eq!(result, expected);
    }
}
