use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};
use web_sys::window;
use crate::{
    resources::time::TIME_STORAGE, 
    ui::components::menu_btn,
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

fn use_menu(context: &mut WidgetContext) {
    context.life_cycle.mount(|context| {
        let sec_opt = sto_utils::get::<u32>(TIME_STORAGE);
        drop(context.state.write(MenuState {
            alpha: 0.,
            sec: sec_opt,
        }));
    });

    context.life_cycle.change(|context| {
        let mut state = context.state.read_cloned_or_default::<MenuState>();
        if state.alpha < 1. {
            state.alpha += 1. / FRAMES;
        }
        drop(context.state.write(state));
    });
}

#[pre_hooks(use_menu)]
fn menu_comp(mut context: WidgetContext) -> WidgetNode {
    let WidgetContext {
        props,
        state,
        ..
    } = context;

    if let Ok(state) = state.read::<MenuState>() {

        let left_margin = left_margin();
        
        let bkg = PaperProps { 
            frame: None, 
            ..Default::default() 
        };
        let margin = Props::new(ContentBoxItemLayout {
            margin: Rect {
                top: 200.,
                bottom: 200.,
                left: left_margin,
                right: 0.
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
        
        let size_btn = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(100.),
            height: SizeBoxSizeValue::Exact(30.),
            ..Default::default()
        });
        if let Some(sec) = state.sec
        {
            time_txt = format!("Time played : {0}s", sec);
            continue_btn = widget! {
                (#{"box-btn-cont"} content_box [
                    (#{"size-btn-cont"} size_box: {&size_btn} {
                        content = (#{"continue_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                            id: "continue".to_string(),
                            label: "Continue".to_string(),
                        }})
                    })
                ])
            };
        }
        let time = Props::new(TextPaperProps {
            text: time_txt.to_owned(),
            width: TextBoxSizeValue::Fill,
            height: TextBoxSizeValue::Fill,
            use_main_color: true,
            ..Default::default()
        }).with(ThemedWidgetProps {
            color: ThemeColor::Secondary,
            ..Default::default()
        });

        widget! {
            (#{"bkg"} paper: {bkg} [
                (#{"margin"} nav_content_box: {margin} | {WidgetAlpha(state.alpha)} [
                    (#{"v-box"} vertical_box [
                        (#{"text"} text_paper: {title})
                        (#{"time"} text_paper: {time})
                        (#{"v-box-btns"} vertical_box [
                                {continue_btn}
                                (#{"box-btn-new"} content_box [
                                (#{"size-btn-new"} size_box: {&size_btn} {
                                    content = (#{"new_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                                        id: "new_game".to_string(),
                                        label: "New Game".to_string(),
                                    }})
                                })
                            ])
                        ])
                    ])
                ])
            ])
        }
    } else {
        widget!{()}
    }
}

pub fn menu(context: WidgetContext) -> WidgetNode {
    widget! {
        (#{context.key} menu_comp : { MenuTextProps { 
            title: "Menu".to_owned()
        }})
    }
}

fn left_margin() -> f32 {
    let res = 100.;
    if let Some(window) = window() {
		let w_res = window.inner_width();
        if let Ok(w_js) = w_res {
            if let Some(w) = w_js.as_f64() {
                return w as f32 / 4.
            }
        }
	}
    res
}