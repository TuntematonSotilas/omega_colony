use oxygengine::user_interface::raui::{
	core::prelude::*,
	material::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::resources::referential::RefeItem;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PanelItemProps {
    #[serde(default)]
    pub refe: Option<RefeItem>,
}
implement_props_data!(PanelItemProps);

pub fn panel_items(context: WidgetContext) -> WidgetNode {
	let WidgetContext {
		key,
        ..
    } = context;

    let props = context.props.read_cloned_or_default::<PanelItemProps>();
    let mut preview_pic = String::new();
    if let Some(refe) = props.refe {
        preview_pic = refe.preview;
    }

	let size = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(32.), 
        width: SizeBoxSizeValue::Exact(32.),
        ..Default::default()
    };
	let margin_ext = ContentBoxItemLayout {
		margin: Rect {
            left: 10.,
            right: 10.,
            top: 10.,
            bottom: 10.,
        },
		..Default::default()
	};
	let margin = ContentBoxItemLayout {
		margin: Rect {
            left: 5.,
            right: 5.,
			top: 5.,
            bottom: 5.,
        },
		..Default::default()
	};
	let bkg = PaperProps { 
        frame: None, 
        variant: "data".to_owned(),
        ..Default::default() 
    };
	let preview = ImageBoxProps {
		width: ImageBoxSizeValue::Exact(32.),
		height: ImageBoxSizeValue::Exact(32.),
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: preview_pic.to_owned(),
			..Default::default()
		}),
		..Default::default()
	};

    widget! {
        (#{key} flex_box [
            /*(#{"size_item"} size_box: {size} {
                content = (#{"content"} content_box [
					(#{"margin_ext"} content_box : {margin_ext} [
					(#{"bkg_title"} paper: {bkg})
					(#{"margin_title"} content_box : {margin} [
						(#{"preview"} image_box: {preview})
					])
				])
            })*/
			(#{"preview"} image_box: {preview})
		])
    }
}