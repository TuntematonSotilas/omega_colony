use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};
use crate::{
    resources::time::TIME_STORAGE, 
    ui::components::{stars, menu_btn},
    storage::sto_utils,
};

const FRAMES: Scalar = 10.;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MenuState {
	pub alpha: Scalar,
    pub sec: Option<u32>,
}
implement_props_data!(MenuState);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MenuTextProps {
    #[serde(default)]
    pub title: String,
}

implement_props_data!(MenuTextProps);

widget_hook! {
    pub use_menu(life_cycle) {
		life_cycle.mount(|context| {
            let sec_opt = sto_utils::get::<u32>(TIME_STORAGE);
			drop(context.state.write(MenuState {
				alpha: 0.,
                sec: sec_opt,
			}));
        });

		life_cycle.change(|context| {
			let mut state = context.state.read_cloned_or_default::<MenuState>();
			if state.alpha < 1. {
				state.alpha += 1. / FRAMES;
			}
			drop(context.state.write(state));
		});
	}
}

widget_component! {
    
    pub menu_comp(key, props, state) [use_menu] {
        if let Ok(state) = state.read::<MenuState>() {
            let margin = Props::new(ContentBoxItemLayout {
                margin: Rect {
                    top: 200.,
                    bottom: 200.,
                    ..Default::default()
                },
                ..Default::default()
            });
            let text_prop = props.read_cloned_or_default::<MenuTextProps>();
            let title = Props::new(TextPaperProps {
                text: text_prop.title,
                width: TextBoxSizeValue::Fill,
                height: TextBoxSizeValue::Fill,
                use_main_color: true,
                ..Default::default()
            });
            
            let mut time_txt = "No save".to_string();
            let mut continue_btn = widget! {()};
            
            if let Some(sec) = state.sec
            {
                time_txt = format!("Time played : {0}s", sec);
                continue_btn = widget! {
                    (#{"continue_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                        id: "continue".to_string(),
                        label: "Continue".to_string(),
                    }})
                };
            }

            let time = Props::new(TextPaperProps {
                text: time_txt.to_owned(),
                width: TextBoxSizeValue::Fill,
                height: TextBoxSizeValue::Fill,
                use_main_color: true,
                ..Default::default()
            });
            
            let size_btns = SizeBoxProps {
                width: SizeBoxSizeValue::Exact(100.),
                height: SizeBoxSizeValue::Exact(70.),
                ..Default::default()
            };
    
            let v_box_btns = VerticalBoxProps {
                transform: Transform {
                    align: Vec2 { x: 3., y: 0.},
                    ..Default::default()
                },
                ..Default::default()
            };
            
            widget! {
                (#{key} nav_content_box [
                    (#{"stars"} stars::stars)
                    (#{"margin"} content_box: {margin} | {WidgetAlpha(state.alpha)} [
                        (#{"v-box"} vertical_box [
                            (#{"text"} text_paper: {title})
                            (#{"time"} text_paper: {time})
                            (#{"size-btns"} size_box: {size_btns} {
                                content = (#{"btns-box"} vertical_box: {v_box_btns} [
                                    (#{"continue_wrap"} content_box [
                                        {continue_btn}
                                    ])
                                    (#{"new_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                                        id: "new_game".to_string(),
                                        label: "New Game".to_string(),
                                    }})
                                ])
                            })
                        ])
                    ])
                ])
            }
        } else {
            widget!{()}
        }
	}
}

widget_component! {
    pub menu(key) {
        widget! {
            (#{key} menu_comp : { MenuTextProps { 
                title: "Menu".to_owned()
            }})
        }
    }
}
