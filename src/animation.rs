use std::path::Path;

use bevy::{
    asset::{AssetServerError, HandleId},
    prelude::*,
};

use crate::physics::Velocity;

pub trait LoadAnimation {
    fn load_animation<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<Handle<Image>>, AssetServerError>;
}

impl LoadAnimation for AssetServer {
    fn load_animation<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Vec<Handle<Image>>, AssetServerError> {
        let handles = self.load_folder(&path)?;

        let mut result = Vec::with_capacity(handles.len());

        for index in 0..handles.len() {
            result.push(self.get_handle(HandleId::AssetPathId(
                path.as_ref().join(format!("{}.png", index)).into(),
            )));
        }

        Ok(result)
    }
}

pub struct Animation {
    pub frame: usize,
    pub textures: Vec<Handle<Image>>,
    pub now: f32,
    pub max: f32,
    pub next: Option<usize>,
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            frame: Default::default(),
            textures: Default::default(),
            now: Default::default(),
            max: 1.0,
            next: Default::default(),
        }
    }
}

#[derive(Default, Component)]
pub struct Animations {
    pub animations: Vec<Animation>,
    pub current: usize,
}

#[derive(Component)]
pub struct Flippable;

pub fn animation(
    time: Res<Time>,
    mut query: Query<(&mut Animations, &mut Handle<Image>)>,
) {
    for (mut animations, mut texture) in query.iter_mut() {
        let changed = animations.is_changed();
        let current = animations.current;
        let mut animation = &mut animations.animations[current];
        let mut frame = animation.frame;

        animation.now += time.delta_seconds();

        while animation.now >= animation.max {
            frame += 1;
            animation.now -= animation.max;
        }

        if let Some(next) = animation.next {
            if frame >= animation.textures.len() {
                animations.current = next;
                let current = animations.current;
                let mut animation = &mut animations.animations[current];
                animation.frame = 0;
                animation.now = 0.0;
                *texture = animation.textures[animation.frame].clone();
            } else {
                animation.frame = frame;
                *texture = animation.textures[animation.frame].clone();
            }
        } else {
            frame %= animation.textures.len();

            if changed || animation.frame != frame {
                animation.frame = frame;
                *texture = animation.textures[animation.frame].clone();
            }
        }

    }
}

pub fn flip(mut query: Query<(&mut Sprite, &Velocity), With<Flippable>>) {
    for (mut sprite, velocity) in query.iter_mut() {
        if velocity.0.x < 0.0 && !sprite.flip_x {
            sprite.flip_x = true;
        } else if velocity.0.x > 0.0 && sprite.flip_x {
            sprite.flip_x = false;
        }
    }
}
