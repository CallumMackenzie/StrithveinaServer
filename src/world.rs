const WORLD_WIDTH: u32 = 2000;
const WORLD_LENGTH: u32 = 2000;
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
//do functions need to be pub to be tested?
pub fn get_data_from_image_cell(bits: u32) -> (u32, u32, u32) {
    let entity_spawn_info_mask: u32 = create_mask(ENTITY_SPAWN_INFO_BITS);
    let entity_spawn_info = bits & entity_spawn_info_mask;

    let tile_instance_mask: u32 = create_mask(TILE_INSTANCE_BITS);
    let tile_instance: u32 = (bits >> ENTITY_SPAWN_INFO_BITS + UNUSED_BITS) & tile_instance_mask;

    let tile_type_mask = create_mask(TILE_TYPE_BITS);
    let tile_type =
        (bits >> (ENTITY_SPAWN_INFO_BITS + UNUSED_BITS + TILE_INSTANCE_BITS)) & tile_type_mask;

    (tile_type, tile_instance, entity_spawn_info)
}
