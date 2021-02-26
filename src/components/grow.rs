use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Grow {
    pub phase: Scalar,
    pub max: Scalar,
}

impl Component for Grow {
    type Storage = VecStorage<Self>;
}

impl Prefab for Grow {}
impl PrefabComponent for Grow {}