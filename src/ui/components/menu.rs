use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
    resources::time::TIME_STORAGE, 
    ui::components::menu_btn::{ menu_btn, MenuBtnProps },
    storage::sto_utils,
};

const FRAMES: Scalar = 5.;

#[derive(PropsData, Debug, Default, Clone, Serialize, Deserialize)]
pub struct MenuState {
	pub alpha: Scalar,
    pub sec: Option<u32>,
}

#[derive(PropsData, Default, Debug, Clone, Serialize, Deserialize)]
pub struct MenuTextProps {
    pub title: String,
}

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
pub fn menu(mut context: WidgetContext) -> WidgetNode {
    let WidgetContext {
        key,
        state,
        ..
    } = context;
    let title = TextPaperProps {
        text: "Menu".to_owned(),
        width: TextBoxSizeValue::Fill,
        height: TextBoxSizeValue::Fill,
        use_main_color: true,
        ..Default::default()
    };
    let mut time_txt = "No save".to_string();
    let mut continue_btn = widget! {()};
    if let Ok(state) = state.read::<MenuState>() {
        if let Some(sec) = state.sec
        {
            time_txt = format!("Time played : {0}s", sec);
            continue_btn = widget! {
                (#{"continue_btn"} menu_btn: { MenuBtnProps {
                    id: "continue".to_string(),
                    label: "Continue".to_string(),
                }})
            };
        }
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

	let mut alpha = 0.;
	if let Ok(state) = state.read::<MenuState>() {
		alpha = state.alpha;
	}
	widget! {
		(#{key} vertical_box | {WidgetAlpha(alpha)} [
			(#{"text"} text_paper: {title})
			(#{"time"} text_paper: {time})
			(#{"v_box"} nav_vertical_box: {NavJumpLooped} [
				{continue_btn}
				(space_box: {SpaceBoxProps::vertical(10.)})
				(#{"new_btn"} menu_btn: { MenuBtnProps {
					id: "new_game".to_string(),
					label: "New Game".to_string(),
				}})
			])
		])
	}
}