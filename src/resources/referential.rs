use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct RefeItem  {
    pub name: String,
	pub preview: String,
    pub cost: u32,
}

#[derive(Default, Clone)]
pub struct Referential {
	pub is_init: bool,
	pub buildings: HashMap<String, RefeItem>,
	pub units: HashMap<String, RefeItem>,
}

impl Referential {
	pub fn init(&mut self) {
		// buildings
		let base = RefeItem {
			name: "BASE".to_string(),
			preview: String::new(),
			cost: 0,
		};
		self.buildings = HashMap::new();
		self.buildings.insert("base".to_string(), base);
		
		// units
		let technician = RefeItem {
			name: "Technician".to_string(),
			preview: "ui/technician.png".to_string(),
			cost: 0,
		};
		self.units = HashMap::new();
		self.units.insert("technician".to_string(), technician);
		
		self.is_init = true;
	}
}