use bevy::prelude::*;

// #[derive(Component, Debug)]
// pub struct Score {
//     pub value: u32,
// }
//
// impl Score {
//     pub fn new() -> Self {
//         Self { value }
//     }
// }

#[derive(Resource)]
pub struct Score {
    pub value: i64,
    pub lives: i64,
}

impl Default for Score {
    fn default() -> Score {
        Score {
            value: 0,
            lives: 3,
        }
    }
}