use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct RefeItem  {
    pub name: String,
    pub cost: u32,
}

#[derive(Default, Clone)]
pub struct Referential {
	pub is_init: bool,
	pub buildings: HashMap<String, RefeItem>,
}

impl Referential {
	pub fn init(&mut self) {
		let base = RefeItem {
			name: "BASE".to_string(),
			cost: 0,
		};
		self.buildings = HashMap::new();
		self.buildings.insert("base".to_string(), base);
		self.is_init = true;
	}
}