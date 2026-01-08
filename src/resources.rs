use bevy::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct Score(u32);
impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[derive(Resource)]
pub(crate) struct ScoreRequirements {
    pub no_damage: bool,
    pub over_robot: bool
}
impl Default for ScoreRequirements {
    fn default() -> Self {
        ScoreRequirements { no_damage: true, over_robot: false }
    }
}
impl ScoreRequirements {
    pub(crate) fn fully_met(&self) -> bool {
        self.no_damage && self.over_robot
    }

    pub(crate) fn reset(&mut self) {
        // this is weird, prob shouldn't need dummy default??
        *self = Self::default();
    }
}