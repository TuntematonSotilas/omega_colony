use std::collections::HashMap;

#[derive(Clone)]
pub enum RefeCateg {
	Unit,
	Building,
	Upgrade
}

impl Default for RefeCateg {
    fn default() -> Self { RefeCateg::Unit }
}

#[derive(Default, Clone)]
pub struct RefeItem  {
    pub name: String,
	pub category: RefeCateg,
    pub cost: u32,
}

#[derive(Default, Clone)]
pub struct Referential {
	pub refes: HashMap<String, RefeItem>
}

impl Referential {
	pub fn init(&mut self) {

		let base = RefeItem {
			name: "BASE".to_string(),
			category: RefeCateg::Building,
			cost: 0,
		};
		self.refes = HashMap::new();
		self.refes.insert("base".to_string(), base);
	}
}