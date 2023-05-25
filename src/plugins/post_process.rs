use std::borrow::Borrow;
use bevy::prelude::*;
use crate::prelude::*;
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy::render::render_resource::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::reflect::TypeUuid;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::texture::BevyDefault;
use bevy::render::view::RenderLayers;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
	fn build(&self, app: &mut App){
		app.add_startup_system(post_process_setup);
	}
}

fn post_process_setup(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut post_processing_materials: ResMut<Assets<EightBitPostProcessingMaterial>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut images: ResMut<Assets<Image>>,
	mut query: Query<&TilemapTexture>,
) {
	let window = windows.primary_mut();
	let size = Extent3d {
		width: window.physical_width(),
		height: window.physical_height(),
		..default()
	};

	println!("Window physical height: {}, float width: {}, scale factor: {}", size.height, window.height(), window.scale_factor());
	// This is the texture that will be rendered to.
	let mut image = Image {
		texture_descriptor: TextureDescriptor {
			label: None,
			size,
			dimension: TextureDimension::D2,
			format: TextureFormat::bevy_default(),
			mip_level_count: 1,
			sample_count: 1,
			usage: TextureUsages::TEXTURE_BINDING
					| TextureUsages::COPY_DST
					| TextureUsages::RENDER_ATTACHMENT,
		},
		..default()
	};

	// fill image.data with zeroes
	image.resize(size);

	let image_handle = images.add(image);

	// Light
	// NOTE: Currently lights are ignoring render layers - see https://github.com/bevyengine/bevy/issues/3462
	commands.spawn(PointLightBundle {
		transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
		..default()
	});

	commands.spawn(Camera2dBundle{
		camera: Camera {
			target: RenderTarget::Image(image_handle.clone()),
			is_active: true,
			..default()},
		projection: OrthographicProjection{
			scaling_mode: ScalingMode::FixedVertical(RESOLUTION),
			..Default::default()
		},
		..Default::default()
	});

	// This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
	let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

	let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
		size.width as f32,
		size.height as f32,
	))));

	// This material has the texture that has been rendered.
	let material_handle = post_processing_materials.add(EightBitPostProcessingMaterial {
		source_image: image_handle,
	});

	//Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: quad_handle.into(),
			material: material_handle,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.5),
				..default()
			},
			..default()
		},
		post_processing_pass_layer,
	));

	// The post-processing pass camera.
	commands.spawn((
		Camera2dBundle {
			camera: Camera {
				// renders after the first main camera which has default value: 0.
				priority: 1,
				..default()
			},
			projection: OrthographicProjection{
				scaling_mode: ScalingMode::FixedVertical(RESOLUTION),
				..Default::default()
			},
			..Camera2dBundle::default()
		},
		post_processing_pass_layer,
	));
}

// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub(crate) struct EightBitPostProcessingMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	source_image: Handle<Image>,
}

impl Material2d for EightBitPostProcessingMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/eight_bit_lines.wgsl".into()
	}
}