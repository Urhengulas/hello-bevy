mod hello_plugin;

use bevy::prelude::*;

use crate::hello_plugin::HelloPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
