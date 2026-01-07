use bevy::prelude::*;

#[derive(Resource, Default, PartialEq)]
pub enum GameMode {
    //Start,
    #[default]
    Playing,
    //Paused,
    GameOver
}

impl GameMode {
    // restrict to only LEGAL transitions
    /*
        Start -> Playing
        Playing -> Paused | GameOver
        Paused -> Playing | Start
        GameOver -> Start | Playing
    */
    pub fn change(&self) -> Self {
        match self {
            Self::Playing   => Self::GameOver,
            Self::GameOver  => Self::Playing
        }
    }
}