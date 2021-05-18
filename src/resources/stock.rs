use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Stock {
	pub data: HashMap<String, u32>,
}

impl Stock {
	pub fn init(&mut self) {
		self.data = HashMap::new();
		self.data.insert("energy".to_string(), 100);
		self.data.insert("water".to_string(), 100);
        self.data.insert("cereal".to_string(), 100);
        self.data.insert("steel".to_string(), 100);
	}
}