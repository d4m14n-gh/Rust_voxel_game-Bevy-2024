use bevy::prelude::*;
use rand::Rng;
use noise::{NoiseFn, Perlin};
use hashbrown::{HashMap, HashSet};
use crate::chunk::*;//{Chunk, CHUNK_SIZE, CHUNK_SIZE_2, CHUNK_SIZE_3, CHUNK_SHIFT, CHUNK_SHIFT_2};


pub trait I32x3Extension {
    fn add(self, other: (i32, i32, i32)) -> (i32, i32, i32);
    fn sub(self, other: (i32, i32, i32)) -> (i32, i32, i32);
    fn times(self, multiplier: i32) -> (i32, i32, i32);
    fn swap_axes(self, axis: i32) -> (i32, i32, i32);
    fn product(self, other: (i32, i32, i32)) -> (i32, i32, i32);
    fn divide(self, divider: i32) -> (i32, i32, i32);
    fn pos_modulo(self, divider: i32) -> (i32, i32, i32);
}
impl I32x3Extension for (i32, i32, i32) {
    fn add(self, other: (i32, i32, i32)) -> (i32, i32, i32){
        return (self.0+other.0, self.1+other.1, self.2+other.2);
    }
    fn sub(self, other: (i32, i32, i32)) -> (i32, i32, i32){
        return (self.0-other.0, self.1-other.1, self.2-other.2);
    }
    fn product(self, other: (i32, i32, i32)) -> (i32, i32, i32){
        return (self.0*other.0, self.1*other.1, self.2*other.2);
    }
    fn times(self, multiplier: i32) -> (i32, i32, i32){
        return (self.0*multiplier, self.1*multiplier, self.2*multiplier);
    }
    fn divide(self, divider: i32) -> (i32, i32, i32){
        let divider = divider as f32;
        return ((self.0 as f32/divider).floor() as i32, (self.1 as f32/divider).floor() as i32, (self.2 as f32/divider).floor() as i32);
    }
    fn pos_modulo(self, divider: i32) -> (i32, i32, i32){
        let mut ret = (self.0%divider, self.1%divider, self.2%divider);
        if ret.0<0 {
            ret.0 += divider;
        }
        if ret.1<0 {
            ret.1 += divider;
        }
        if ret.2<0 {
            ret.2 += divider;
        }
        return ret;
    }
    fn swap_axes(self, axis: i32) -> (i32, i32, i32) {
        if axis == 0{
            return (self.0, self.2, self.1);
        }
        else if axis == 1{
            return (self.2, self.1, self.0);
        }
        else if axis == 2{
            return (self.1, self.0, self.2);
        }
        else{
            return self;
        }
    }
}

