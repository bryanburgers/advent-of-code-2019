use std::io::{self, Read};

struct DigitIterator<R> {
    read: R,
}

impl<R> DigitIterator<R>
where
    R: Read,
{
    fn new(read: R) -> Self {
        DigitIterator { read }
    }
}

impl<R> Iterator for DigitIterator<R>
where
    R: Read,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut n = [0];
            let result = self.read.read(&mut n);
            match result {
                Ok(0) => return None,
                Ok(1) => {
                    if n[0] == 10 {
                        continue;
                    } else if 0x30 <= n[0] && n[0] <= 0x39 {
                        return Some(n[0] - 0x30);
                    } else {
                        panic!("Unexpected character '{}'", n[0] as char);
                    }
                }
                Ok(size) => panic!("How in the world did we read {} bytes?", size),
                Err(e) => panic!("Unexpected error: {}", e),
            }
        }
    }
}

#[derive(Debug)]
struct Layer {
    width: usize,
    height: usize,
    items: Vec<u8>,
}

enum LayerError {
    NoInputAvailable,
    UnexpectedEndOfInput,
}

impl Layer {
    fn new(
        width: usize,
        height: usize,
        source: &mut impl Iterator<Item = u8>,
    ) -> Result<Layer, LayerError> {
        let mut items = Vec::with_capacity(width * height);
        for i in 0..width {
            for j in 0..height {
                match source.next() {
                    Some(next) => items.push(next),
                    None => {
                        if i == 0 && j == 0 {
                            Err(LayerError::NoInputAvailable)?
                        } else {
                            Err(LayerError::UnexpectedEndOfInput)?
                        }
                    }
                }
            }
        }
        Ok(Layer {
            width,
            height,
            items,
        })
    }

    fn count_digit(&self, digit: u8) -> usize {
        self.items.iter().filter(|&i| *i == digit).count()
    }
}

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    fn new(
        width: usize,
        height: usize,
        source: &mut impl Iterator<Item = u8>,
    ) -> Result<Image, String> {
        let mut layers = Vec::new();
        loop {
            match Layer::new(width, height, source) {
                Ok(layer) => layers.push(layer),
                Err(LayerError::NoInputAvailable) => break,
                Err(LayerError::UnexpectedEndOfInput) => {
                    Err("Unexpected end of input".to_string())?
                }
            }
        }

        Ok(Image {
            width,
            height,
            layers,
        })
    }

    fn rasterize(&self) -> RasterizedImage {
        let mut pixels = Vec::new();

        for _ in 0..self.width {
            for _ in 0..self.height {
                pixels.push(Pixel::Transparent);
            }
        }

        for layer in self.layers.iter() {
            for (idx, item) in layer.items.iter().enumerate() {
                pixels[idx] = pixels[idx] + (*item).into()
            }
        }

        RasterizedImage {
            width: self.width,
            height: self.height,
            pixels,
        }
    }
}

#[derive(Debug)]
struct RasterizedImage {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl std::fmt::Display for RasterizedImage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut iter = self.pixels.iter();
        for _ in 0..self.height {
            for _ in 0..self.width {
                write!(fmt, "{}", iter.next().unwrap())?;
            }
            write!(fmt, "\n")?;
        }

        assert_eq!(iter.next(), None);
        Ok(())
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Pixel {
    Transparent,
    Black,
    White,
}

impl From<u8> for Pixel {
    fn from(item: u8) -> Pixel {
        match item {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => Pixel::Transparent,
            i => panic!("Unexpected pixel value {}", i),
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::Transparent
    }
}

impl std::fmt::Debug for Pixel {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                Pixel::White => " ",
                Pixel::Black => "*",
                Pixel::Transparent => "?",
            }
        )
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                Pixel::Black => " ",
                Pixel::White => "*",
                Pixel::Transparent => "?",
            }
        )
    }
}

impl std::ops::Add for Pixel {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Pixel::Black => Pixel::Black,
            Pixel::White => Pixel::White,
            Pixel::Transparent => other,
        }
    }
}

fn main() {
    let mut digit_iterator = DigitIterator::new(io::stdin());

    let image = Image::new(25, 6, &mut digit_iterator).unwrap();
    assert_eq!(digit_iterator.next(), None);

    let mut fewest = 25 * 6 + 1;
    let mut focus_layer = None;
    for layer in image.layers.iter() {
        let zero_count = layer.count_digit(0);
        if zero_count < fewest {
            focus_layer = Some(layer);
            fewest = zero_count;
        }
    }

    let layer = focus_layer.expect("A minimum layer should have been found");
    let ones = layer.count_digit(1);
    let twos = layer.count_digit(2);
    println!("{}", ones * twos);

    let rasterized = image.rasterize();
    println!("{}", rasterized);
}
