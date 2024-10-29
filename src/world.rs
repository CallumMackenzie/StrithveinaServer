//base world size dimensions -> is necessary to determine image[] length

const TILE_TYPE_BITS: u32 = 7;
const TILE_INSTANCE_BITS: u32 = 18;
const UNUSED_BITS: u32 = 1;
const ENTITY_SPAWN_INFO_BITS: u32 = 6;
// format for image cell
// | tile type 7-bits | tile instance 18-bits | unused 1-bit | entity spawn info 6-bits| = 32 bits
//does it matter the size of the ints?

///Tile struct containing the mapped part of entities, tile types, and specific instances
/// data from the image[] is used to
struct Tile {}

struct World {
    width: u32,
    height: u32,
    // use vec or fixed array with constants at top?
    image: Vec<u32>,
    tiles: Vec<Tile>,
}

trait WorldGenerator {
    fn generateWorld(&self) -> World;
}

fn create_mask(bit_amt: u32) -> u32 {
    (1 << bit_amt) - 1
}

/**
 * Tile type used on const enum of all the tile types implemented
 * Tile instance goes to Tile[] for the specific created instances
 * EntitySpawnInfo is a collection of flags
 */

type TileTypeIndex = u32;
type TileInstanceIndex = u32;
type EntitySpawnInfoIndex = u32;
struct ImageCellData {
    tile_type_index: TileTypeIndex,
    tile_instance_index: TileInstanceIndex,
    entity_spawn_info_index: EntitySpawnInfoIndex,
}

//do functions need to be pub to be tested?
fn get_data_from_image_cell(bits: u32) -> ImageCellData {
    let entity_spawn_info_mask: u32 = create_mask(ENTITY_SPAWN_INFO_BITS);
    let entity_spawn_info = bits & entity_spawn_info_mask;

    let tile_instance_mask: u32 = create_mask(TILE_INSTANCE_BITS);
    let tile_instance: u32 = (bits >> ENTITY_SPAWN_INFO_BITS + UNUSED_BITS) & tile_instance_mask;

    let tile_type_mask = create_mask(TILE_TYPE_BITS);
    let tile_type =
        (bits >> (ENTITY_SPAWN_INFO_BITS + UNUSED_BITS + TILE_INSTANCE_BITS)) & tile_type_mask;

    ImageCellData {
        tile_type_index: (tile_type),
        tile_instance_index: (tile_instance),
        entity_spawn_info_index: (entity_spawn_info),
    }
}

#[test]
//contains optional println!'s for debugging
fn ensure_proper_bit_collection_from_image_cell() {
    let mut num: u32 = 0b10000011000000000000000010111111;
    let mut real_parta: u32 = 0b1000001;
    let mut real_partb: u32 = 0b100000000000000001;
    let mut real_partc: u32 = 0b111111;
    let ImageCellData {
        tile_type_index,
        tile_instance_index,
        entity_spawn_info_index,
    } = get_data_from_image_cell(num);

    // println!("Original number: {:32b}", num);
    // println!("Extracted 7-bit part: {:7b}", parta); // Print in binary
    // println!("Extracted 18-bit part: {:18b}", partb); // Print in binary
    // println!("Extracted 6-bit part: {:6b}", partc); // Print in binary

    assert_eq!(tile_type_index, real_parta);
    assert_eq!(tile_instance_index, real_partb);
    assert_eq!(entity_spawn_info_index, real_partc);

    num = !num; // 0b01111100111111111111111101000000
    real_parta = 0b0111110;
    real_partb = 0b011111111111111110;
    real_partc = 0b000000;
    let ImageCellData {
        tile_type_index,
        tile_instance_index,
        entity_spawn_info_index,
    } = get_data_from_image_cell(num);

    // println!("Original number: {:32b}", num);
    // println!(
    //     "Extracted 7-bit part: {:7b}, compared to: {:7b}",
    //     tile_type_index, real_parta
    // ); // Print in binary
    // println!(
    //     "Extracted 18-bit part: {:18b}, compared to: {:18b}",
    //     tile_instance_index, real_partb
    // ); // Print in binary
    // println!(
    //     "Extracted 6-bit part: {:6b}, compared to: {:6b}",
    //     entity_spawn_info_index, real_partc
    // ); // Print in binary

    assert_eq!(tile_type_index, real_parta);
    assert_eq!(tile_instance_index, real_partb as u32);
    assert_eq!(entity_spawn_info_index, real_partc as u32);
}
