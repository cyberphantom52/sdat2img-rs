pub struct Range((u64, u64));
impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Range((start, end))
    }

    pub fn inner(&self) -> (u64, u64) {
        self.0
    }

    pub fn start(&self) -> u64 {
        self.inner().0
    }

    pub fn end(&self) -> u64 {
        self.inner().1
    }

    pub fn len(&self) -> u64 {
        self.inner().1 - self.inner().0
    }
}

pub struct RangeSet {
    pairs: Vec<Range>,
}

pub struct Iter<'a> {
    pairs: &'a Vec<Range>,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Range;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.pairs.len() {
            let result = Some(&self.pairs[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl Iterator for RangeSet {
    type Item = Range;
    fn next(&mut self) -> Option<Self::Item> {
        self.pairs.pop()
    }
}

impl RangeSet {
    pub fn new(pairs: Vec<Range>) -> Self {
        RangeSet { pairs }
    }

    pub fn pairs(&self) -> &Vec<Range> {
        &self.pairs
    }

    pub fn iter(&self) -> Iter {
        Iter {
            pairs: &self.pairs,
            index: 0,
        }
    }
}

impl TryFrom<&str> for RangeSet {
    type Error = std::io::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        if tokens.len() < 3 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid range text.",
            ));
        }

        let num_pairs = tokens.remove(0);
        if num_pairs == 0 || num_pairs % 2 != 0 || num_pairs != tokens.len() as u64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid number of tokenss",
            ));
        }

        let mut pairs = Vec::new();
        tokens.chunks(2).for_each(|pair| {
            pairs.push(Range::new(pair[0], pair[1]));
        });

        Ok(RangeSet { pairs })
    }
}
