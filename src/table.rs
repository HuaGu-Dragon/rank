use gpui::{
    AnyElement, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled,
    TextAlign, Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    ActiveTheme, Sizable, Size, StyleSized, StyledExt,
    button::Button,
    h_flex,
    label::Label,
    table::{Column, ColumnFixed, ColumnGroup, DataTable, TableDelegate, TableState},
    v_flex,
};

use crate::form;

macro_rules! pass_nproc {
    ($mac:ident) => {
        $mac! { 3 }
    };
}

macro_rules! literal_identity_macro {
    ($nproc:literal) => {
        $nproc
    };
}

pub const RESOURCE_COUNT: usize = pass_nproc!(literal_identity_macro);

struct Data {
    id: usize,
    name: String,
    allocation: [usize; RESOURCE_COUNT],
    max: [usize; RESOURCE_COUNT],
    need: [usize; RESOURCE_COUNT],
    finish: bool,
}

struct Table {
    data: Vec<Data>,
    columns: Vec<Column>,
    global_available: [usize; RESOURCE_COUNT],
    step_index: usize,
    finished_count: usize,
}

impl Table {
    pub fn new() -> Self {
        let data = vec![
            Data {
                id: 0,
                name: "P0".to_string(),
                allocation: [0, 1, 0],
                max: [7, 5, 3],
                need: [7, 4, 3],
                finish: false,
            },
            Data {
                id: 1,
                name: "P1".to_string(),
                allocation: [2, 0, 0],
                max: [3, 2, 2],
                need: [1, 2, 2],
                finish: false,
            },
            Data {
                id: 2,
                name: "P2".to_string(),
                allocation: [3, 0, 2],
                max: [9, 0, 2],
                need: [6, 0, 2],
                finish: false,
            },
            Data {
                id: 3,
                name: "P3".to_string(),
                allocation: [2, 1, 1],
                max: [2, 2, 2],
                need: [0, 1, 1],
                finish: false,
            },
            Data {
                id: 4,
                name: "P4".to_string(),
                allocation: [0, 0, 2],
                max: [4, 3, 3],
                need: [4, 3, 1],
                finish: false,
            },
        ];

        Self {
            data,
            global_available: [3, 3, 2],
            step_index: 0,
            finished_count: 0,
            columns: {
                let mut cols = Vec::with_capacity(2 + 3 * RESOURCE_COUNT + 1);

                cols.extend([
                    Column::new("id", "Process")
                        .fixed(ColumnFixed::Left)
                        .text_center()
                        .width(80.),
                    Column::new("name", "Name")
                        .fixed(ColumnFixed::Left)
                        .resizable(true)
                        .width(150.),
                ]);

                macro_rules! push_resource_columns {
                    ($nproc:literal) => {
                        seq_macro::seq!(NUM in 0..$nproc {
                            cols.push(Column::new(concat!("allocation_", stringify!(NUM)), stringify!(NUM)).width(50.).text_right().p_0().sortable());
                        });
                        seq_macro::seq!(NUM in 0..$nproc {
                            cols.push(Column::new(concat!("max_", stringify!(NUM)), stringify!(NUM)).width(50.).text_right().p_0().sortable());
                        });
                        seq_macro::seq!(NUM in 0..$nproc {
                            cols.push(Column::new(concat!("need_", stringify!(NUM)), stringify!(NUM)).width(50.).text_right().p_0().sortable());
                        });
                    }
                }

                pass_nproc!(push_resource_columns);

                cols.push(Column::new("state", "state").width(100.).text_center());
                cols
            },
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn step(&mut self) {
        if self.finished_count == self.data.len() {
            return;
        }

        if let Some(p) = self
            .data
            .iter_mut()
            .find(|p| !p.finish && p.need <= self.global_available)
        {
            for j in 0..RESOURCE_COUNT {
                self.global_available[j] += p.allocation[j];
            }
            self.step_index += 1;
            p.finish = true;
        }
    }

    fn render_value_cell(&self, col: &Column, val: usize, idx: usize, cx: &mut App) -> AnyElement {
        let this = div()
            .h_full()
            .table_cell_size(Size::Large)
            .child(format!("{val}"));

        let this = if val > self.global_available[idx] {
            this.text_color(cx.theme().red)
                .bg(cx.theme().red_light.alpha(0.05))
        } else {
            this.text_color(cx.theme().green)
                .bg(cx.theme().green_light.alpha(0.05))
        };

        this.when(col.align == TextAlign::Right, |this| {
            this.h_flex().justify_end()
        })
        .into_any_element()
    }
}

impl TableDelegate for Table {
    fn columns_count(&self, _cx: &gpui::App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &gpui::App) -> usize {
        self.data.len()
    }

    fn column(&self, col_ix: usize, _cx: &gpui::App) -> Column {
        self.columns[col_ix].clone()
    }

    fn group_headers(&self, _cx: &App) -> Option<Vec<Vec<ColumnGroup>>> {
        Some(vec![vec![
            ColumnGroup::new("Process Info", 2),
            ColumnGroup::new("Allocation", RESOURCE_COUNT),
            ColumnGroup::new("Max", RESOURCE_COUNT),
            ColumnGroup::new("Need", RESOURCE_COUNT),
        ]])
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<gpui_component::table::TableState<Self>>,
    ) -> impl gpui::IntoElement {
        let data = &self.data[row_ix];
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "id" => div()
                .child(data.id.to_string())
                .when(col.align == TextAlign::Center, |this| this.text_center())
                .into_any_element(),
            "name" => data.name.clone().into_any_element(),
            k if k.starts_with("allocation") => {
                let idx: usize = k.trim_start_matches("allocation_").parse().unwrap();
                self.render_value_cell(col, data.allocation[idx], idx, cx)
            }
            k if k.starts_with("max") => {
                let idx: usize = k.trim_start_matches("max_").parse().unwrap();
                self.render_value_cell(col, data.max[idx], idx, cx)
            }
            k if k.starts_with("need") => {
                let idx: usize = k.trim_start_matches("need_").parse().unwrap();
                self.render_value_cell(col, data.need[idx], idx, cx)
            }
            "state" => {
                if data.finish {
                    Label::new("Finish")
                        .text_center()
                        .text_color(cx.theme().green)
                        .into_any_element()
                } else {
                    Label::new("Running")
                        .text_center()
                        .text_color(cx.theme().info)
                        .into_any_element()
                }
            }
            _ => "".to_string().into_any_element(),
        }
    }

    fn render_th(
        &mut self,
        col_ix: usize,
        _window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let col = self.column(col_ix, cx);

        div()
            .child(col.name.clone())
            .when(col_ix >= 2, |this| this.table_cell_size(Size::Large))
            .when(col.align == TextAlign::Center, |this| {
                this.h_flex().w_full().justify_center()
            })
            .when(col.align == TextAlign::Right, |this| {
                this.h_flex().w_full().justify_end()
            })
    }
}

pub struct TableView {
    table: Entity<TableState<Table>>,
    form: Entity<form::FormView>,
}

impl TableView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = Table::new();
        let table = cx.new(|cx| TableState::new(delegate, window, cx));

