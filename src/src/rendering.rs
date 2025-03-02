use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use hashbrown::HashSet;
//use bevy_rapier3d::prelude::Collider;
//use bevy_rapier3d::prelude::*;
use std::collections::VecDeque;

use bevy::render::{
    mesh::Indices, //, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};

use crate::{chunk, world_generation::{ChunkTable, I32x3Extension, WorldGenerator}};

#[derive(Resource)]
pub struct BlockColorTable{
    colors: Vec<Color>
}
impl Default for BlockColorTable {
    fn default() -> Self {
        BlockColorTable{
            colors: vec![
                //0-air 1-stone 2-dirt 3-grass 4-grass2 
                Color::srgb_u8(127, 127, 127), Color::srgb_u8(127, 127, 127), Color::srgb_u8(129, 93, 41), Color::srgb_u8(106, 153, 85), Color::srgb_u8(50, 144, 113),
                //5-stone_exp  6-sand 7-wood 8-leaves 9-water
                Color::srgb_u8(103, 103, 105), Color::srgb_u8(220, 201, 124), Color::srgb_u8(73, 61, 40), Color::srgb_u8(41, 150, 46), Color::srgb_u8(0, 122, 204),
                //10-brich wood 11-brich_leaves 12-red_oak_wood 12 red_oak_leaves 
                Color::srgb_u8(136, 136, 136), Color::srgb_u8(185, 175, 24), Color::srgb_u8(64, 44, 25), Color::srgb_u8(141, 56, 33),
             ]
        }
    }
}

#[derive(Resource)]
pub struct BlockMaterialTable{
    materials: Vec<Handle<StandardMaterial>>
}
impl Default for BlockMaterialTable {
    fn default() -> Self {
        BlockMaterialTable{
            materials: Vec::new()
        }
    }
}
impl BlockMaterialTable{
    pub fn init_materials(
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut block_materials: ResMut<BlockMaterialTable>,
        block_colors: Res<BlockColorTable>,
    ){
        for color in block_colors.colors.iter(){
            block_materials.materials.push(materials.add(StandardMaterial {
                base_color: color.clone(),
                ..default()
            }));

        }
    }
}


pub fn bfs_distance_points_generator(center: (i32, i32, i32), visited: &mut HashSet<(i32, i32, i32)>, distance: i32) -> Vec<(i32, i32, i32)>{
    //println!("bfs");
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let mut new_position_set: Vec<(i32, i32, i32)> = Vec::new();

    let dst = distance+10;
    for ox in -dst..dst{
        for oz in -dst..dst{
            for oy in -dst..dst{
                let new_point = center.add((ox, oy, oz));
                if (new_point.0-center.0)*(new_point.0-center.0) + (new_point.1-center.1)*(new_point.1-center.1) + (new_point.2-center.2)*(new_point.2-center.2) <= distance*distance && !visited.contains(&new_point){

                    new_position_set.push(center.add((ox, oy, oz)));
                    visited.insert(center.add((ox, oy, oz)));
                }
            }
        }
    }

    return new_position_set;

    if visited.is_empty(){
        queue.push_back(center);
        visited.insert(center);
        new_position_set.push(center);
    }
    else{
        for pos in visited.iter(){
            queue.push_back(*pos);
        }
    }

    while !queue.is_empty() {
        let point: (i32, i32, i32) = queue.pop_front().unwrap();
        for ox in -1..2{
            for oz in -1..2{
                for oy in -1..2{
                    let new_point: (i32, i32, i32) = (point.0+ox, point.1+oy, point.2+oz);
                    if (new_point.0-center.0)*(new_point.0-center.0) + (new_point.1-center.1)*(new_point.1-center.1) + (new_point.2-center.2)*(new_point.2-center.2) <= distance*distance && !visited.contains(&new_point){
                        queue.push_back(new_point);
                        visited.insert(new_point);
                        new_position_set.push(new_point);
                    }
                }
            }
        }
    }
    return new_position_set;
}

///pub fn actual_spawn()

pub fn spawn_chunk( //do rozbicia na spawnowanie blokow
    mut commands: Commands,

    //mut materials: ResMut<Assets<StandardMaterial>>,
    //mut meshes: ResMut<Assets<Mesh>>,

    mut chunk_table: ResMut<ChunkTable>,
    block_colors: Res<BlockColorTable>,
    block_materials: Res<BlockMaterialTable>,
    block_meshes: Res<BlockMeshTable>,
    generator: ResMut<WorldGenerator>,
){    
    let player_position = chunk_table.player_position;
    // let a = player_position;
    // let b = chunk_table.prev_player_position ;
    // if (a.0-b.0)*(a.0-b.0) +  (a.1-b.1)*(a.1-b.1) + (a.2-b.2)*(a.2-b.2) < 2{
    //     //print!("ww");
    //     return;
    // } 
    chunk_table.prev_player_position = player_position;
    let render_distance = 120/chunk::CHUNK_SIZE as i32;//15;


    let points = bfs_distance_points_generator(player_position, &mut chunk_table.rendered_chunk_set , render_distance);
    //chunk_table.update_rendered_chunks(&mut commands, player_position, render_distance);
    for position in points{
        if chunk_table.get_add_chunk(position).spawned {
            continue;
        }
        let lod: usize= 1;//if position.0>2 {2} else {1};
        generator.generate_chunk(chunk_table.as_mut(), position);
        generator.calculate_faces_table(chunk_table.as_mut(), position, lod as i32);
        generator.calculate_ambient_occlusion(chunk_table.as_mut(), position);
        

        let mut entity_vector: Vec<Entity> = Vec::new();
        let chunk = chunk_table.get_chunk(position).unwrap();
        
        let adjust: f32 =  if lod!=1 {0.25*lod as f32} else {0.};//0.25*lod as f32 + 0.5;
        for x in 0..chunk::CHUNK_SIZE{
            for y in 0..chunk::CHUNK_SIZE{
                for z in 0..chunk::CHUNK_SIZE{
                    if x%lod!=0 || y%lod!=0||z%lod!=0{
                        continue;
                    }
                    let xw: i32 = (chunk.world_chunk_position.0<<chunk::CHUNK_SHIFT)+x as i32;
                    let yw: i32= (chunk.world_chunk_position.1<<chunk::CHUNK_SHIFT)+y as i32;
                    let zw: i32 = (chunk.world_chunk_position.2<<chunk::CHUNK_SHIFT)+z as i32;
                    let index: usize = ((x<<chunk::CHUNK_SHIFT_2)+(y<<chunk::CHUNK_SHIFT)+z) as usize;
                    let block_type = chunk.blocks_table[index];
                    let aot_mask = chunk.aot_table[index];
                    let mesh_type_mask: u8 = chunk.faces_table[index];//|63;
                                        
                    
                    let color: Color;
                    if block_type == 3 || block_type == 8{ // || 4
                        color = block_colors.colors[block_type as usize].mix(&Color::srgb_u8(77, 189, 145), (generator.get_perlin(xw, zw, 0.04))*1.);
                    }
                    else if block_type == 9{
                        color = block_colors.colors[block_type as usize].mix(&Color::srgb_u8(56, 167, 241), (((xw as f32) + (zw as f32))/1.).sin()*0.125);
                    }
                    else{
                        color = block_colors.colors[block_type as usize];
                    }
                    
                    if block_type != 0 && mesh_type_mask != 63 {
                        let transform = Transform::from_xyz((xw) as f32 + adjust, yw as f32 + adjust, zw as f32 + adjust).with_scale(Vec3::new(lod as f32, lod as f32, lod as f32));
                        let mut ec = commands.spawn((
                            PbrBundle {    
                                mesh: block_meshes.table[aot_mask as usize].clone(),
                                // material: materials.add(StandardMaterial {
                                //     base_color: color,
                                //     ..default()
                                // }),
                                material: block_materials.materials[block_type as usize].clone(),
                                transform: transform,
                                ..default()
                            },
                        ));
                        if y == chunk::CHUNK_SIZE-1 || chunk.blocks_table[index+chunk::CHUNK_SIZE] == 0{
                            ec.insert(Collider::cuboid(0.5, 0.5, 0.5));
                        }
                        //entity_vector.push(ec.id());
                    }
                }
            }
        }


        
        // for (orientation, axis) in [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)]{
        //     for rect in chunk.rects_table[axis*2+orientation].iter(){
        //         let xw: i32 = (chunk.world_chunk_position.0<<chunk::CHUNK_SHIFT)+rect.0;
        //         let yw: i32= (chunk.world_chunk_position.1<<chunk::CHUNK_SHIFT)+rect.1;
        //         let zw: i32 = (chunk.world_chunk_position.2<<chunk::CHUNK_SHIFT)+rect.2;
        //         let index: usize = ((rect.0<<chunk::CHUNK_SHIFT_2)+(rect.1<<chunk::CHUNK_SHIFT)+rect.2) as usize;
        //         let block_type = chunk.blocks_table[index];
                
        //         let mut color: Color;
                
        //         if block_type == 3 || block_type == 8{ // || 4
        //             color = block_colors.colors[block_type as usize].mix(&Color::srgb_u8(77, 189, 145), (generator.get_perlin(xw, zw, 0.04))*1.);
        //         }
        //         else if block_type == 9{
        //             color = block_colors.colors[block_type as usize].mix(&Color::srgb_u8(56, 167, 241), (((xw as f32) + (zw as f32))/1.).sin()*0.125);
        //         }
        //         else{
        //             color = block_colors.colors[block_type as usize];
        //         }

        //         color = color.darker(((xw*yw%50).abs() as f32)/250.0);
                
        //         let mesh = create_rect_mesh(rect.0, rect.1, rect.2, rect.3, rect.4, axis as i32, orientation==0);
        //         let tmp = chunk.world_chunk_position.times(chunk::CHUNK_SIZE as i32);
        //         let ec = commands.spawn((
        //             PbrBundle {    
        //                 mesh: meshes.add(mesh),
        //                 material: block_materials.materials[block_type as usize].clone().unwrap(),
        //                 transform: Transform::from_xyz(tmp.0 as f32, tmp.1 as f32, tmp.2 as f32),
        //                 //transform: Transform::from_xyz(xw as f32, yw as f32, zw as f32),
        //                 ..default()
        //             },
        //         ));
        //         entity_vector.push(ec.id());
        //     }
        // }


        // let mesh = create_chunk_mesh(&chunk.rects_table);
        // if mesh.get_vertex_size() != 0{

        //     let tmp = chunk.world_chunk_position.times(8);
        //     let color = Color::srgb_u8(100, 100, 100);
        //     let ec = commands.spawn((
        //         PbrBundle {    
        //             mesh: meshes.add(mesh),
        //             material: block_materials.materials[0 as usize].clone().unwrap(),
        //             transform: Transform::from_xyz(tmp.0 as f32, tmp.1 as f32, tmp.2 as f32),
        //             //transform: Transform::from_xyz(xw as f32, yw as f32, zw as f32),
        //             ..default()
        //         },
        //     ));
        //     entity_vector.push(ec.id());
        // }
            
        chunk_table.get_chunk_mut(position).unwrap().entities_table = entity_vector;
        chunk_table.get_chunk_mut(position).unwrap().spawned = true;
        //println!("adding chunk x:{} y:{} z:{}", position.0, position.1, position.2);
    }
}


#[derive(Resource)]
pub struct BlockMeshTable{
    table: Vec<Handle<Mesh>>
}
impl Default for BlockMeshTable {
    fn default() -> Self {
        
        BlockMeshTable{
            table: Vec::new()
        }
    }
}
impl BlockMeshTable{
    pub fn init_meshes(
        mut meshes_table: ResMut<BlockMeshTable>,
        mut meshes: ResMut<Assets<Mesh>>
    ){
        for i in 0..256{
            meshes_table.table.push(   meshes.add(create_cube_mesh2(i as u8))  );
        }
    }
}

fn create_vertices_normals(x:i32, y:i32, z:i32, xwidth: i32, zheight: i32, axis: i32, positive: bool) -> (Vec<[f32; 3]>, Vec<[f32; 3]>){
    let xw = xwidth as f32;
    let zh = zheight as f32;
    let center_axis = if positive {0.5} else {-0.5};
    let center_normal = center_axis*2 as f32;

    let mut  vertices = vec![
        [-0.5, center_axis, -0.5],
        [xw+0.5, center_axis, -0.5], 
        [xw+0.5, center_axis, zh+0.5], 
        [-0.5, center_axis, zh+0.5],
    ];
    let mut normals = vec![
            [0.0, center_normal, 0.0],
            [0.0, center_normal, 0.0],
            [0.0, center_normal, 0.0],
            [0.0, center_normal, 0.0],
    ];
    // y->x z->y x->z  x->y y-> z->x
    if axis == 1{
        for i in 0..4{
            vertices[i][1] = vertices[i][2];
            vertices[i][2] = vertices[i][0];
            vertices[i][0] = center_axis;
            
            normals[i][0]=center_normal;
            normals[i][1]=0.;
        }
    }//z->x x->y y->z  -->  x->z y->x z->y
    else if axis == 2{
        for i in 0..4{
            vertices[i][1] = vertices[i][0];
            vertices[i][0] = vertices[i][2];
            vertices[i][2] = center_axis;
            
            normals[i][2]=center_normal;
            normals[i][1]=0.;
        }
    }
    for i in 0..4{
        vertices[i][0] += x as f32;
        vertices[i][1] += y as f32;
        vertices[i][2] += z as f32;
    }
    return (vertices, normals);
}

fn create_chunk_mesh(faces: &[Vec<(i32, i32, i32, i32, i32)>; 6]) -> Mesh{
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let indeces_offset = [vec![0,3,1 , 1,3,2], vec![0,1,3 , 1,2,3]];

    let mut cnt = 0;
    for axis in 0..3{
        for orientation in 0..2{
            for rect in faces[axis*2+orientation].iter(){
                let (mut new_vertices, mut new_normals) = create_vertices_normals(rect.0, rect.1, rect.2, rect.3, rect.4, axis as i32, orientation==0);
                vertices.append(&mut new_vertices);
                normals.append(&mut new_normals);
                let mut new_indices = indeces_offset[orientation].clone();
                for k in 0..6{
                    new_indices[k]+=cnt*4;
                }
                indices.append(&mut new_indices);
                cnt+=1;
            }
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals
    )
    .with_inserted_indices(Indices::U32(indices))
}

fn create_rect_mesh(x:i32, y:i32, z:i32, xwidth: i32, zheight: i32, axis: i32, positive: bool) -> Mesh{
    let xw = xwidth as f32;
    let zh = zheight as f32;
    let center_axis = if positive {0.5} else {-0.5};

    let mut  vertices = vec![
        [-0.5, center_axis, -0.5],
        [xw+0.5, center_axis, -0.5], 
        [xw+0.5, center_axis, zh+0.5], 
        [-0.5, center_axis, zh+0.5],
    ];
    let indices = if positive{vec![0,3,1 , 1,3,2]}
    else{vec![0,1,3 , 1,2,3]};
   
    let center_normal = center_axis*2 as f32;
    let mut normals = vec![
            // Normals for the top side (towards +y)
            [0.0, center_normal, 0.0],
            [0.0, center_normal, 0.0],
            [0.0, center_normal, 0.0],
            [0.0, center_normal, 0.0],
    ];

    // y->x z->y x->z  x->y y-> z->x
    if axis == 1{
        for i in 0..4{
            vertices[i][1] = vertices[i][2];
            vertices[i][2] = vertices[i][0];
            vertices[i][0] = center_axis;
            
            normals[i][0]=center_normal;
            normals[i][1]=0.;
        }
    }//z->x x->y y->z  -->  x->z y->x z->y
    else if axis == 2{
        for i in 0..4{
            vertices[i][1] = vertices[i][0];
            vertices[i][0] = vertices[i][2];
            vertices[i][2] = center_axis;
            
            normals[i][2]=center_normal;
            normals[i][1]=0.;
        }
    }
    for i in 0..4{
        vertices[i][0] += x as f32;
        vertices[i][1] += y as f32;
        vertices[i][2] += z as f32;
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.clone()
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals.clone()
    )
    .with_inserted_indices(Indices::U32(indices.clone()))
}


fn create_ambient_occlusion(occlusion_type: u8, mask: u8){
    let old_vertices = vec![
        // top (facing towards +y)
        [-0.5, 0.5, -0.5], // vertex with index 0
        [0.5, 0.5, -0.5], // vertex with index 1
        [0.5, 0.5, 0.5], // etc. until 23
        [-0.5, 0.5, 0.5],
        // bottom   (-y)
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        // right    (+x)
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5], 
        [0.5, 0.5, -0.5],
        // left     (-x)
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        // back     (+z)
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        // forward  (-z)
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        ];
    let mut new_colors: Vec<[f32; 4]> = Vec::new();
    for i in 0..6{
        if mask&(1<<i) == 0{
            for j in 0..4{
                
                let mut is_shaded = false;
                for w in 0..8{
                    if occlusion_type&(1<<w) != 0{
                        let mut same = true;
                        for k in 0..3{
                            if old_vertices[i*4+j][k] != old_vertices[w][k]{
                                same = false;
                            }
                        }
                        if same{
                            is_shaded = true;
                        }
                    }
                }

                if is_shaded {
                    new_colors.push([0.5, 0.5, 0.5, 1.0].clone());
                }else{
                    new_colors.push([1.0, 1.0, 1.0, 1.0].clone());
                }
            }
        }
    }
    //return new_colors;
}
#[rustfmt::skip]
fn create_cube_mesh(mask: u8) -> Mesh {
    let old_vertices = vec![
        // top (facing towards +y)
        [-0.5, 0.5, -0.5], // vertex with index 0
        [0.5, 0.5, -0.5], // vertex with index 1
        [0.5, 0.5, 0.5], // etc. until 23
        [-0.5, 0.5, 0.5],
        // bottom   (-y)
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        // right    (+x)
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5], 
        [0.5, 0.5, -0.5],
        // left     (-x)
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        // back     (+z)
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        // forward  (-z)
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        ];

    let old_normals = vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
    ];

    let old_indeces = vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // front (-z)
    ];

    let mut new_vertices: Vec<[f32; 3]> = Vec::new();
    let mut new_normals: Vec<[f32; 3]> = Vec::new();
    let mut new_colors: Vec<[f32; 4]> = Vec::new();
    for i in 0..6{
        if mask&(1<<i) == 0{
            for j in 0..4{
                new_vertices.push(old_vertices[i*4+j].clone());
                new_normals.push(old_normals[i*4+j].clone());
                
                let ws: [u8; 4] = [4, 5, 6, 7];
                let mut is_shaded = false;
                for w in 0..8{
                    if ws.contains(&w){
                        let mut same = true;
                        for k in 0..3{
                            if old_vertices[i*4+j][k] != old_vertices[w as usize][k]{
                                same = false;
                            }
                        }
                        if same{
                            is_shaded = true;
                        }
                    }
                }

                if is_shaded {
                    new_colors.push([0.5, 0.5, 0.5, 1.0].clone());
                }else{
                    new_colors.push([1.0, 1.0, 1.0, 1.0].clone());
                }
            }
        }
    }

    let mut cnt = 0;
    let mut new_indeces: Vec<u32> = Vec::new();
    for i in 0..6{
        if mask&(1<<i) == 0{
            for j in 0..6{
                new_indeces.push(old_indeces[j+i*6]-(i as u32)*4+cnt*4);
            }
            cnt += 1;
        }
    }


    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "front" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        new_vertices
    )


    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    // .with_inserted_attribute(
    //     Mesh::ATTRIBUTE_UV_0,
    //     vec![
    //         // Assigning the UV coords for the top side.
    //         [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
    //         // Assigning the UV coords for the bottom side.
    //         [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
    //         // Assigning the UV coords for the right side.
    //         [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
    //         // Assigning the UV coords for the left side.
    //         [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
    //         // Assigning the UV coords for the back side.
    //         [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
    //         // Assigning the UV coords for the front side.
    //         [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
    //     ],
    // )

    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        new_normals
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR,
        new_colors
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_inserted_indices(Indices::U32(new_indeces))
}

