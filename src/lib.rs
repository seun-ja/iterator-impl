struct Split<'a> {
    pub delimiter: char,
    pub input: &'a str,
    pub index: usize
}

impl<'a> Split<'a> {
    fn new(delimiter: char, input: &'a str) -> Self {
        Self { delimiter, input, index: 0 }
    }
}

impl<'a> Iterator for Split<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let current = if let Some(item) = self.input.chars().nth(self.index) {
            item
        } else {
            return None
        };

        if current != self.delimiter {
            self.index += 1;
            return Some(current)
        } else {
            self.index += 1;
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iter() {
        let split = crate::Split::new('x', "opxtioxn");
        let res: String = split.into_iter()
            .collect();

        assert_eq!(res.as_str(), "option");
    }
}
