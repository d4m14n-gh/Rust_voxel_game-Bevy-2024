use std::collections::VecDeque;

use bevy::prelude::*;
use hashbrown::HashSet;

use crate::world_generation::I32x3Extension;

#[derive(Resource)]
pub struct ChunkLoader{
    player_position: (i32, i32, i32),
    render_distance: i32,
    rendered_chunk_set: HashSet<(i32, i32, i32)>,
    spawned_set: HashSet<(i32, i32, i32)>
}

impl Default for ChunkLoader{
    fn default() -> Self {
        ChunkLoader{
            player_position: (0, 0, 0),
            render_distance: 10,
            rendered_chunk_set: HashSet::new(),
            spawned_set: HashSet::new()       
        }
    }
}

impl ChunkLoader{
    pub fn set_player_position(&mut self, position: (i32, i32, i32)){
        self.player_position = position;
    }
    pub fn get_chunks_to_generate(){
        
    }
    pub fn get_chunks_to_spawn(){

    }
    pub fn get_chunks_to_despawn(){

    }
    pub fn bfs_distance_points_generator(&mut self, center: (i32, i32, i32)) -> Vec<(i32, i32, i32)>{
        //println!("bfs");
        let distance = self.render_distance;
        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
        let mut new_position_set: Vec<(i32, i32, i32)> = Vec::new();
    
        if self.rendered_chunk_set.is_empty(){
            queue.push_back(center);
            self.rendered_chunk_set.insert(center);
            new_position_set.push(center);
        }
        else{
            for pos in self.rendered_chunk_set.iter(){
                queue.push_back(*pos);
            }
        }
    
        while !queue.is_empty() {
            let point: (i32, i32, i32) = queue.pop_front().unwrap();
            for ox in -1..2{
                for oz in -1..2{
                    for oy in -1..2{
                        let new_point: (i32, i32, i32) = (point.0+ox, point.1+oy, point.2+oz);
                        if (new_point.0-center.0)*(new_point.0-center.0) + (new_point.1-center.1)*(new_point.1-center.1) + (new_point.2-center.2)*(new_point.2-center.2) <= distance*distance && !self.rendered_chunk_set.contains(&new_point){
                            queue.push_back(new_point);
                            self.rendered_chunk_set.insert(new_point);
                            new_position_set.push(new_point);
                        }
                    }
                }
            }
        }
        return new_position_set;
    }
}