use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InteractiveSprite {
    pub size: Scalar,
	pub code: String,
}

impl Component for InteractiveSprite {
    type Storage = VecStorage<Self>;
}

impl Prefab for InteractiveSprite {}
impl PrefabComponent for InteractiveSprite {}