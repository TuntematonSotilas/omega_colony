use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Flash {
    pub show_time: Scalar,
    pub phase: Scalar,
}

impl Component for Flash {
    type Storage = VecStorage<Self>;
}

impl Prefab for Flash {}
impl PrefabComponent for Flash {}