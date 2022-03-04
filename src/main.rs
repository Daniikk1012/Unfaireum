use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use unfaireum::UnfaireumPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugins(UnfaireumPlugins)
        .run();
}
