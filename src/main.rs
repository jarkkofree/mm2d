use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    sprite::Mesh2dHandle,
};
// use bevy::math::DVec2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CircleHandles>()
        .add_systems(Startup, setup_system)
        .add_systems(Update,modify_circle_system) 
        .run();
}

#[derive(Resource)]
struct CircleHandles {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
}

impl Default for CircleHandles {
    fn default() -> Self {
        CircleHandles {
            mesh: Mesh2dHandle(Default::default()),
            material: Default::default()
        }
    }
}

fn setup_system (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut circle_handles: ResMut<CircleHandles>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    let circle = shape::Circle::new(10.0).into();
    println!("circle: {:?}", circle);

    circle_handles.mesh = meshes.add(circle).into();
    println!("mesh: {:?}", circle_handles.mesh);

    let red = ColorMaterial::from(Color::RED);
    println!("red: {:?}", red);

    circle_handles.material = materials.add(red);
    println!("material: {:?}", circle_handles.material);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    println!("transform: {:?}", transform);

    let e = commands.spawn((
        MaterialMesh2dBundle::<ColorMaterial> {
            mesh: circle_handles.mesh.clone(),
            material: circle_handles.material.clone(),
            transform,
            ..default()
        },
    )).id();
    println!("e: {:?}", e);
}

fn modify_circle_system(
    circle_handles: Res<CircleHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Access the mesh and material using the handles
    if let Some(mesh) = meshes.get_mut(&circle_handles.mesh.0) {
        // Modify the mesh here
    }

    if let Some(material) = materials.get_mut(&circle_handles.material) {
        // Modify the material here
    }
}