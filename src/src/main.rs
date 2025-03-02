use bevy::core_pipeline::tonemapping::DebandDither;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_rapier3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod world_generation;
mod rendering;
mod chunk_loader;
mod chunk;

//use rendering::create_cube_mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{  primary_window: Some(Window{ present_mode: bevy::window::PresentMode::Immediate, ..default() }), ..default() }))
        
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())

        .add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))

        .insert_resource(GameSettings{player_id: None, paused: true, camera_mode: CameraMode::Spectator, brightness: 1.})
        .insert_resource(world_generation::ChunkTable::default())
        .insert_resource(rendering::BlockColorTable::default())
        .insert_resource(rendering::BlockMeshTable::default())
        .insert_resource(rendering::BlockMaterialTable::default())
        .insert_resource(world_generation::WorldGenerator::default())
        .insert_resource(AmbientLight {
            color: Color::srgb(0.9, 0.8, 0.7), // Kolor ambient light
            brightness: 0.7e3, // Jasność ambient light
        })
        .insert_resource( ClearColor(Color::srgb_u8(38, 140, 226)) )



        .add_systems(Startup, setup)
        .add_systems(Startup, rendering::BlockMaterialTable::init_materials)
        .add_systems(Startup, rendering::BlockMeshTable::init_meshes)
        .add_systems(Update, update)
        .add_systems(Update, rendering::spawn_chunk)
        
        .run();
}

#[derive(Eq, PartialEq, Debug, Hash, Default)]
enum CameraMode {
    FirstPerson,
    #[default]
    Spectator,
    ThirdPerson
}

#[derive(Resource)]
struct GameSettings {
    player_id: Option<Entity>,
    paused: bool,
    camera_mode: CameraMode,
    brightness: f32
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MyGameCamera;

#[derive(Bundle)]
struct PlayerBundle{
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    restitution: Restitution,
    player: Player
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<GameSettings>,
) {

    game.player_id = Some(commands.spawn(PlayerBundle{
        transform: Transform::from_xyz(2., 54.5, 0.),
        rigid_body: RigidBody::Dynamic,
        collider: Collider::capsule_y(1., 0.5),
        restitution: Restitution::coefficient(0.05),
        player: Player
    }).insert(
        LockedAxes::ROTATION_LOCKED,
    ).insert(
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.5, 2.0)),
            material: materials.add(Color::srgb_u8(202, 178, 122)),
            transform: Transform::from_xyz(0.0, 15.5, 0.0),
            ..default()
        }
    ).insert(Velocity {
        linvel: Vec3::new(0.0, 2.0, 0.0),
        angvel: Vec3::new(0.2, 0.0, 0.0),
    }).insert(
        Friction::coefficient(0.95)
    )
    .id());

    //spawn3d camera


    commands.spawn((Camera3dBundle {
        deband_dither: DebandDither::Disabled,
        transform: Transform::from_xyz(-2.5, 14.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        // projection: OrthographicProjection {
        //     // 6 world units per window height.
        //     scaling_mode: ScalingMode::FixedVertical(21.0),
        //     ..default()
        // }
        // .into(),
        ..default()
    },
    // FogSettings {
    //     color: Color::srgba(0.35, 0.48, 0.66, 1.0),
    //     directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
    //     directional_light_exponent: 10.0,
    //     falloff: FogFalloff::from_visibility_colors(
    //         1450.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
    //         Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
    //         Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
    //     ),
    // },
    MyGameCamera));

    
    commands.spawn(
        TextBundle::from_section(
            "diagnostic data",
            TextStyle::default(),
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
    );
    // circular base
    commands.spawn(Collider::cuboid(15.0, 1., 15.0))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 11., 0.0)));
    
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Circle::new(6.0)),
    //     material: materials.add(Color::WHITE),
    //     transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //     ..default()
    // });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(30.0, 2.0, 30.0)),
        material: materials.add(Color::srgb_u8(0, 111, 209)),
        transform: Transform::from_xyz(0.0, 11.01, 0.0),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 255, 144)),
        transform: Transform::from_xyz(0.0, 12.5, 0.0),
        ..default()
    }).insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5, 0.5))
    .insert(Restitution::coefficient(0.7));


    //balll
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(0.5)),
        material: materials.add(Color::srgb_u8(204, 0, 0)),
        transform: Transform::from_xyz(2.0, 14.5, 0.0),
        ..default()
    }).insert(RigidBody::Dynamic)
    .insert(Collider::ball(0.5))
    .insert(Restitution::coefficient(0.7));
    

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::srgb_u8(240, 220, 150),
            intensity: 3222331.,
            range: 250.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
   
}


