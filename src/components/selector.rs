use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Selector(pub bool);

impl Component for Selector {
    type Storage = VecStorage<Self>;
}

impl Prefab for Selector {}
impl PrefabComponent for Selector {}