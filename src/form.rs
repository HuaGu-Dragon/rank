use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, ParentElement, Render, SharedString,
    Styled, WeakEntity, Window,
};
use gpui_component::{
    WindowExt,
    button::{Button, ButtonVariants},
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
    notification::NotificationType,
    v_flex,
};

use crate::table::{self, Proc, RESOURCE_COUNT};

pub struct DataForm {
    focus_handle: FocusHandle,
    parent: WeakEntity<table::TableView>,
    name_input: Entity<InputState>,
    allocation_inputs: [Entity<InputState>; RESOURCE_COUNT],
    max_inputs: [Entity<InputState>; RESOURCE_COUNT],
}

pub struct ResForm {
    focus_handle: FocusHandle,
    parent: WeakEntity<table::TableView>,
    res_inputs: [Entity<InputState>; RESOURCE_COUNT],
}

impl DataForm {
    pub fn view(
        parent: WeakEntity<table::TableView>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(parent, window, cx))
    }

    fn new(
        parent: WeakEntity<table::TableView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_input = cx.new(|cx| InputState::new(window, cx));

        let allocation_inputs =
            std::array::from_fn(|_| cx.new(|cx| InputState::new(window, cx).placeholder("0")));
        let max_inputs =
            std::array::from_fn(|_| cx.new(|cx| InputState::new(window, cx).placeholder("0")));

        Self {
            focus_handle: cx.focus_handle(),
            parent,
            name_input,
            allocation_inputs,
            max_inputs,
        }
    }

    fn parse_inputs(
        cx: &Context<Self>,
        inputs: &[Entity<InputState>; RESOURCE_COUNT],
    ) -> Result<[usize; RESOURCE_COUNT], &'static str> {
        let mut result = [0; RESOURCE_COUNT];
        for i in 0..RESOURCE_COUNT {
            let val = inputs[i].read(cx).value();
            if val.is_empty() {
                return Err("Inputs cannot be empty");
            }
            match val.parse::<usize>() {
                Ok(v) => result[i] = v,
                Err(_) => return Err("Inputs must be positive numbers"),
            }
        }
        Ok(result)
    }

    pub fn submit(&mut self, cx: &mut Context<Self>) -> Result<(), SharedString> {
        let name = self.name_input.read(cx).value().to_string();
        if name.is_empty() {
            return Err("Process name cannot be empty".into());
        }

        let allocation = match Self::parse_inputs(cx, &self.allocation_inputs) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Allocation: {}", e).into());
            }
        };

        let max = match Self::parse_inputs(cx, &self.max_inputs) {
            Ok(v) => v,
            Err(e) => {
                return Err(format!("Max: {}", e).into());
            }
        };

        for i in 0..RESOURCE_COUNT {
            if allocation[i] > max[i] {
                return Err("Allocation cannot be greater than Max".into());
            }
        }

        let need = std::array::from_fn(|i| max[i] - allocation[i]);

        match self.parent.update(cx, |this, cx| {
            this.push_proc(
                Proc {
                    name,
                    allocation,
                    max,
                    need,
                },
                cx,
            )
        }) {
            Ok(e) => e,
            Err(_) => Ok(()),
        }
    }
}

impl ResForm {
    pub fn view(
        parent: WeakEntity<table::TableView>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(parent, window, cx))
    }

    fn new(
        parent: WeakEntity<table::TableView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let res_inputs =
            std::array::from_fn(|_| cx.new(|cx| InputState::new(window, cx).placeholder("0")));

        Self {
            focus_handle: cx.focus_handle(),
            parent,
            res_inputs,
        }
    }

    fn parse_inputs(
        cx: &Context<Self>,
        inputs: &[Entity<InputState>; RESOURCE_COUNT],
    ) -> Result<[usize; RESOURCE_COUNT], &'static str> {
        let mut result = [0; RESOURCE_COUNT];
        for i in 0..RESOURCE_COUNT {
            let val = inputs[i].read(cx).value();
            if val.is_empty() {
                return Err("Inputs cannot be empty");
            }
            match val.parse::<usize>() {
                Ok(v) => result[i] = v,
                Err(_) => return Err("Inputs must be positive numbers"),
            }
        }
        Ok(result)
    }

    pub fn submit(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Result<[usize; RESOURCE_COUNT], SharedString> {
        match Self::parse_inputs(cx, &self.res_inputs) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl Focusable for DataForm {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Focusable for ResForm {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DataForm {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_form()
            .child(
                field()
                    .label("Process Name")
                    .child(Input::new(&self.name_input)),
            )
            .child(
                field().label("Allocation").child(
                    h_flex()
                        .gap_2()
                        .children(self.allocation_inputs.iter().map(Input::new)),
                ),
            )
            .child(
                field().label("Max").child(
                    h_flex()
                        .gap_2()
                        .children(self.max_inputs.iter().map(Input::new)),
                ),
            )
    }
}

impl Render for ResForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let parent = self.parent.clone();

        v_form()
            .h_full()
            .child(
                field().label("Allocation").child(
                    v_flex()
                        .gap_2()
                        .children(self.res_inputs.iter().map(Input::new)),
                ),
            )
            .child(
                field().label_indent(false).child(
                    Button::new("gl_submit")
                        .label("Modify")
                        .primary()
                        .on_click(cx.listener(move |this, _, window, cx| {
                            let res = this.submit(cx);
                            match res {
                                Ok(data) => {
                                    let _ = parent.update(cx, |this, cx| {
                                        match this.modify_global_res(data, cx) {
                                            Ok(_) => {
                                                this.form_popover_open = false;
                                                cx.notify();
                                            }
                                            Err(e) => window.push_notification(
                                                (NotificationType::Error, e),
                                                cx,
                                            ),
                                        }
                                    });
                                }
                                Err(e) => {
                                    window.push_notification((NotificationType::Error, e), cx)
                                }
                            }
                        })),
                ),
            )
    }
}
