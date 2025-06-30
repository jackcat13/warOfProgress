use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Component, PartialEq)]
pub struct Villager;

#[derive(Component, PartialEq)]
pub struct House;

#[derive(Component)]
pub struct Achievement {
    pub progress: f32,
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Selected {
    pub entities: Vec<UnitId>,
}

#[derive(Component)]
pub struct SelectChild;

#[derive(Component, Eq, Hash, PartialEq, Clone, Deref, DerefMut, Debug)]
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

#[derive(Component)]
pub struct BuildTarget {
    pub id: Option<UnitId>,
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

pub struct BuildingSpecs {
    pub r#type: MenuAction,
    pub cost: BuildingCost,
}

impl BuildingSpecs {
    pub fn resolve_in_progress_asset_path(&self) -> String {
        match self.r#type {
            MenuAction::House => "caveman_age/buildings/house_in_progress_caveman.png".to_string()
        }
    }

    pub fn resolve_finished_asset_path(&self) -> String {
        match self.r#type {
            MenuAction::House => "caveman_age/buildings/house_caveman.png".to_string()
        }
    }
}

pub struct BuildingCost {
    pub wood: i8,
    pub stone: i8,
    pub gold: i8,
}

#[derive(Component)]
pub enum ResourceText {
    Wood, Stone, Gold
}
