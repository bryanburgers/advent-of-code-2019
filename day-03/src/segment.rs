use super::command::Command;

pub type Point = (isize, isize);

#[derive(Debug, Eq, PartialEq)]
pub enum Segment {
    Vertical { x: isize, y0: isize, y1: isize },
    Horizontal { y: isize, x0: isize, x1: isize },
}

impl Segment {
    pub fn intersection(&self, other: &Self) -> Option<Point> {
        use Segment::*;
        match (self, other) {
            (Vertical { x, y0, y1 }, Horizontal { y, x0, x1 })
                if x0 < x && x < x1 && y0 < y && y < y1 =>
            {
                Some((*x, *y))
            }
            (Horizontal { y, x0, x1 }, Vertical { x, y0, y1 })
                if x0 < x && x < x1 && y0 < y && y < y1 =>
            {
                Some((*x, *y))
            }
            _ => None,
        }
    }
}

pub struct SegmentIter<I> {
    iterator: I,
    current_point: Point,
}

impl<I: Iterator<Item = Command>> SegmentIter<I> {
    #[allow(dead_code)]
    pub fn new(iterator: I) -> SegmentIter<I> {
        SegmentIter {
            iterator,
            current_point: (0, 0),
        }
    }
}

impl<I> Iterator for SegmentIter<I>
where
    I: Iterator<Item = Command>,
{
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        let direction = self.iterator.next()?;

        let previous_point = self.current_point;

        let (segment, next_point) = match direction {
            Command::Up(size) => (
                Segment::Vertical {
                    x: previous_point.0,
                    y0: previous_point.1,
                    y1: previous_point.1 + size,
                },
                (self.current_point.0, self.current_point.1 + size),
            ),
            Command::Down(size) => (
                Segment::Vertical {
                    x: previous_point.0,
                    y0: previous_point.1 - size,
                    y1: previous_point.1,
                },
                (self.current_point.0, self.current_point.1 - size),
            ),
            Command::Left(size) => (
                Segment::Horizontal {
                    y: previous_point.1,
                    x0: previous_point.0 - size,
                    x1: previous_point.0,
                },
                (self.current_point.0 - size, self.current_point.1),
            ),
            Command::Right(size) => (
                Segment::Horizontal {
                    y: previous_point.1,
                    x0: previous_point.0,
                    x1: previous_point.0 + size,
                },
                (self.current_point.0 + size, self.current_point.1),
            ),
        };

        self.current_point = next_point;
        Some(segment)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersect() {
        use Segment::*;

        let s1 = Vertical {
            x: 2,
            y0: -5,
            y1: 5,
        };
        let s2 = Horizontal {
            y: 1,
            x0: -5,
            x1: 5,
        };

        assert_eq!(s1.intersection(&s2), Some((2, 1)));
        assert_eq!(s2.intersection(&s1), Some((2, 1)));
    }

    #[test]
    fn test_iter() {
        use Command::*;
        use Segment::*;

        let vec = vec![Up(10), Right(10), Down(5), Left(5)];
        let mut iter = SegmentIter::new(vec.into_iter());

        assert_eq!(
            iter.next(),
            Some(Vertical {
                x: 0,
                y0: 0,
                y1: 10
            })
        );
        assert_eq!(
            iter.next(),
            Some(Horizontal {
                y: 10,
                x0: 0,
                x1: 10
            })
        );
        assert_eq!(
            iter.next(),
            Some(Vertical {
                x: 10,
                y0: 5,
                y1: 10
            })
        );
        assert_eq!(
            iter.next(),
            Some(Horizontal {
                y: 5,
                x0: 5,
                x1: 10
            })
        );
        assert_eq!(iter.next(), None);
    }
}
