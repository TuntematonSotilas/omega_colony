use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Clickable {
    pub is: bool,
}

impl Component for Clickable {
    type Storage = VecStorage<Self>;
}

impl Prefab for Clickable {}
impl PrefabComponent for Clickable {}