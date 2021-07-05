/*use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::referential::RefeCode;

#[derive(Serialize, Deserialize)]
pub struct InteractiveSprite {
    pub w: Scalar,
	pub h: Scalar,
	pub code: RefeCode,
}

impl Component for InteractiveSprite {
    type Storage = VecStorage<Self>;
}

impl Prefab for InteractiveSprite {}
impl PrefabComponent for InteractiveSprite {}*/