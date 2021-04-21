use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};
use crate::{
    resources::time::{Time, TIME_STORAGE}, 
    ui::components::{stars, menu_btn},
    storage::sto_utils,
};

const FRAMES: Scalar = 10.;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MenuState {
	pub alpha: Scalar,
    pub time: Option<Time>,
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
            let time_opt = sto_utils::get::<Time>(TIME_STORAGE);
			drop(context.state.write(MenuState {
				alpha: 0.,
                time: time_opt,
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
            let text_prop = props.read_cloned_or_default::<MenuTextProps>();
            let title = TextPaperProps {
                text: text_prop.title,
                variant: String::new(),
                use_main_color: true,
                ..Default::default()
            };
            let margin = Props::new(ContentBoxItemLayout {
                margin: Rect {
                    top: 200.,
                    bottom: 200.,
                    ..Default::default()
                },
                ..Default::default()
            });
            
            let list_items = match &state.time {
                Some(time) => {
                    let time_props = Props::new(TextBoxProps {
                        height: TextBoxSizeValue::Exact(10.),
                        text: format!("Time played : {0}s", time.sec.to_owned()),
                        alignment: TextBoxAlignment::Center,
                        font: TextBoxFont {
                            name: "fonts/orbitron.json".to_owned(),
                            size: 14.0,
                        },
                        color: color_from_rgba(0, 153, 255, 1.),
                        ..Default::default()
                    });

                    vec![
                        widget! {
                            (#{"title"} text_paper: {title})
                        },
                        widget! {
                            (#{"continue_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                                id: "continue".to_string(),
                                label: "Continue".to_string(),
                            }})
                        },
                        widget! {
                            (#{"time"} text_box: {time_props})
                        },
                        widget! {
                            (#{"new_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                                id: "new_game".to_string(),
                                label: "New Game".to_string(),
                            }})
                        },
                    ]
                },
                None => vec![
                    widget! {
                        (#{"title"} text_paper: {title})
                    },
                    widget! {
                        (#{"new_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                            id: "new_game".to_string(),
                            label: "New Game".to_string(),
                        }})
                    },
                ],
            };

            widget! {
                (#{key} nav_content_box [
                    (#{"stars"} stars::stars)
                    (#{"margin"} content_box: {margin} | {WidgetAlpha(state.alpha)} [
                        (#{"v-box"} vertical_box | [ list_items ] |)
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