#[derive(Clone)]
pub struct WorldStructure{
    root: (i32, i32, i32),
    root_chunk_position: (i32, i32, i32),
    blocks: Vec<((i32, i32, i32), i16)>,
    miroor_multiplier: (i32, i32, i32),
    mirror_axis: i32
}
impl Default for WorldStructure{
    fn default() -> Self {
        WorldStructure{
            root: (0, 0, 0),
            root_chunk_position: (0, 0, 0),
            blocks: Vec::new(),
            miroor_multiplier: (1, 1, 1),
            mirror_axis: -1
        }

    }
}
impl WorldStructure{
    pub fn tree(root: (i32, i32, i32), root_chunk: (i32, i32, i32)) -> Self{
        let blocks = vec![((0, 0, 0), 7), ((0, 1, 0), 7), ((0, 2, 0), 7), ((0, 3, 0), 7),
         ((1, 3, 0), 8), ((-1, 3, 0), 8), ((0, 3, 1), 8), ((0, 3, -1), 8), ((1, 3, 1), 8), ((1, 3, -1), 8), ((-1, 3, 1), 8), ((-1, 3, -1), 8),
         ((1, 4, 0), 8), ((-1, 4, 0), 8), ((0, 4, 1), 8), ((0, 4, -1), 8), ((1, 4, 1), 8), ((1, 4, -1), 8), ((-1, 4, 1), 8), ((-1, 4, -1), 8), ((0, 4, 0), 8),
         ((1, 5, 0), 8), ((-1, 5, 0), 8), ((0, 5, 1), 8), ((0, 5, -1), 8), ((0, 5, 0), 8),
        ];
        return WorldStructure{
            root: root,
            root_chunk_position: root_chunk,
            blocks: blocks,
            ..default()
        };
    }
    pub fn brich_tree(root: (i32, i32, i32), root_chunk: (i32, i32, i32)) -> Self{
        let mut tree = WorldStructure::tree(root, root_chunk);
        for block in tree.blocks.iter_mut(){
            block.1 = block.1+3;
        }
        return tree;
    }
    pub fn dark_oak_tree(root: (i32, i32, i32), root_chunk: (i32, i32, i32)) -> Self{
        let mut tree = WorldStructure::tree(root, root_chunk);
        for block in tree.blocks.iter_mut(){
            block.1 = block.1+5;
        }
        return tree;
    }
    pub fn big_tree(root: (i32, i32, i32), root_chunk: (i32, i32, i32)) -> Self{
        let blocks = vec![((0, 0, 0), 7), ((0, 1, 0), 7), ((0, 2, 0), 7), ((0, 3, 0), 7), ((0, 4, 0), 7), ((0, 5, 0), 7),
         ((1, 5, 0), 8), ((-1, 5, 0), 8), ((0, 5, 1), 8), ((0, 5, -1), 8), ((1, 5, 1), 8), ((1, 5, -1), 8), ((-1, 5, 1), 8), ((-1, 5, -1), 8),
         ((1, 6, 0), 8), ((-1, 6, 0), 8), ((0, 6, 1), 8), ((0, 6, -1), 8), ((1, 6, 1), 8), ((1, 6, -1), 8), ((-1, 6, 1), 8), ((-1, 6, -1), 8), ((0, 6, 0), 8),
         ((1, 7, 0), 8), ((-1, 7, 0), 8), ((0, 7, 1), 8), ((0, 7, -1), 8), ((0, 7, 0), 8), ((1, 7, -1), 8), ((-1, 7, 1), 8), ((-1, 7, -1), 8), ((-1, 7, -1), 8),
         ((1, 8, 0), 8), ((-1, 8, 0), 8), ((0, 8, 1), 8), ((0, 8, -1), 8), ((0, 8, 0), 8),
        ];
        return WorldStructure{
            root: root,
            root_chunk_position: root_chunk,
            blocks: blocks,
            ..default()
        };
    }
    pub fn bush(root: (i32, i32, i32), root_chunk: (i32, i32, i32)) -> Self{
        let blocks = vec![
         ((1, 0, 0), 8), ((0, 0, 1), 8), ((1, 0, 1), 8), ((0, 0, 0), 8),
         ((1, 1, 0), 8), ((0, 1, 1), 8), ((1, 1, 1), 8),
        ];
        return WorldStructure{
            root: root,
            root_chunk_position: root_chunk,
            blocks: blocks,
            ..default()
        };
    }
    pub fn small_bush(root: (i32, i32, i32), root_chunk: (i32, i32, i32)) -> Self{
        let blocks = vec![
         ((0, 0, 0), 8),
         ((0, 1, 0), 8)
        ];
        return WorldStructure{
            root: root,
            root_chunk_position: root_chunk,
            blocks: blocks,
            ..default()
        };
    }
    pub fn logs(root: (i32, i32, i32), root_chunk: (i32, i32, i32)) -> Self{
        let blocks = vec![
         ((1, 1, 0), 7), ((0, 1, 0), 7), ((-1, 1, 0), 7),
        ];
        return WorldStructure{
            root: root,
            root_chunk_position: root_chunk,
            blocks: blocks,
            ..default()
        };
    }
    pub fn indexed_structure(root: (i32, i32, i32), root_chunk: (i32, i32, i32), index: i32) -> Self{
        if index==0{
            return WorldStructure::tree(root, root_chunk);
        }
        else if index==1{
            return WorldStructure::big_tree(root, root_chunk);
        }
        else if index==2{
            return WorldStructure::tree(root, root_chunk);
        }
        else if index==3{
            return WorldStructure::small_bush(root, root_chunk);
        }
        else if index==4{
            return WorldStructure::brich_tree(root, root_chunk);
        }
        else if index==5{
            return WorldStructure::logs(root, root_chunk);
        }
        else if index==6{
            return WorldStructure::dark_oak_tree(root, root_chunk);
        }
        else{   
            return WorldStructure::bush (root, root_chunk);
        }
    }
    pub fn get_indexed_length() -> i32{
        return 8;
    }
}

//8x8x8 blocks chunk -> (2^3)^3=512 x*64+y*8+z  

