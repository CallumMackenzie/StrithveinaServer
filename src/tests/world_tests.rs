use crate::world::get_data_from_image_cell;

#[test]
fn a_world_test() {
    assert_eq!(1, 1);
}

#[test]
//contains optional println!'s for debugging
fn ensure_proper_bit_collection_from_image_cell() {
    let mut num: u32 = 0b10000011000000000000000010111111;
    let mut real_parta: u32 = 0b1000001;
    let mut real_partb: u32 = 0b100000000000000001;
    let mut real_partc: u32 = 0b111111;
    let (parta, partb, partc) = get_data_from_image_cell(num);

    // println!("Original number: {:32b}", num);
    // println!("Extracted 7-bit part: {:7b}", parta); // Print in binary
    // println!("Extracted 18-bit part: {:18b}", partb); // Print in binary
    // println!("Extracted 6-bit part: {:6b}", partc); // Print in binary

    assert_eq!(parta, real_parta);
    assert_eq!(partb, real_partb);
    assert_eq!(partc, real_partc);

    num = !num; // 0b01111100111111111111111101000000
    real_parta = 0b0111110;
    real_partb = 0b011111111111111110;
    real_partc = 0b000000;
    let (parta, partb, partc) = get_data_from_image_cell(num);

    println!("Original number: {:32b}", num);
    println!(
        "Extracted 7-bit part: {:7b}, compared to: {:7b}",
        parta, real_parta
    ); // Print in binary
    println!(
        "Extracted 18-bit part: {:18b}, compared to: {:18b}",
        partb, real_partb
    ); // Print in binary
    println!(
        "Extracted 6-bit part: {:6b}, compared to: {:6b}",
        partc, real_partc
    ); // Print in binary

    assert_eq!(parta, real_parta);
    assert_eq!(partb, real_partb as u32);
    assert_eq!(partc, real_partc as u32);
}
