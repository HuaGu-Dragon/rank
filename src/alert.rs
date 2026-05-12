use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, ParentElement,
    Render, Styled, Window, div,
};
use gpui_component::{
    IconName, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    dialog::{AlertDialog, DialogFooter, DialogHeader, DialogTitle},
    notification::NotificationType,
    v_flex,
};

use crate::form;

pub struct AlertView {
    focus_handle: FocusHandle,
    form: Entity<form::DataForm>,
}

impl AlertView {
    pub fn view(form: Entity<form::DataForm>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(form, window, cx))
    }

    pub fn new(form: Entity<form::DataForm>, _window: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            form,
        }
    }
}

impl Focusable for AlertView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AlertView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let form = self.form.clone();

        div().track_focus(&self.focus_handle).child(
            v_flex().gap_6().child(
                AlertDialog::new(cx)
                    .trigger(
                        Button::new("show-alert")
                            .outline()
                            .icon(IconName::Plus)
                            .label("Add"),
                    )
                    .content(move |content, _window, _cx| {
                        content
                            .child(
                                DialogHeader::new()
                                    .child(DialogTitle::new().child("Create a process")),
                            )
                            .child(form.clone())
                            .child(
                                DialogFooter::new()
                                    .h_flex()
                                    .justify_end()
                                    .gap_3()
                                    .child(
                                        Button::new("cancel").outline().label("Cancel").on_click(
                                            |_, window, cx| {
                                                window.close_dialog(cx);
                                            },
                                        ),
                                    )
                                    .child(Button::new("add").label("Add").primary().on_click({
                                        let form = form.clone();
                                        move |_, window, cx| {
                                            let res = form.update(cx, |f, cx| f.submit(cx));
                                            if let Err(err) = res {
                                                window.push_notification(
                                                    (NotificationType::Error, err),
                                                    cx,
                                                );
                                            } else {
                                                window.close_dialog(cx);
                                            }
                                        }
                                    })),
                            )
                    }),
            ),
        )
    }
}
