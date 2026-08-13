use app::views::Example;
use gpui::WindowOptions;
use gpui::prelude::*;

fn main() {
    gpui_platform::application().run(move |cx| {
        // This must be called before using any gpui-base features.
        gpui_base::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |_window, cx| cx.new(|_| Example))
                .expect("Failed to open window");
        })
        .detach();
    });
}
