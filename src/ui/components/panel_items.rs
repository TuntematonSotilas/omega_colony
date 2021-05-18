use oxygengine::user_interface::raui::{
	core::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::resources::referential::RefeItem;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PanelItemProps {
    #[serde(default)]
    pub refe: Option<RefeItem>,
}
implement_props_data!(PanelItemProps);

pub fn panel_items(mut context: WidgetContext) -> WidgetNode {
	let WidgetContext {
		key,
        ..
    } = context;

    let props = context.props.read_cloned_or_default::<PanelItemProps>();
    let mut preview_pic = String::new();
    if let Some(refe) = props.refe {
        preview_pic = refe.preview;
    }
    
	let preview = ImageBoxProps {
		width: ImageBoxSizeValue::Exact(32.),
		height: ImageBoxSizeValue::Exact(32.),
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: preview_pic.to_owned(),
			..Default::default()
		}),
		..Default::default()
	};
    let size_item = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(32.), 
        width: SizeBoxSizeValue::Exact(32.),
        ..Default::default()
    };
    widget! {
        (#{key} flex_box [
            (#{"size_item"} size_box: {size_item} {
                content = (#{"preview"} image_box: {preview})
            })
		])
    }
}