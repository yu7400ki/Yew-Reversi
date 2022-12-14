pub struct Coordinate {
    position: u8,
}

impl Iterator for Coordinate {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position + 1;
        if position > Coordinate::MAX {
            None
        } else {
            self.position = position;
            Some(Coordinate::from(position))
        }
    }
}

impl Coordinate {
    const MIN: u8 = 0;
    const MAX: u8 = 63;

    pub fn from(position: u8) -> Coordinate {
        if position < Coordinate::MIN || position > Coordinate::MAX {
            panic!("Invalid position: {}", position);
        }
        Coordinate { position }
    }

    pub fn to_int(&self) -> u8 {
        self.position
    }

    pub fn to_mask(&self) -> u64 {
        1 << (63 - self.position)
    }
}
