use oxygengine::prelude::*;

use crate::components::text_ani::TextAni;

pub struct TextAniSystem;

impl<'s> System<'s> for TextAniSystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
        WriteStorage<'s, TextAni>,
        WriteStorage<'s, CompositeUiElement>,
    );

    fn run(
        &mut self,
        (lifecycle,  mut text_anis, mut ui_elements): Self::SystemData,
    ) {
        let delta_time = lifecycle.delta_time_seconds();
		for (mut text_ani, ui_element) in (&mut text_anis, &mut ui_elements).join() {
            if text_ani.curr_text < text_ani.texts.len() {
                text_ani.phase += delta_time;
                if text_ani.phase > text_ani.interval {
                    text_ani.phase = 0.;
                    //Get current text
                    let text_opt = text_ani.texts.get(text_ani.curr_text);
                    if let Some(text) = text_opt {
                        let text = self.get_text(text.clone(), text_ani);
                        ui_element.children.push(text);
                    }
                    text_ani.curr_text += 1;
                }
            }
		}
    }
}

impl TextAniSystem {
    fn get_text(&mut self,
		data: String,
        text_ani: &mut TextAni) -> CompositeUiElement
	{
        let mut text: Text = Text::default();
		text.text = data.into();
		text.color = text_ani.color;
		let mut ui_ele = CompositeUiElement::default();
		ui_ele.theme = Some("text_intro".into());
        ui_ele.element_type = UiElementType::Text(text);
        ui_ele.right_anchor = UiValue::Value(1.);
        ui_ele.top_anchor = UiValue::Value(2.5 * text_ani.curr_text as Scalar);
        ui_ele
	}
}