fn create_cube_mesh2(mask: u8) -> Mesh {
    let old_vertices = vec![
        // top (facing towards +y)
        [-0.5, 0.5, -0.5], // vertex with index 0
        [0.5, 0.5, -0.5], // vertex with index 1
        [0.5, 0.5, 0.5], // etc. until 23
        [-0.5, 0.5, 0.5],
        // bottom   (-y)
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        // right    (+x)
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5], 
        [0.5, 0.5, -0.5],
        // left     (-x)
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        // back     (+z)
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        // forward  (-z)
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        ];

    let old_normals = vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
    ];

    let old_indeces = vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // front (-z)
    ];

    let mut new_vertices: Vec<[f32; 3]> = Vec::new();
    let mut new_normals: Vec<[f32; 3]> = Vec::new();
    let mut new_colors: Vec<[f32; 4]> = Vec::new();
    for i in 0..6{
            for j in 0..4{
                new_vertices.push(old_vertices[i*4+j].clone());
                new_normals.push(old_normals[i*4+j].clone());
                
                //let ws: [u8; 4] = [4, 5, 6, 7];
                let mut is_shaded = false;
                for w in 0..8{
                    if mask&(1<<w) != 0{
                        let mut same = true;
                        for k in 0..3{
                            if old_vertices[i*4+j][k] != old_vertices[w as usize][k]{
                                same = false;
                            }
                        }
                        if same{
                            is_shaded = true;
                        }
                    }
                }

                if is_shaded {
                    new_colors.push([0.5, 0.5, 0.5, 1.0].clone());
                }else{
                    new_colors.push([1.0, 1.0, 1.0, 1.0].clone());
                }
            }
    }

    let mut cnt = 0;
    let mut new_indeces: Vec<u32> = Vec::new();
    for i in 0..6{
            for j in 0..6{
                new_indeces.push(old_indeces[j+i*6]-(i as u32)*4+cnt*4);
            }
            cnt += 1;
    }


    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "front" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        new_vertices
    )


    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    
    
    // .with_inserted_attribute(
    //     Mesh::ATTRIBUTE_UV_0,
    //     vec![
    //         // Assigning the UV coords for the top side.
    //         [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
    //         // Assigning the UV coords for the bottom side.
    //         [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
    //         // Assigning the UV coords for the right side.
    //         [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
    //         // Assigning the UV coords for the left side.
    //         [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
    //         // Assigning the UV coords for the back side.
    //         [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
    //         // Assigning the UV coords for the front side.
    //         [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
    //     ],
    // )

    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        new_normals
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_COLOR,
        new_colors
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_inserted_indices(Indices::U32(new_indeces))
}