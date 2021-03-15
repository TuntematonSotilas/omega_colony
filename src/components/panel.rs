use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Panel(pub bool);

impl Component for Panel {
    type Storage = VecStorage<Self>;
}

impl Prefab for Panel {}
impl PrefabComponent for Panel {}