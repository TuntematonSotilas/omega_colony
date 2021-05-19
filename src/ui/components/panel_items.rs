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
	let bkg = PaperProps { 
        frame: None, 
        variant: "data".to_owned(),
        ..Default::default() 
    };
	let size_cost = SizeBoxProps {
        height: SizeBoxSizeValue::Fill, 
        width: SizeBoxSizeValue::Exact(25.),
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
                .map(|(_code, sic)| {
                    let pic_cost = ImageBoxProps {
                        width: ImageBoxSizeValue::Exact(10.),
                        height: ImageBoxSizeValue::Exact(10.),
                        material: ImageBoxMaterial::Image(ImageBoxImage {
                            id: sic.item.pic.to_owned(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    };
                    let text_cost = TextPaperProps {
                        variant: "unit".to_owned(),
                        text: sic.cost.to_string(),
                        width: TextBoxSizeValue::Exact(15.),
                        height: TextBoxSizeValue::Fill,
                        use_main_color: true,
                        alignment_override: Some(TextBoxAlignment::Left),
                        ..Default::default()
                    };
                    
                    widget! {
                        (#{sic.item.name} size_box: {size_cost.to_owned()} {
                            content = (#{"box_cost"} horizontal_box [
                                (#{"pic_cost"} image_box: {pic_cost})
                                (#{"text_cost"} text_paper: {text_cost})
                            ])
                        })
                    }
                }).collect::<Vec<_>>();
                
            widget! {
                (#{child.name} size_box: {size.to_owned()} {
                    content = (#{"content"} content_box [
                        (#{"bkg"} paper: {bkg.to_owned()})
                        (#{"v_box"} vertical_box : {margin.to_owned()} [
                            (#{"name"} text_paper: {name.to_owned()})
                            (#{"prev_box"} content_box  [
                                (#{"prev_pic"} image_box: {prev_pic})
                            ])
                            (#{"h-box"} horizontal_box |[ costs_list ]|)
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