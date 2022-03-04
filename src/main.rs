use bevy::prelude::*;
use unfaireum::UnfaireumPlugins;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugins(UnfaireumPlugins).run();
}
