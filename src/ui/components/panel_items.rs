use std::collections::HashMap;
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
    let mut childs = HashMap::new();
    if let Some(refe) = props.refe {
        childs = refe.childs;
    }

	let size = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(100.), 
        width: SizeBoxSizeValue::Exact(100.),
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
	
    let childs_list = childs.iter()
        .map(|(_code, child)| {
            let name = TextPaperProps {
                variant: "unit".to_owned(),
                text: child.name.to_owned(),
                width: TextBoxSizeValue::Fill,
                height: TextBoxSizeValue::Fill,
                use_main_color: true,
                ..Default::default()
            };
            let prev_pic = Props::new(ImageBoxProps {
                width: ImageBoxSizeValue::Exact(32.),
                height: ImageBoxSizeValue::Exact(32.),
                material: ImageBoxMaterial::Image(ImageBoxImage {
                    id: child.preview.to_owned(),
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

            let costs_list = child.cost.iter()
                .map(|(_code, cost)| {
                    let cost_text = TextPaperProps {
                        variant: "unit".to_owned(),
                        text: "1".to_string(), //cost.name.to_owned(),
                        width: TextBoxSizeValue::Fill,
                        height: TextBoxSizeValue::Fill,
                        use_main_color: true,
                        ..Default::default()
                    };
                    widget! {
                        (#{"cost"} text_paper: {cost_text})
                    }
                });

            widget! {
                (#{"size_item"} size_box: {size.to_owned()} {
                    content = (#{"content"} content_box [
                        (#{"bkg"} paper: {bkg.to_owned()})
                        (#{"v_box"} vertical_box : {margin.to_owned()} [
                            (#{"name"} text_paper: {name})
                            (#{"prev"} content_box  [
                                (#{"prev_pic"} image_box: {prev_pic})
                            ])
                            (#{"costs"} horizontal_box |[ costs_list ]|)
                        ])
                    ])
                })
            }
        })
        .collect::<Vec<_>>();

    widget! {
        (#{key} flex_box |[ childs_list ]|)
    }
}