fn update(
    mut cameraq: Query<&mut Transform, With<MyGameCamera>>,
    mut playerq: Query<(&mut Velocity, &mut Transform), (With<Player>, Without<MyGameCamera>)>,
    mut windowq: Query<&mut Window, With<PrimaryWindow>>,
    mut textq: Query<&mut Text>,
    input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<GameSettings>,
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut chunk_table: ResMut<world_generation::ChunkTable>,
    mut light: ResMut<AmbientLight>,
    mut clear_color: ResMut<ClearColor>,
){
    let mut window = windowq.single_mut();
    let mut camera_transform = cameraq.single_mut();
    let (mut player_velocity, mut player_transform) = playerq.single_mut();
    let direction_forward_flat = Vec3::new(camera_transform.forward().x, 0., camera_transform.forward().z).normalize(); 
    let direction_right_flat = Vec3::new(camera_transform.right().x, 0., camera_transform.right().z).normalize(); 
    let direction_forward = camera_transform.forward().normalize(); 
    let direction_right = camera_transform.right().normalize(); 
    let mut speed = 5.;
    let jump_speed = 6.;
    let acceleration = 30.;
    let mouse_sensivity = 1.4;
    // chunk code

    let new_positon: (i32, i32, i32) ;
    if game.camera_mode == CameraMode::Spectator{
        new_positon = ((camera_transform.translation.x/chunk::CHUNK_SIZE as f32).floor() as i32, (camera_transform.translation.y/chunk::CHUNK_SIZE as f32).floor() as i32-1, (camera_transform.translation.z/chunk::CHUNK_SIZE as f32).floor() as i32);
    }
    else if game.camera_mode == CameraMode::ThirdPerson || true{
        new_positon = ((player_transform.translation.x/8.).floor() as i32, (player_transform.translation.y/8.).floor() as i32-1, (player_transform.translation.z/8.).floor() as i32);
    }
    else{
        new_positon = (21, 21, 21);
    }
    
    //println!("x:{} y:{} z:{}", chunk_table.player_position.0, chunk_table.player_position.1, chunk_table.player_position.2);
    //let chunk_table = chunk_tabler.as_mut();
    chunk_table.player_position = new_positon;

    //
    
    
    textq.single_mut().sections[0].value = format!("x:{:.2} y:{:.2} z:{:.2}\ncx:{} cy:{} cz:{}", camera_transform.translation.x, camera_transform.translation.y, camera_transform.translation.z, new_positon.0,  new_positon.1, new_positon.2);
    if input.just_pressed(KeyCode::Escape){
        game.paused = !game.paused;
    }
    if game.paused{
        switch_cursor(&mut window, true);
        return;
    }
    if input.pressed(KeyCode::ShiftLeft) {
        speed *= 2.5;
    }

    for motion in mouse_motion.read() {
        let anglex = -motion.delta.y*0.003*mouse_sensivity;
        let angley = -motion.delta.x*0.002*mouse_sensivity;
        camera_transform.rotate_y(angley);
        camera_transform.rotate_local_x(anglex);
        if camera_transform.rotation.to_euler(EulerRot::YXZ).1.to_degrees().abs() > 75. {
           camera_transform.rotate_local_x(-anglex);
        }  
    }
    switch_cursor(&mut window, false);
    center_cursor(&mut window);

    //if transform.translation.y > 1. {
        //transform.translation += Vec3::new(0., -5., 0.)*time.delta_seconds(); 
    //}

    if input.pressed(KeyCode::KeyL) {
        game.brightness *= 1.1
    }
    if input.pressed(KeyCode::KeyZ) {
        game.brightness *= 0.9;
    }
    light.brightness = game.brightness*1e3;
    clear_color.0 = Color::srgb_u8((38.*game.brightness).min(255.) as u8, (140.*game.brightness).min(255.) as u8, (226.*game.brightness).min(255.) as u8);
    


    if input.pressed(KeyCode::KeyF) && game.camera_mode != CameraMode::FirstPerson {
        game.camera_mode = CameraMode::FirstPerson;
        println!("{:?} mode on!", game.camera_mode);
    }
    else if input.pressed(KeyCode::KeyE) && game.camera_mode != CameraMode::Spectator {
        game.camera_mode = CameraMode::Spectator;
        println!("{:?} mode on!", game.camera_mode);
    }
    else if input.pressed(KeyCode::KeyH) && game.camera_mode != CameraMode::ThirdPerson {
        game.camera_mode = CameraMode::ThirdPerson;
        println!("{:?} mode on!", game.camera_mode);
    }
    
    
    if game.camera_mode == CameraMode::Spectator || game.camera_mode == CameraMode::ThirdPerson {
        if input.pressed(KeyCode::KeyW){
            camera_transform.translation += direction_forward*time.delta_seconds()*speed; 
        }
        if input.pressed(KeyCode::KeyS){
            camera_transform.translation -= direction_forward*time.delta_seconds()*speed; 
        }
        if input.pressed(KeyCode::KeyD){
            camera_transform.translation += direction_right*time.delta_seconds()*speed; 
        }
        if input.pressed(KeyCode::KeyA){
            camera_transform.translation -= direction_right*time.delta_seconds()*speed; 
        }
        if input.pressed(KeyCode::Space){
            camera_transform.translation += Vec3::new(0., 1., 0.)*time.delta_seconds()*speed; 
        }
        if input.pressed(KeyCode::ControlLeft){
            camera_transform.translation -= Vec3::new(0., 1., 0.)*time.delta_seconds()*speed; 
        }
    }
    if game.camera_mode == CameraMode::FirstPerson {
        //player_velocity.linvel = Vec3::new(0., player_velocity.linvel.y, 0.);
        let mut my_velocity = Vec3::new(player_velocity.linvel.x, 0., player_velocity.linvel.z);
        if input.pressed(KeyCode::KeyW){
            my_velocity += direction_forward_flat*time.delta_seconds()*acceleration; 
        }
        if input.pressed(KeyCode::KeyS){
            my_velocity -= direction_forward_flat*time.delta_seconds()*acceleration; 
        }
        if input.pressed(KeyCode::KeyD){
            my_velocity += direction_right_flat*time.delta_seconds()*acceleration; 
        }
        if input.pressed(KeyCode::KeyA){
            my_velocity -= direction_right_flat*time.delta_seconds()*acceleration; 
        }
        if my_velocity.length() > speed{
            my_velocity = my_velocity.normalize()*speed;
        }
        my_velocity.y = player_velocity.linvel.y;
        player_velocity.linvel = my_velocity;
        if input.just_pressed(KeyCode::Space){
            player_velocity.linvel.y = jump_speed; 
        }
        player_transform.rotation = Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0);
        camera_transform.translation = player_transform.translation+Vec3::new(0., 1.5, 0.);
    }
    if game.camera_mode == CameraMode::ThirdPerson {
        //player_velocity.linvel = Vec3::new(0., player_velocity.linvel.y, 0.);
        let mut my_velocity = Vec3::new(player_velocity.linvel.x, 0., player_velocity.linvel.z);
        if input.pressed(KeyCode::ArrowUp){
            my_velocity += direction_forward_flat*time.delta_seconds()*acceleration; 
        }
        if input.pressed(KeyCode::ArrowDown){
            my_velocity -= direction_forward_flat*time.delta_seconds()*acceleration; 
        }
        if input.pressed(KeyCode::ArrowRight){
            my_velocity += direction_right_flat*time.delta_seconds()*acceleration; 
        }
        if input.pressed(KeyCode::ArrowLeft){
            my_velocity -= direction_right_flat*time.delta_seconds()*acceleration; 
        }
        if my_velocity.length() > speed{
            my_velocity = my_velocity.normalize()*speed;
        }
        my_velocity.y = player_velocity.linvel.y;
        player_velocity.linvel = my_velocity;
        if input.just_pressed(KeyCode::Enter){
            player_velocity.linvel.y = jump_speed; 
        }
        //camera_transform.translation = player_transform.translation;
        player_transform.rotation = Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0);
    }
}

fn switch_cursor(window: &mut Window, visible: bool) {
    window.cursor.visible = visible; // Ukrywa kursor
}

fn center_cursor(window: &mut Window) {
    window.set_cursor_position(Some(Vec2 { x: window.width()/2., y: window.height()/2. }));
}