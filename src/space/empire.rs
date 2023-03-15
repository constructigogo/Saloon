use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Faction(pub u32);

/*
    separate entities in multiple factions
    TODO : all of it

*/