#[derive(Resource)]
pub struct ChunkTable{
    pub player_position: (i32, i32, i32),
    pub prev_player_position: (i32, i32, i32),
    pub chunk_map: HashMap<(i32, i32, i32), Chunk>,
    pub rendered_chunk_set: HashSet<(i32, i32, i32)>
}
impl Default for ChunkTable {
    fn default() -> Self {
        ChunkTable{
            chunk_map: HashMap::new(),
            player_position: (0, 0, 0),
            prev_player_position: (0, 2137, 0),
            rendered_chunk_set: HashSet::new()
        }
    }
}
impl ChunkTable{
    pub fn add_chunk(&mut self, position: (i32, i32, i32)){
        if !self.has_chunk(position) {
            self.chunk_map.insert(position, Chunk::from_position(position));
        }
    }
    pub fn get_add_chunk(&mut self, position: (i32, i32, i32)) -> &Chunk{
        if self.chunk_map.contains_key(&position){
            return self.chunk_map.get(&position).unwrap();
        }
        self.chunk_map.insert(position, Chunk::from_position(position));
        return self.chunk_map.get(&position).unwrap();
    }
    pub fn get_add_chunk_mut(&mut self, position: (i32, i32, i32)) -> &mut Chunk{
        if self.chunk_map.contains_key(&position){
            return self.chunk_map.get_mut(&position).unwrap();
        }
        self.chunk_map.insert(position, Chunk::from_position(position));
        return self.chunk_map.get_mut(&position).unwrap();
    }
    pub fn get_chunk(&self, position: (i32, i32, i32)) -> Option<&Chunk>{
        return self.chunk_map.get(&position);
    }
    pub fn get_chunk_mut(&mut self, position: (i32, i32, i32)) -> Option<&mut Chunk>{
        return self.chunk_map.get_mut(&position);
    }
    pub fn has_chunk(&self, position: (i32, i32, i32)) -> bool{
        return self.chunk_map.contains_key(&position);
    }
    pub fn update_rendered_chunks(&mut self, commands: &mut Commands, center: (i32, i32, i32), distance: i32) -> Vec<(i32, i32, i32)>{
        let mut to_delete_position_set: Vec<(i32, i32, i32)> = Vec::new();
        for point in self.rendered_chunk_set.iter(){
            if (point.0-center.0)*(point.0-center.0) + (point.1-center.1)*(point.1-center.1) + (point.2-center.2)*(point.2-center.2) > distance*distance{
                to_delete_position_set.push(*point);
            }
        }
        for point in to_delete_position_set.iter(){
            let chunk = self.get_chunk_mut(*point).unwrap();
            
            for entity in chunk.entities_table.iter(){
                if let Some(mut existing_entity) = commands.get_entity(*entity){
                    existing_entity.despawn();//despawn();
                }
            }

            chunk.spawned = false;
            self.rendered_chunk_set.remove(point);
        }
        return to_delete_position_set;
    }
    fn get_block(&self, root_chunk: (i32, i32, i32), offset: (i32, i32, i32)) -> i16{
        let chunk_offset = offset.divide(CHUNK_SIZE as i32);
        let new_offset = offset.pos_modulo(CHUNK_SIZE as i32);
        let index = new_offset.0*(CHUNK_SIZE_2 as i32)+new_offset.1*(CHUNK_SIZE as i32)+new_offset.2;
        return self.chunk_map.get(&root_chunk.add(chunk_offset)).unwrap().blocks_table[index as usize];
    }
}


#[derive(Resource)]
pub struct WorldGenerator{
    seed: i32,
    perlin: Perlin,
    perlin2: Perlin,
    perlin3: Perlin,
}
impl Default for WorldGenerator{
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        WorldGenerator{
            seed: 2137,
            perlin: Perlin::new(rng.gen()),
            perlin2: Perlin::new(rng.gen()),
            perlin3: Perlin::new(rng.gen()),
        }
    }
}
impl WorldGenerator{
    const WORLD_GRASS_LEVEL: i32 = 3;
    const WORLD_WATER_LEVEL: i32 = -6;
    const MAX_DISTANCE: i32 = 250;
    
    pub fn get_perlin(&self, x: i32, z: i32, f: f64) -> f32{
        let noise_value: f32 = (self.perlin3).get([x as f64 * f, z as f64 * f]) as f32;
        return noise_value;
    }

