use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
pub struct SectionAssignments {
    ids: RangeInclusive<usize>,
}

impl SectionAssignments {
    pub fn contains(&self, other: &SectionAssignments) -> bool {
        self.ids.contains(other.ids.start()) && self.ids.contains(other.ids.end())
    }

    pub fn overlaps(&self, other: &SectionAssignments) -> bool {
        self.ids.contains(other.ids.start())
            || self.ids.contains(other.ids.end())
            || other.ids.contains(self.ids.start())
            || other.ids.contains(self.ids.end())
    }
}

impl From<&str> for SectionAssignments {
    fn from(value: &str) -> Self {
        let (start, end) = value
            .split_once('-')
            .map(|(s, e)| {
                (
                    s.parse::<usize>()
                        .expect(&format!("Failed to parse usize from: {}", s)),
                    e.parse::<usize>()
                        .expect(&format!("Failed to parse usize from: {}", e)),
                )
            })
            .expect(&format!("Could not parse: {}", value));

        SectionAssignments { ids: (start..=end) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let expected = SectionAssignments { ids: (6..=6) };

        let result = SectionAssignments::from("6-6");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_contains() {
        let first = SectionAssignments::from("2-8");
        let second = SectionAssignments::from("3-7");
        let third = SectionAssignments::from("1-1");

        assert!(first.contains(&second));
        assert!(!second.contains(&first));
        assert!(!first.contains(&third));
        assert!(!third.contains(&first));
    }

    #[test]
    fn test_overlaps() {
        let first = SectionAssignments::from("2-8");
        let second = SectionAssignments::from("3-7");
        let third = SectionAssignments::from("1-2");

        assert!(first.overlaps(&second));
        assert!(first.overlaps(&third));
        assert!(second.overlaps(&first));
        assert!(!second.overlaps(&third));
    }
}
