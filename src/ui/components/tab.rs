use oxygengine::{
	user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
}};
use serde::{Deserialize, Serialize};

#[derive(PropsData, Default, Debug, Clone, Serialize, Deserialize)]
pub struct TabProps {
    pub label: String,
}

pub fn tab(ctx: WidgetContext) -> WidgetNode {
    let tab_plate_props = ctx.props.read_cloned_or_default::<TabPlateProps>();
	let tab_props = ctx.props.read_cloned_or_default::<TabProps>();

	let variant = match tab_plate_props.active {
		true => "tab_active",
		false => "tab_inactive",
	};
	let tab_size = SizeBoxProps {
		height: SizeBoxSizeValue::Exact(30.), 
		width: SizeBoxSizeValue::Fill,
		..Default::default()
	};
	let tab_bkg = PaperProps { 
		variant: variant.to_owned(),
        frame: None, 
        ..Default::default() 
    };
	let tab_text_u = TextPaperProps {
		text: tab_props.label.to_owned(),
		width: TextBoxSizeValue::Fill,
		height: TextBoxSizeValue::Fill,
		transform: Transform {
			align: Vec2 { x: 0., y: 0.3 },
			..Default::default()
		},
		use_main_color: true,
		..Default::default()
	};

    widget! {
		(#{"tab_size_u"} size_box: {tab_size} {
			content = (#{"tab_bkg_u"} paper: {tab_bkg} [
				(#{"tab_text_u"} text_paper: {tab_text_u})
			])
		})
	}
}
