use oxygengine::user_interface::raui::{
	core::prelude::*,
	material::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    ui::components::panel_cost,
    resources::referential::RefeItem
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PanelItemProps {
    #[serde(default)]
    pub item: RefeItem,
}
implement_props_data!(PanelItemProps);

pub fn panel_item(context: WidgetContext) -> WidgetNode {
	
	let WidgetContext {
        key,
        props,
        ..
    } = context;

    let item_props = props.read_cloned_or_default::<PanelItemProps>();
    
	let size = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(120.), 
        width: SizeBoxSizeValue::Exact(120.),
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

    let name = TextPaperProps {
        variant: "unit".to_owned(),
        text: item_props.item.name,
        width: TextBoxSizeValue::Fill,
        height: TextBoxSizeValue::Fill,
        use_main_color: true,
        ..Default::default()
    };
    let prev_pic = Props::new(ImageBoxProps {
        width: ImageBoxSizeValue::Exact(32.),
        height: ImageBoxSizeValue::Exact(32.),
        material: ImageBoxMaterial::Image(ImageBoxImage {
            id: item_props.item.preview,
            ..Default::default()
        }),
        ..Default::default()
    }).with( ContentBoxItemLayout {
        margin: Rect {
            left: 32.,
            right: 32.,
            top: 0.,
            bottom: 0.,
        },
        ..Default::default()
    });

    let costs_list = item_props.item.cost.iter()
        .map(|(_code, sic)| {
            widget! {
                (#{sic.item.name} panel_cost::panel_cost : { panel_cost::PanelCostProps { sic: sic.clone() }} )
            }
        }).collect::<Vec<_>>();
    
	let btn_props = props.to_owned()
        .with(PaperProps { frame: None, ..Default::default() })
        .with(NavItemActive);
        //.with(ButtonNotifyProps(id.to_owned().into()));

    widget! {
        (#{key} size_box: {size.to_owned()} {
            content = (#{"content"} button_paper: {btn_props} {
                content = (#{"v_box"} vertical_box: {margin.to_owned()} [
                    (#{"name"} text_paper: {name.to_owned()})
                    (#{"prev_pic"} image_box: {prev_pic})
                    (#{"h-box"} horizontal_box |[ costs_list ]|)
                ])
            })
        })
    }
}