#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CharBuffer {
    pub data: Vec<Vec<char>>,
    pub dimensions: (usize, usize),
}

impl CharBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        CharBuffer {
            data: vec![vec![' '; width]; height],
            dimensions: (width, height),
        }
    }
    pub fn get_char(&self, x: usize, y: usize) -> Option<char> {
        self.data.get(y)?.get(x).map(|x| *x)
    }
    pub fn set_char(&mut self, x: usize, y: usize, value: char) -> Result<(), &'static str> {
        let row = self.data.get_mut(y);
        if row.is_none() {
            return Err("Could not retrieve row, likely y out of range");
        }

        let item = row.unwrap().get_mut(x);
        if item.is_none() {
            return Err("Could not retrieve item, likely x out of range");
        }

        *item.unwrap() = value;

        Ok(())
    }
    pub fn fill(&mut self, char: char) {
        for row in self.data.iter_mut() {
            for item in row.iter_mut() {
                *item = char;
            }
        }
    }
}

impl std::fmt::Display for CharBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|x| [
                    x.iter().map(|x| [x, &' ']).flatten().collect::<String>(),
                    "\n".to_owned()
                ])
                .flatten()
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_retreival() {
        let mut buf = CharBuffer::new(3, 3);
        buf.set_char(2, 2, 'x').unwrap();
        assert_eq!(buf.get_char(2, 2).unwrap(), 'x');
    }

    #[test]
    #[should_panic]
    fn char_setter_bounds_check() {
        let mut buf = CharBuffer::new(2, 2);
        buf.set_char(2, 2, 'x').unwrap();
    }

    #[test]
    #[should_panic]
    fn char_getter_bounds_check() {
        let buf = CharBuffer::new(2, 2);
        buf.get_char(2, 2).unwrap();
    }

    #[test]
    fn char_buf_to_string() {
        let mut buf = CharBuffer::new(3, 3);
        buf.set_char(0, 0, 'n').unwrap();
        buf.set_char(2, 1, 'x').unwrap();
        buf.set_char(2, 2, 'z').unwrap();
        assert_eq!(&buf.to_string(), "n     \n    x \n    z \n");
    }
}
