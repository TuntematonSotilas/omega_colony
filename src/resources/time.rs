pub const TIME_STORAGE: &str = "time";

#[derive(Default)]
pub struct Time {
	pub launched: bool,
    pub phase: f32,
    pub sec: u32,
}