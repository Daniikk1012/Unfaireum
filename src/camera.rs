use bevy::{
    prelude::*,
    render::camera::{ScalingMode, WindowOrigin},
};

#[derive(Component)]
pub struct CameraSize(Vec2);

#[derive(Component)]
pub struct GameCamera;

pub fn init(mut commands: Commands) {
    let mut bundle = OrthographicCameraBundle::new_2d();
    bundle.orthographic_projection.scaling_mode = ScalingMode::None;
    bundle.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands
        .spawn_bundle(bundle)
        .insert(CameraSize(Vec2::new(1920.0, 1080.0)))
        .insert(GameCamera);
}

pub fn resize(
    windows: Res<Windows>,
    mut query: Query<(&mut OrthographicProjection, &Camera, &CameraSize)>,
) {
    let (mut projection, camera, CameraSize(min_size)) = query.single_mut();

    let window = windows.get(camera.window).unwrap();
    let window_size = Vec2::new(window.width(), window.height());

    let size = if window_size.x * min_size.y > min_size.x * window_size.y {
        Vec2::new(window_size.x * min_size.y / window_size.y, min_size.y)
    } else {
        Vec2::new(min_size.x, window_size.y * min_size.x / window_size.x)
    };

    if let ScalingMode::None = projection.scaling_mode {
        match projection.window_origin {
            WindowOrigin::Center => {
                let half_size = size / 2.0;

                if projection.left != -half_size.x
                    || projection.bottom != -half_size.y
                    || projection.right == half_size.x
                    || projection.top == half_size.y
                {
                    projection.left = -half_size.x;
                    projection.bottom = -half_size.y;
                    projection.right = half_size.x;
                    projection.top = half_size.y;
                }
            }
            WindowOrigin::BottomLeft => {
                if projection.left != 0.0
                    || projection.bottom != 0.0
                    || projection.right != size.x
                    || projection.top != size.y
                {
                    projection.left = 0.0;
                    projection.bottom = 0.0;
                    projection.right = size.x;
                    projection.top = size.y;
                }
            }
        }
    }
}
