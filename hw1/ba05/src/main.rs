use ba05::*;

fn main() {
    let image = [
        "..####..", ".#....#.", "#.#..#.#", "#..##..#", "#......#", "#.#..#.#", ".#....#.",
        "..####..",
    ];
    let bytes = parse_bitmap_8x8(image);
    println!("Bytes:");

    for byte in bytes {
        println!("{byte:08b} 0x{byte:02X}");
    }

    println!();
    println!("Rendered:");
    for line in render_bitmap_8x8(bytes) {
        println!("{line}");
    }

    println!();
    println!("Inverted:");
    for line in render_bitmap_8x8(invert_bitmap_8x8(bytes)) {
        println!("{line}");
    }

}