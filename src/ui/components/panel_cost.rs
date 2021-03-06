use oxygengine::user_interface::raui::{
	core::prelude::*,
	material::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::resources::stock::StockItemCost;

#[derive(PropsData, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PanelCostProps {
    pub sic: StockItemCost,
}

pub fn panel_cost(context: WidgetContext) -> WidgetNode {
	
    let props = context.props.read_cloned_or_default::<PanelCostProps>();

    let pic_cost = ImageBoxProps {
        width: ImageBoxSizeValue::Exact(10.),
        height: ImageBoxSizeValue::Exact(10.),
        material: ImageBoxMaterial::Image(ImageBoxImage {
            id: props.sic.item.pic,
            ..Default::default()
        }),
        ..Default::default()
    };
    let text_cost = TextPaperProps {
        variant: "unit".to_owned(),
        text: props.sic.cost.to_string(),
        width: TextBoxSizeValue::Exact(15.),
        height: TextBoxSizeValue::Fill,
        use_main_color: true,
        horizontal_align_override: Some(TextBoxHorizontalAlign::Left),
        ..Default::default()
    };
    let size_cost = SizeBoxProps {
        height: SizeBoxSizeValue::Fill, 
        width: SizeBoxSizeValue::Exact(25.),
        ..Default::default()
    };

    widget! {
        (#{context.key} size_box: {size_cost} {
            content = (#{"box_cost"} horizontal_box [
                (#{"pic_cost"} image_box: {pic_cost})
                (#{"text_cost"} text_paper: {text_cost})
            ])
        })
    }
}