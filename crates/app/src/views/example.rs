use gpui::prelude::*;
use gpui::{Context, Window, div};
use gpui_base::StyledExt as _;
use ui::components::Button;

pub struct Example;

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .bg(ui::theme::background())
            .text_color(ui::theme::text())
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .child("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}