    fn get_elevation(&self, x: i32, z:i32)->i32{
        let frequency: f64 = 0.015;
        let frequency2: f64 = 0.15;
        let frequency3: f64 = 0.5;
        let frequency4: f64 = 0.003;
        let noise_value: f32 = (self.perlin).get([x as f64 * frequency, z as f64 * frequency]) as f32;
        let noise_value2: f32 = (self.perlin2).get([x as f64 * frequency2, z as f64 * frequency2]) as f32;
        let noise_value3: f32 = (self.perlin2).get([x as f64 * frequency3, z as f64 * frequency3]) as f32;
        let noise_value4: f32 = (self.perlin2).get([x as f64 * frequency4, z as f64 * frequency4]) as f32;
        //let elevation =  (25.*noise_value+3.5*noise_value2).round() as i32;
        //let elevation =  (10.*noise_value+0.5*noise_value2+0.15*noise_value3).round() as i32;
        let elevation =  (15.*noise_value+2.5*noise_value2+0.5*noise_value3).round() as i32+10;
        let elevation =  (10.*noise_value+1.25*noise_value2+0.25*noise_value3).round().max((25.*noise_value+2.5*noise_value2+0.5*noise_value3).round()) as i32;
        //let elevation =  (140.*noise_value4+(10.*noise_value+1.25*noise_value2+0.25*noise_value3).round().max((25.*noise_value+2.5*noise_value2+0.5*noise_value3)).round()) as i32;
        return elevation;
    }
      
    fn get_distance(&self, x: i32, z:i32)->i32{
        return ((x*x+z*z) as f32).sqrt() as i32;
    }

    fn get_distance2(&self, x: i32, z:i32, y:i32)->i32{
        return ((x*x+z*z+y*y) as f32).sqrt() as i32;
    }
    
    fn pre_generate_structures(&self, chunk_table: &mut ChunkTable, chunk_position: (i32, i32, i32)){
        let chunk = chunk_table.get_chunk_mut(chunk_position).unwrap();
        if chunk.structers_pre_generated {return;}
        chunk.structers_pre_generated=true;
        
        let mut rng = rand::thread_rng();
        let cnt = ((rng.gen::<i32>().abs()%5 +((chunk_position.1+2)*3).abs()) as f32 *CHUNK_SIZE as f32 / 8.) as i32;
        for _ in 0..cnt{
            let x = rng.gen::<i32>().abs()%CHUNK_SIZE as i32;
            let z = rng.gen::<i32>().abs()%CHUNK_SIZE as i32;
            let (xw, yw, zw) = chunk.calculate_world_positon((x, 0, z));
            let elevation =  self.get_elevation(xw, zw);
            let distance = self.get_distance(xw, zw);
            
            let index = rng.gen::<i32>().abs()%WorldStructure::get_indexed_length();
            let x_miroor = (rng.gen::<i32>().abs()%2)*2-1;
            let z_miroor = (rng.gen::<i32>().abs()%2)*2-1;
            let mirror = (x_miroor, 1, z_miroor);
            let swap_axis = if rng.gen::<i32>().abs()%100<50 {1} else {-1};
            
            if elevation>=yw&&elevation<yw+CHUNK_SIZE as i32 && elevation >= WorldGenerator::WORLD_GRASS_LEVEL && distance < WorldGenerator::MAX_DISTANCE {
                let mut structure = WorldStructure::indexed_structure((x, elevation-yw, z), chunk_position, index);
                structure.miroor_multiplier = mirror;
                structure.mirror_axis = swap_axis;
                chunk.structure_table.push(structure);   
            }
        }
    }

    fn pre_generate_external_structures(&self, chunk_table: &mut ChunkTable, chunk_position: (i32, i32, i32)){
        let chunk = chunk_table.get_chunk_mut(chunk_position).unwrap();
        if chunk.structres_generated {return;}
        chunk.structres_generated = true;
        

        if !chunk.structers_pre_generated{
            self.pre_generate_structures(chunk_table, chunk_position);
        }

        let mut externals = Vec::new();
        for x in -1..2{
            for y in -1..2{
                for z in -1..2{
                    if !(x==0&&y==0&&z==0){
                        let next_position = chunk_position.add((x, y, z));
                        chunk_table.add_chunk(next_position);
                        self.pre_generate_structures(chunk_table, next_position);
                        
                        for strctr in chunk_table.get_chunk_mut(next_position).unwrap().structure_table.iter(){
                            externals.push(strctr.clone());
                        }
                    }
                }
            }
        }
        chunk_table.get_chunk_mut(chunk_position).unwrap().external_structure_table=externals;
    }

