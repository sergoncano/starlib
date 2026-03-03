use crate::model::coords::Coords;

pub struct Map {
    pub(crate) lines: Vec<String>,
}

impl Map {
    pub fn build(lines: Vec<&'static str>) -> Self {
        let res = Self {
            lines: lines.iter().map(|&s| String::from(s)).collect(),
        };
        res.is_rectangular();
        res
    }

    #[allow(dead_code)]
    pub(crate) fn test_map() -> Self {
        Self::build(vec!["..@", "..|", "o.."])
    }

    pub fn get_size(&self) -> Coords {
        let x = self.lines[0].len();
        let y = self.lines.len();
        Coords::new(x as i32, y as i32)
    }

    fn is_rectangular(&self) {
        if self.lines.is_empty() {
            panic!("Null maps aren't valid!");
        }
        let len = self.lines[0].len();
        for (i, line) in self.lines.iter().enumerate() {
            let line_len = line.len();
            if line_len != len {
                panic!(
                    "Map was not rectangular. Line {i} was expected to have {len} chars, but it had {line_len}."
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_builder() {
        Map::build(vec!["...", "..@", "..|"]);
    }

    #[test]
    #[should_panic]
    fn test_builder_fail() {
        Map::build(vec!["..", "..."]);
    }

    #[test]
    #[should_panic]
    fn test_builder_null_fail() {
        Map::build(vec![]);
    }
}
