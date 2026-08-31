// должна принять картинку как массив из 8 строк и вернуть массив из 8 байтов (c)
// Каждая строка превращается в один u8
// # - 1, . - 0
pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {

    let mut result: [u8; 8] = [0; 8];

    let mut i = 0;
    for bitmap_row in &lines {
        let mut item: u8 = 0b0000_0000;

        if bitmap_row.len() != 8 {
            panic!("parse error")
        }

        let mut n = 7;

        for b in bitmap_row.bytes() {
            match b {
                b'#' => item |= 1 << n,
                _ => {},
            }
            n -= 1;
        }

        result[i] = item;
        i += 1;
    }

    result
}


// обратная операция к parse_bitmap_8x8 (с)
pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
    let mut result = [const { String::new() }; 8];

    let mut i = 0;
    for row_byte in bytes {
        
        let mut chars = ['.'; 8];

        for n in (0..8).rev() {
            if row_byte & (1 << n) != 0 {
                chars[n] = '#';
            }
        }

        result[i] = String::from_iter(chars);
        i += 1;
    }

    result
}


// должна инвертировать картинку (с)
pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
    let mut result: [u8; 8] = [0; 8];

    let mut i = 0;

    for row in bytes {
        result[i] = !row;
        i += 1;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitmap() {
        let image = ["..####..", ".#....#.", "#.#..#.#", "#..##..#", "#......#", "#.#..#.#", ".#....#.", "..####.."];

        let left = parse_bitmap_8x8(image);
        let right = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b0011_1100,
        ];

        assert_eq!(left, right);
    }

    #[test]
    fn test_render_bitmap() {
        let bytes = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b0011_1100,
        ];
        let left = render_bitmap_8x8(bytes);
        let right = ["..####..", ".#....#.", "#.#..#.#", "#..##..#", "#......#", "#.#..#.#", ".#....#.", "..####.."];
        assert_eq!(left, right);
    }

     
    #[test]
    fn test_invert_bitmap() {
        let bytes = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b0011_1100,
        ];

        let left = invert_bitmap_8x8(bytes);

        let right = [
            0b1100_0011,
            0b1011_1101,
            0b0101_1010,
            0b0110_0110,
            0b0111_1110,
            0b0101_1010,
            0b1011_1101,
            0b1100_0011,
        ];
        assert_eq!(left, right);
    }
    
}
