use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};
use web_sys::window;
use crate::{
    resources::time::{Time, TIME_STORAGE}, 
    ui::components::{stars, menu_btn}
};

const FRAMES: Scalar = 10.;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MenuState {
	pub alpha: Scalar,
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
            let time_opt = get_time();
			if let Some(time) = time_opt {
				debug!("{0}", time.sec);
			}
			
            drop(context.state.write(MenuState {
				alpha: 0.,
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

            widget! {
                (#{key} nav_content_box [
                    (#{"stars"} stars::stars)
                    (#{"margin"} content_box: {margin} | {WidgetAlpha(state.alpha)} [
                        (#{"v-box"} vertical_box [
                            (#{"title"} text_paper: {title})
                            (#{"new_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                                id: "new_game".to_string(),
                                label: "New Game".to_string(),
                            }})
                            (#{"continue_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                                id: "continue".to_string(),
                                label: "Continue".to_string(),
                            }})
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

fn get_time() -> Option<Time> {
    if let Some(storage) = get_storage() {
        let sto_res = storage.get_item(TIME_STORAGE);
        if let Ok(sto_opt) = sto_res {
            if let Some(sto) = sto_opt {
                let obj_res = serde_json::from_str::<Time>(sto.as_str());
                if let Ok(obj) = obj_res {
					return Some(obj);
                }
            }
        }
    }
    None
}

fn get_storage() -> Option<web_sys::Storage> {
    if let Some(window) = window() {
        if let Ok(storage_opt) = window.local_storage() {
            return storage_opt;
        }
    }
    None
}