    fn generate_structures(&self, chunk: &mut Chunk, external: bool){
        let structures = if external {&chunk.external_structure_table} else {&chunk.structure_table};
        for world_structure in structures.iter(){
            let root_position = world_structure.root;
            let root_chunk_position = world_structure.root_chunk_position;
            
            for block in world_structure.blocks.iter(){
                let new_local_position = block.0.product(world_structure.miroor_multiplier).swap_axes(world_structure.mirror_axis);
                let new_position = root_position.add(new_local_position).add( root_chunk_position.sub(chunk.world_chunk_position).times(CHUNK_SIZE as i32) );
                if new_position.0 >= CHUNK_SIZE as i32 || new_position.0 < 0 || new_position.1 >= CHUNK_SIZE  as i32|| new_position.1 < 0 || new_position.2 >= CHUNK_SIZE  as i32|| new_position.2 < 0 {continue;};
                let new_index=((new_position.0<<CHUNK_SHIFT_2)+(new_position.1<<CHUNK_SHIFT)+new_position.2) as usize;
                chunk.blocks_table[new_index] = block.1;
            }
        }
    }

    fn greedy_mesher_calculator(&self, chunk_table: &mut ChunkTable, chunk_positon: (i32, i32, i32)){
        let chunk = chunk_table.get_chunk_mut(chunk_positon).unwrap();
        
        for axis in 0..3{
            for orientation in 0..2{
                let mut rectangles: Vec<(i32, i32, i32, i32, i32)> = Vec::new();
                for y in 0..CHUNK_SIZE{
                    let mut visited = vec![vec![false; CHUNK_SIZE]; CHUNK_SIZE];
                    let mut grid = vec![vec![-1; CHUNK_SIZE]; CHUNK_SIZE];
                    
                    for z in 0..CHUNK_SIZE{
                        for x in 0..CHUNK_SIZE{
                            let index: usize = 
                            if axis==0 {((x<<CHUNK_SHIFT_2)+(y<<CHUNK_SHIFT)+z) as usize}
                            else if axis==1 {((y<<CHUNK_SHIFT_2)+(z<<CHUNK_SHIFT)+x) as usize} // y->x z->y x->z
                            else {((z<<CHUNK_SHIFT_2)+(x<<CHUNK_SHIFT)+y) as usize}; //z->x x->y y->z  -->  x->z y->x z->y
                            
                            let has_side: bool = chunk.faces_table[index]&(0b1<<(axis*2+orientation)) == 0;
                            let block_type: i16 = chunk.blocks_table[index];
                            grid[z][x]= if !has_side || block_type==0 {-1} else {block_type};
                        }
                    }
                    
                    for z in 0..CHUNK_SIZE{
                        for x in 0..CHUNK_SIZE{
                            if visited[z][x] || grid[z][x]==-1{
                                continue;
                            }
                            
                            //rectangles.push((x as i32, y as i32, z as i32, 0, 0));
                            
                            let value = grid[z][x];
                            
                            // Znajdź największy możliwy prostokąt zaczynający się od (x, y)
                            let mut width = 0;
                            let mut height = 1;
                            
                            // Zwiększ szerokość, dopóki wartości są takie same
                            while x + width < CHUNK_SIZE && grid[z][x + width] == value && !visited[z][x + width] {
                                width += 1;
                            }
                            
                            // Zwiększ wysokość, jeśli wartości w całej szerokości są takie same
                            'outer: while z + height < CHUNK_SIZE {
                                for i in 0..width {
                                    if grid[z + height][x + i] != value || visited[z + height][x + i] {
                                        break 'outer;
                                    }
                                }
                                height += 1;
                            }
                            
                            // Oznacz wszystkie komórki prostokąta jako przetworzone
                            for dz in 0..height {
                                for dx in 0..width {
                                    visited[z + dz][x + dx] = true;
                                }
                            }
                            
                            // Dodaj prostokąt do listy
                            if axis==0{
                                rectangles.push((x as i32, y as i32, z as i32, width as i32-1, height as i32-1));
                            }
                            else if axis==1 {
                                rectangles.push((y as i32, z as i32, x as i32, width as i32-1, height as i32-1));
                            }
                            else {//z->x x->y y->z  -->  x->z y->x z->y
                                rectangles.push((z as i32, x as i32, y as i32, width as i32-1, height as i32-1));
                            }
                        }
                    }
            
                }
                chunk.rects_table[axis*2+orientation]=rectangles;
            }
        }
    }
    
    pub fn calculate_ambient_occlusion(&self, chunk_table: &mut ChunkTable, chunk_position: (i32, i32, i32)){
        if  chunk_table.get_chunk(chunk_position).unwrap().aot_calculated { return; }
        chunk_table.get_chunk_mut(chunk_position).unwrap().aot_calculated = true; 

        let mut ao_mask_table = [0 as u8; CHUNK_SIZE_3];
        for x in 0..CHUNK_SIZE{
            for y in 0..CHUNK_SIZE{
                for z in 0..CHUNK_SIZE{
                    let mut msk: u8 = 0;
                    for yd in [1, -1]{
                        let pos = (x as i32, y as i32, z as i32);
                        let index: usize = ((x<<CHUNK_SHIFT_2)+(y<<CHUNK_SHIFT)+z) as usize;

                        if chunk_table.get_chunk(chunk_position).unwrap().faces_table[index] == 63{
                            continue;
                        }
                        if chunk_table.get_chunk(chunk_position).unwrap().blocks_table[index] == 0{
                            continue;
                        }

                        if yd == 1 && chunk_table.get_block(chunk_position, (pos).add((0, 1, 0))) != 0 {continue;}

                        let extra_shift = if yd == 1 {0} else {4};
                        let case = if yd == 1 {3} else {1};
                        
                        let side1 = chunk_table.get_block(chunk_position, (pos).add((0, yd, -1))) != 0;
                        let side2 = chunk_table.get_block(chunk_position, (pos).add((-1, yd, 0))) != 0;
                        let corner = chunk_table.get_block(chunk_position, (pos).add((-1, yd, -1))) != 0;
                        
                        let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                        if ao_type<case {
                            msk |= 1<<(extra_shift);
                        }
                        
                        
                        let side1 = chunk_table.get_block(chunk_position, (pos).add((0, yd, -1))) != 0;
                        let side2 = chunk_table.get_block(chunk_position, (pos).add((1, yd, 0))) != 0;
                        let corner = chunk_table.get_block(chunk_position, (pos).add((1, yd, -1))) != 0;
                        
                        let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                        if ao_type<case {
                            msk |= 1<<(extra_shift+1);
                        }
                        
                        let side1 = chunk_table.get_block(chunk_position, (pos).add((0, yd, 1))) != 0;
                        let side2 = chunk_table.get_block(chunk_position, (pos).add((1, yd, 0))) != 0;
                        let corner = chunk_table.get_block(chunk_position, (pos).add((1, yd, 1))) != 0;
                        
                        let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                        if ao_type<case {
                            msk |= 1<<(extra_shift+2);
                        }

                        
                        let side1 = chunk_table.get_block(chunk_position, (pos).add((0, yd, 1))) != 0;
                        let side2 = chunk_table.get_block(chunk_position, (pos).add((-1, yd, 0))) != 0;
                        let corner = chunk_table.get_block(chunk_position, (pos).add((-1, yd, 1))) != 0;
                        
                        let ao_type = if side1 && side2 {0} else {3 - (if side1 {1} else {0} + if side2 {1} else {0} + if corner {1} else {0})};
                        if ao_type<case {
                            msk |= 1<<(extra_shift+3);
                        }
                    }
                    ao_mask_table[(x<<CHUNK_SHIFT_2)+(y<<CHUNK_SHIFT)+z] = msk;
                }
            }
        }
        chunk_table.get_chunk_mut(chunk_position).unwrap().aot_table=ao_mask_table;
    }

    pub fn calculate_faces_table(&self, chunk_table: &mut ChunkTable, chunk_position: (i32, i32, i32), lod: i32){
        if  chunk_table.get_chunk(chunk_position).unwrap().faces_calculated { return; }
        chunk_table.get_chunk_mut(chunk_position).unwrap().faces_calculated = true; 

        let top_position = (chunk_position.0, chunk_position.1+1, chunk_position.2);
        let bottom_position = (chunk_position.0, chunk_position.1-1, chunk_position.2);
        let right_position = (chunk_position.0+1, chunk_position.1, chunk_position.2);
        let left_position = (chunk_position.0-1, chunk_position.1, chunk_position.2);
        let front_position = (chunk_position.0, chunk_position.1, chunk_position.2-1);
        let back_position = (chunk_position.0, chunk_position.1, chunk_position.2+1);
        self.generate_chunk(chunk_table, top_position);
        self.generate_chunk(chunk_table, bottom_position);
        self.generate_chunk(chunk_table, front_position);
        self.generate_chunk(chunk_table, back_position);
        self.generate_chunk(chunk_table, right_position);
        self.generate_chunk(chunk_table, left_position);
        // let chunk = chunk_table.get_chunk(chunk_position).unwrap();
        // let top_chunk = chunk_table.get_chunk(top_position).unwrap();
        // let bottom_chunk = chunk_table.get_chunk(bottom_position).unwrap();
        // let right_chunk = chunk_table.get_chunk(right_position).unwrap();
        // let left_chunk = chunk_table.get_chunk(left_position).unwrap();
        // let front_chunk = chunk_table.get_chunk(front_position).unwrap();
        // let back_chunk = chunk_table.get_chunk(back_position).unwrap();
        let mut faces_table = [0 as u8; CHUNK_SIZE_3];
        for x in (0..CHUNK_SIZE).step_by(lod as usize){
            for y in (0..CHUNK_SIZE).step_by(lod as usize){
                for z in (0..CHUNK_SIZE).step_by(lod as usize){
                    let index: usize = ((x<<CHUNK_SHIFT_2)+(y<<CHUNK_SHIFT)+z) as usize;
                    let block_position = (x as i32, y as i32, z as i32);
                    let mut mesh_type_mask: u8 = 0b111111;

                    faces_table[index]=mesh_type_mask;
                    if chunk_table.get_block(chunk_position, block_position) == 0{
                        continue;
                    }
                    
               
                    if chunk_table.get_block(chunk_position, (0, lod, 0).add(block_position)) == 0{
                        mesh_type_mask &= 0b111110;
                    }
                    if chunk_table.get_block(chunk_position, (0, -lod, 0).add(block_position)) == 0{
                        mesh_type_mask &= 0b111101;
                    }
                    if chunk_table.get_block(chunk_position, (lod, 0, 0).add(block_position)) == 0{
                        mesh_type_mask &= 0b111011;
                    }
                    if chunk_table.get_block(chunk_position, (-lod, 0, 0).add(block_position)) == 0{
                        mesh_type_mask &= 0b110111;
                    }
                    if chunk_table.get_block(chunk_position, (0, 0, lod).add(block_position)) == 0{
                        mesh_type_mask &= 0b101111;
                    }
                    if chunk_table.get_block(chunk_position, (0, 0, -lod).add(block_position)) == 0{
                        mesh_type_mask &= 0b011111;
                    }
                    

                    // if y==CHUNK_SIZE-1 {//top
                    //     if top_chunk.blocks_table[((x<<CHUNK_SHIFT_2)+z) as usize] == 0{
                    //         mesh_type_mask &= 0b111110;
                    //     }
                    // }
                    // else if chunk.blocks_table[index+CHUNK_SIZE]==0{
                    //     mesh_type_mask &= 0b111110;
                    // }
                    // if y==0 {//bottom
                    //     if bottom_chunk.blocks_table[((x<<CHUNK_SHIFT_2)+z+(CHUNK_SIZE-1)*CHUNK_SIZE) as usize] == 0{
                    //         mesh_type_mask &= 0b111101;
                    //     }
                    // }
                    // else if chunk.blocks_table[index-CHUNK_SIZE]==0{
                    //     mesh_type_mask &= 0b111101;
                    // }
                    
                    
                    // if x==CHUNK_SIZE-1 {//right
                    //     if right_chunk.blocks_table[((y<<CHUNK_SHIFT)+z) as usize] == 0{
                    //         mesh_type_mask &= 0b111011;
                    //     }
                    // }
                    // else if chunk.blocks_table[index+CHUNK_SIZE_2]==0{
                    //     mesh_type_mask &= 0b111011;
                    // }
                    // if x==0 {//left
                    //     if left_chunk.blocks_table[((y<<CHUNK_SHIFT)+z+(CHUNK_SIZE-1)*CHUNK_SIZE_2) as usize] == 0{
                    //         mesh_type_mask &= 0b110111;
                    //     }
                    // }
                    // else if chunk.blocks_table[index-CHUNK_SIZE_2]==0{
                    //     mesh_type_mask &= 0b110111;
                    // }
                    
                    
                    // if z==CHUNK_SIZE-1 {//back
                    //     if back_chunk.blocks_table[((x<<CHUNK_SHIFT_2)+(y<<CHUNK_SHIFT)) as usize] == 0{
                    //         mesh_type_mask &= 0b101111;
                    //     }
                    // }
                    // else if chunk.blocks_table[index+1]==0{
                    //     mesh_type_mask &= 0b101111;
                    // }
                    // if z==0 {//front
                    //     if front_chunk.blocks_table[((x<<CHUNK_SHIFT_2)+(CHUNK_SIZE-1)+(y<<CHUNK_SHIFT)) as usize] == 0{
                    //         mesh_type_mask &= 0b011111;
                    //     }
                    // }
                    // else if chunk.blocks_table[index-1]==0{
                    //     mesh_type_mask &= 0b011111;
                    // }
                    faces_table[index]=mesh_type_mask;
                }
            }
        }
        chunk_table.get_chunk_mut(chunk_position).unwrap().faces_table=faces_table;
        //self.greedy_mesher_calculator(chunk_table, chunk_position);
    }
    
    pub fn generate_chunk(&self, chunk_table: &mut ChunkTable, chunk_position: (i32, i32, i32)){
        if chunk_table.get_add_chunk(chunk_position).generated {return;}
        chunk_table.get_add_chunk_mut(chunk_position).generated = true;
        
        
        
        self.pre_generate_external_structures(chunk_table, chunk_position);
       
        if chunk_position.1 > 5{
            return;
        }
        
        let chunk = chunk_table.get_chunk_mut(chunk_position).unwrap();
        let mut rng = rand::thread_rng(); // Tworzenie generatora
        
        for x in 0..CHUNK_SIZE{
            let xw: i32 = (chunk.world_chunk_position.0<<CHUNK_SHIFT)+x as i32;
            for z in 0..CHUNK_SIZE{
                let zw: i32 = (chunk.world_chunk_position.2<<CHUNK_SHIFT)+z as i32;
                
                
                let distance: i32 = self.get_distance(xw, zw);
                if distance>WorldGenerator::MAX_DISTANCE{
                    continue;
                }
                let elevation =  self.get_elevation(xw, zw);
                
                for y in 0..CHUNK_SIZE{
                    let yw: i32= (chunk.world_chunk_position.1<<CHUNK_SHIFT)+y as i32;
                    let distance2: i32 = self.get_distance2(xw, zw, yw);
                    if distance2>WorldGenerator::MAX_DISTANCE && yw<0{
                        continue;
                    }
                    let index = ((x<<CHUNK_SHIFT_2)+(y<<CHUNK_SHIFT)+z) as usize;
                    
                    
                    if yw <= elevation-8 && (rng.gen::<i32>().abs()%100)+((yw-(elevation-8))/5).pow(1).max(-33) <= 35{
                        chunk.blocks_table[index] = 5;
                    }
                    else if yw <= elevation-3{
                        chunk.blocks_table[index] = 1;
                    }
                    else if yw <= elevation-2{
                        chunk.blocks_table[index] = 2;
                    }                    
                    else if yw == elevation && yw>=WorldGenerator::WORLD_GRASS_LEVEL && self.get_perlin(xw, zw, 0.01)>0. {
                        chunk.blocks_table[index] = 4;
                    }
                    else if yw == elevation && (rng.gen::<i32>().abs())%100 <= 40 && yw >= WorldGenerator::WORLD_GRASS_LEVEL-2 {
                        chunk.blocks_table[index] = 4;
                    }
                    else if yw == elevation && yw<=WorldGenerator::WORLD_WATER_LEVEL{
                        chunk.blocks_table[index] = 6;
                    }
                    else if yw == elevation && (rng.gen::<i32>().abs())%100 <= 70 && yw <= WorldGenerator::WORLD_WATER_LEVEL+1 {
                        chunk.blocks_table[index] = 6;
                    }
                    else if yw <= elevation{
                        chunk.blocks_table[index] = 3;
                    }
                    else if yw <= WorldGenerator::WORLD_WATER_LEVEL{
                        chunk.blocks_table[index] = 9;
                    }
                }
            }
        }
        //println!("generating chunk x:{} y:{} z:{}", chunk.world_chunk_position.0, chunk.world_chunk_position.1, chunk.world_chunk_position.2);
        self.generate_structures(chunk, false);
        self.generate_structures(chunk, true);
    }
}