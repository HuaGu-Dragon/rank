use gpui::{
    App, AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable, ParentElement, Render,
    Styled, Window,
};
use gpui_component::{
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
};

use crate::table::RESOURCE_COUNT;

pub enum FormEvent {
    Submit {
        name: String,
        allocation: [usize; RESOURCE_COUNT],
        max: [usize; RESOURCE_COUNT],
        need: [usize; RESOURCE_COUNT],
    },
    Invalid(String),
}

pub struct FormView {
    focus_handle: FocusHandle,
    name_input: Entity<InputState>,
    allocation_inputs: [Entity<InputState>; RESOURCE_COUNT],
    max_inputs: [Entity<InputState>; RESOURCE_COUNT],
    need_inputs: [Entity<InputState>; RESOURCE_COUNT],
}

impl FormView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name_input = cx.new(|cx| InputState::new(window, cx));

        let allocation_inputs =
            std::array::from_fn(|_| cx.new(|cx| InputState::new(window, cx).placeholder("0")));
        let max_inputs =
            std::array::from_fn(|_| cx.new(|cx| InputState::new(window, cx).placeholder("0")));
        let need_inputs =
            std::array::from_fn(|_| cx.new(|cx| InputState::new(window, cx).placeholder("0")));

        Self {
            focus_handle: cx.focus_handle(),
            name_input,
            allocation_inputs,
            max_inputs,
            need_inputs,
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

    pub fn submit(&mut self, cx: &mut Context<Self>) -> bool {
        let name = self.name_input.read(cx).value().to_string();
        if name.is_empty() {
            cx.emit(FormEvent::Invalid(
                "Process name cannot be empty".to_string(),
            ));
            return false;
        }

        let allocation = match Self::parse_inputs(cx, &self.allocation_inputs) {
            Ok(v) => v,
            Err(e) => {
                cx.emit(FormEvent::Invalid(format!("Allocation: {}", e)));
                return false;
            }
        };

        let max = match Self::parse_inputs(cx, &self.max_inputs) {
            Ok(v) => v,
            Err(e) => {
                cx.emit(FormEvent::Invalid(format!("Max: {}", e)));
                return false;
            }
        };

        let need = match Self::parse_inputs(cx, &self.need_inputs) {
            Ok(v) => v,
            Err(e) => {
                cx.emit(FormEvent::Invalid(format!("Need: {}", e)));
                return false;
            }
        };

        for i in 0..RESOURCE_COUNT {
            if need[i] > max[i] {
                cx.emit(FormEvent::Invalid(
                    "Need cannot be greater than Max".to_string(),
                ));
                return false;
            }
        }

        cx.emit(FormEvent::Submit {
            name,
            allocation,
            max,
            need,
        });

        true
    }
}

impl EventEmitter<FormEvent> for FormView {}

impl Focusable for FormView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for FormView {
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
            .child(
                field().label("Need").child(
                    h_flex()
                        .gap_2()
                        .children(self.need_inputs.iter().map(Input::new)),
                ),
            )
    }
}
