use gpui::*;
use gpui_component::{
    Root, TitleBar,
    button::{Button, ButtonVariants},
    h_flex,
    menu::AppMenuBar,
    v_flex,
};

mod menu;
mod themes;

pub struct Example {
    menu: Entity<AppMenuBar>,
}

impl Render for Example {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                // Render custom title bar on top of Root view.
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .pr_2()
                        .justify_between()
                        .child(self.menu.clone()),
                ),
            )
            .child(
                div()
                    .id("window-body")
                    .p_5()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .child("Hello, World!")
                    .child(
                        Button::new("ok")
                            .primary()
                            .label("Let's Go!")
                            .on_click(|_, _, _| println!("Clicked!")),
                    ),
            )
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        themes::init(cx);

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                // Setup GPUI to use custom title bar
                titlebar: Some(TitlebarOptions {
                    title: Some("rank".into()),
                    ..TitleBar::title_bar_options()
                }),
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                let menu = menu::init("rank", cx);
                let view = cx.new(|_| Example { menu });
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
