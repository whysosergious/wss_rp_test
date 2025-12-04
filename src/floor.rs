use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub fn setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let texture = images.add(create_checkerboard_texture());
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(25.0, 25.0)).mesh().size(50.0, 50.0)),
        // The above line was user-edited.
        // Replaced:
        //   mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(50.0, 50.0)),
        // With:
        //   mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(25.0, 25.0)).mesh().size(50.0, 50.0)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            ..default()
        }),
        ..default()
    });
}

fn create_checkerboard_texture() -> Image {
    let size = Extent3d {
        width: 1024,
        height: 1024,
        depth_or_array_layers: 1,
    };
    let mut data = vec![0; (size.width * size.height * 4) as usize];
    for y in 0..size.height {
        for x in 0..size.width {
            let idx = ((y * size.width) + x) as usize * 4;
            let color = if (x / 64) % 2 == (y / 64) % 2 {
                [128, 128, 128, 255]
            } else {
                [64, 64, 64, 255]
            };
            data[idx..idx + 4].copy_from_slice(&color);
        }
    }
    Image::new(
        size,
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    )
}
