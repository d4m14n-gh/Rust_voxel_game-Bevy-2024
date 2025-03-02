use bevy::prelude::*;

use crate::world_generation::WorldStructure;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_SHIFT: i32 = 4;

pub const CHUNK_SIZE_2: usize = CHUNK_SIZE*CHUNK_SIZE;
pub const CHUNK_SIZE_3: usize = CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE;
pub const CHUNK_SHIFT_2: i32 = CHUNK_SHIFT*2;

pub struct Chunk{
    //chunk-representation
    pub world_chunk_position: (i32, i32, i32),  //top rigt front block
    pub blocks_table: [i16; CHUNK_SIZE_3],
    pub structure_table: Vec<WorldStructure>,
    //spawner-renderer meta
    pub entities_table: Vec<Entity>,
    pub spawned: bool,
    pub faces_calculated: bool,
    pub aot_calculated: bool,
    pub faces_table: [u8; CHUNK_SIZE_3],
    pub aot_table: [u8; CHUNK_SIZE_3],
    pub rects_table: [Vec<(i32, i32, i32, i32, i32)>; 6],
    //generator meta
    pub external_structure_table: Vec<WorldStructure>,
    pub structers_pre_generated: bool,
    pub structres_generated: bool,
    pub generated: bool,
}
impl Default for Chunk {
    fn default() -> Self {
        Chunk{
            world_chunk_position: (0, 0, 0),
            blocks_table: [0; CHUNK_SIZE_3],
            faces_table: [0; CHUNK_SIZE_3],
            aot_table: [0; CHUNK_SIZE_3],
            entities_table: Vec::new(),
            structure_table: Vec::new(),
            external_structure_table: Vec::new(),
            spawned: false,
            generated: false,
            structres_generated: false,
            structers_pre_generated: false,
            faces_calculated: false,
            aot_calculated: false,
            rects_table: [const {Vec::new()}; 6]
        }
    }
}
impl Chunk {
    pub fn from_position(position: (i32, i32, i32)) -> Self{
        Chunk{
            world_chunk_position: position,
            ..default()
        }
    }
    pub fn calculate_world_positon(&self, indeces: (i32, i32, i32)) -> (i32, i32, i32) {
        return ((self.world_chunk_position.0<<CHUNK_SHIFT)+indeces.0, (self.world_chunk_position.1<<CHUNK_SHIFT)+indeces.1, (self.world_chunk_position.2<<CHUNK_SHIFT)+indeces.2);
    }
}