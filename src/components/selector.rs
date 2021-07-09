use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Selector(pub bool);

impl Prefab for Selector {}
impl PrefabComponent for Selector {}