        let form = form::FormView::view(window, cx);

        cx.subscribe(&form, |this, _emitter, ev: &form::FormEvent, cx| match ev {
            form::FormEvent::Submit {
                name,
                allocation,
                max,
                need,
            } => {
                this.table.update(cx, |table, cx| {
                    let delegate = table.delegate_mut();
                    delegate.data.push(Data {
                        id: delegate.data.len(),
                        name: name.clone(),
                        allocation: *allocation,
                        max: *max,
                        need: *need,
                        finish: false,
                    });
                    cx.notify();
                });
            }
            form::FormEvent::Cancel => {
                println!("Form cancelled");
            }
            form::FormEvent::Invalid(reason) => {
                println!("Invalid form input: {}", reason);
            }
        })
        .detach();

        Self { table, form }
    }

    fn on_step(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.table.update(cx, |table, cx| {
            table.delegate_mut().step();
            cx.notify();
        });
    }

    fn on_reset(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.table.update(cx, |table, cx| {
            table.delegate_mut().reset();
            cx.notify();
        });
    }
}

impl Render for TableView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let global_available = self.table.read(cx).delegate().global_available;

        v_flex()
            .size_full()
            .p_6()
            .text_lg()
            .gap_6()
            .child(
                h_flex()
                    .gap_4()
                    .items_center()
                    .child(Button::new("step_btn").label("Step").on_click(cx.listener(
                        |this, _ev, window, cx| {
                            this.on_step(window, cx);
                        },
                    )))
                    .child(
                        Button::new("reset_btn")
                            .label("Reset")
                            .on_click(cx.listener(|this, _ev, window, cx| {
                                this.on_reset(window, cx);
                            })),
                    )
                    .child(
                        div()
                            .ml_6()
                            .text_xl()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(format!("Global Available: {:?}", global_available)),
                    ),
            )
            .child(
                DataTable::new(&self.table)
                    .stripe(true)
                    .scrollbar_visible(true, true)
                    .with_size(Size::Large),
            )
            .child(self.form.clone())
    }
}
