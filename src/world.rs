const WORLD_WIDTH: u32 = 2000;
const WORLD_LENGTH: u32 = 2000;
const TILE_TYPE_BITS: u32 = 7;
const TILE_INSTANCE_BITS: u32 = 18;
const UNUSED_BITS: u32 = 1;
const ENTITY_SPAWN_INFO_BITS: u32 = 6;
//does it matter the size of the ints?

struct Tile {}

struct World {
    width: u32,
    height: u32,
    // use vec or fixed array with constants at top?
    image: Vec<u32>,
    tiles: Vec<u32>,
}

trait WorldGenerator {
    fn generateWorld(&self) -> World;
}

fn create_mask(bit_amt: u32) -> u32 {
    (1 << bit_amt) - 1
}

fn getDataFromImageCell(bits: u32) -> (u32, u32, u32) {
    let tile_type_mask = create_mask(TILE_TYPE_BITS);
    let tile_type = (bits & tile_type_mask);

    let tile_instance_mask: u32 = create_mask(TILE_INSTANCE_BITS);
    let tile_instance: u32 = ((bits >> TILE_TYPE_BITS) & tile_instance_mask);

    let entity_spawn_info_mask: u32 = create_mask(ENTITY_SPAWN_INFO_BITS);
    let entity_spawn_info =
        (bits >> (TILE_TYPE_BITS + TILE_INSTANCE_BITS + UNUSED_BITS) & entity_spawn_info_mask);

    (tile_type, tile_instance, entity_spawn_info)
}
