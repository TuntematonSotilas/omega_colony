use oxygengine::widget::WidgetId;

#[derive(Default)]
pub struct UiWidget {
	pub side_panel: Option<WidgetId>,
	pub top_bar: Option<WidgetId>,
}