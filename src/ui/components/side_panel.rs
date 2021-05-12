use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}

pub fn side_panel(context: WidgetContext) -> WidgetNode {
	let bkg = Props::new(PaperProps { 
        frame: None, 
        ..Default::default() 
    });

	let c_box = ContentBoxProps {
        transform: Transform {
			align: Vec2 { x: 0.9, y: 0. },
            ..Default::default()
        },
        ..Default::default()
    };

    widget! {
        (#{context.key} content_box: {c_box} [
            (#{"bkg"} paper: {bkg})
		])
    }
}