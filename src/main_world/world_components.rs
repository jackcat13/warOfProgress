use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Component, PartialEq)]
pub struct Villager;

#[derive(Component, PartialEq)]
pub struct House;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Selected {
    pub entities: Vec<UnitId>,
}

#[derive(Component)]
pub struct SelectChild;

#[derive(Component, Eq, Hash, PartialEq, Clone, Deref, DerefMut)]
pub struct UnitId(pub String);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct NewPositions {
    pub positions: HashMap<UnitId, Vec2>,
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct CurrentMouseAsset {
    pub asset: Option<Handle<Image>>,
}

#[derive(Component)]
pub struct MouseComponent;

#[derive(Debug)]
pub enum MenuAction {
    House,
}

#[derive(Resource)]
pub struct PlayerResources {
    pub wood: i8,
    pub stone: i8,
    pub gold: i8,
}

impl Default for PlayerResources {
    fn default() -> Self {
        PlayerResources {
            wood: 100,
            stone: 50,
            gold: 50,
        }
    }
}

pub struct BuildingCost {
    pub wood_cost: i8,
    pub stone_cost: i8,
    pub gold_cost: i8,
}
