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
    
    let h_box = HorizontalBoxProps {
        transform: Transform {
            align: Vec2 { x: 0.9, y: 0. },
            ..Default::default()
        },
        ..Default::default()
    };

    widget! {
        (#{"h_box"} horizontal_box: {h_box} [
            (#{"bkg"} paper: {bkg})
        ])
    }
}