use oxygengine::user_interface::raui::{
    core::{
        implement_props_data, 
        prelude::*,
    },
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProps {
    #[serde(default)]
    pub img: String,
}
implement_props_data!(ResourceProps);

pub fn resource(context: WidgetContext) -> WidgetNode {
	let props = context.props.read_cloned_or_default::<ResourceProps>();
	let size = SizeBoxProps {
		width: SizeBoxSizeValue::Exact(80.), 
		height: SizeBoxSizeValue::Exact(20.),
		..Default::default()
	};
	let bkg = PaperProps { 
		variant: "data".to_string(),
		frame: None, 
		..Default::default() 
	};
	let margin = ContentBoxItemLayout {
		margin: Rect {
			top: 1.,
			left: 4.,
			..Default::default()
		},
		..Default::default()
	};
	let img = ImageBoxProps {
		width: ImageBoxSizeValue::Exact(16.),
		height: ImageBoxSizeValue::Exact(16.),
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: props.img.to_owned(),
			..Default::default()
		}),
		..Default::default()
	};
	let text = TextPaperProps {
        text: "0".to_owned(),
        width: TextBoxSizeValue::Exact(60.),
        height: TextBoxSizeValue::Fill,
        use_main_color: true,
		alignment_override: Some(TextBoxAlignment::Right),
		transform: Transform {
            align: Vec2 { x: -0.1, y: 0.1},
            ..Default::default()
        },
        ..Default::default()
    };
	widget! {
		(#{context.key} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"margin"} content_box : {margin} [
					(#{"h-box"} horizontal_box [
						(#{"img"} image_box: {img})
						(#{"text"} text_paper: {text.clone()})
					])	
				])
			])
		})
	}
}