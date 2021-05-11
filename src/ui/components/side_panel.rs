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
    
    widget! {
        (#{context.key} content_box [
            (#{"bkg"} paper: {bkg})
        ])
    }
}