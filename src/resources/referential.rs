#[derive(Default, Clone)]
pub struct Item  {
    pub code: String,
    pub name: String,
    pub cost: u32,
}

#[derive(Default, Clone)]
pub struct Referential {
    pub units: Vec<Item>,
	pub upgrades: Vec<Item>,
}