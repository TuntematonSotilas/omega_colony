use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TextAni {
    pub curr_text: usize,
    pub phase: Scalar,
    pub interval: Scalar,
    pub texts: Vec<String>,
    pub color: Color,
}

impl Component for TextAni {
    type Storage = VecStorage<Self>;
}

impl Prefab for TextAni {}
impl PrefabComponent for TextAni {}