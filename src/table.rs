use gpui::{
    AnyElement, App, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString,
    Styled, TextAlign, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{
    ActiveTheme, IconName, Root, Sizable, Size, StyleSized, StyledExt,
    button::Button,
    h_flex,
    label::Label,
    popover::Popover,
    resizable::{h_resizable, resizable_panel},
    scroll::ScrollableElement,
    stepper::{Stepper, StepperItem},
    table::{Column, ColumnFixed, ColumnGroup, DataTable, TableDelegate, TableState},
    v_flex,
};

use rand::RngExt;

use crate::{
    alert, algo,
    form::{self, ResForm},
};

macro_rules! pass_nproc {
    ($mac:ident) => {
        $mac! { 6 }
    };
}

macro_rules! literal_identity_macro {
    ($nproc:literal) => {
        $nproc
    };
}

pub const RESOURCE_COUNT: usize = pass_nproc!(literal_identity_macro);

#[derive(Clone)]
pub struct Data {
    pub id: usize,
    name: String,
    pub allocation: [usize; RESOURCE_COUNT],
    max: [usize; RESOURCE_COUNT],
    pub need: [usize; RESOURCE_COUNT],
    finish: bool,
}

#[derive(Clone)]
pub struct Proc {
    pub name: String,
    pub allocation: [usize; RESOURCE_COUNT],
    pub max: [usize; RESOURCE_COUNT],
    pub need: [usize; RESOURCE_COUNT],
}

impl Proc {
    pub fn random_data(
        id: usize,
        available: &[usize; RESOURCE_COUNT],
        total: &[usize; RESOURCE_COUNT],
    ) -> Self {
        let mut rng = rand::rng();

        let mut process_max = [0; RESOURCE_COUNT];
        let mut allocation = [0; RESOURCE_COUNT];
        let mut need = [0; RESOURCE_COUNT];

        for i in 0..RESOURCE_COUNT {
            process_max[i] = if total[i] > 0 {
                rng.random_range(0..=total[i])
            } else {
                0
            };

            let max_alloc = std::cmp::min(process_max[i], available[i]);
            allocation[i] = if max_alloc > 0 {
                rng.random_range(0..=max_alloc)
            } else {
                0
            };

            need[i] = process_max[i] - allocation[i];
        }

        Self {
            name: format!("P{id}"),
            allocation,
            max: process_max,
            need,
        }
    }
}

struct Table {
    data: Vec<Data>,
    columns: Vec<Column>,
    global_available: [usize; RESOURCE_COUNT],
    total_resources: [usize; RESOURCE_COUNT],
    finished_count: usize,
    safe_sequence: Vec<Data>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            data: vec![],
            global_available: Default::default(),
            total_resources: Default::default(),
            finished_count: 0,
            safe_sequence: vec![],
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
            .find(|p| p.id == self.safe_sequence[self.finished_count].id)
        {
            p.finish = true;
            self.finished_count += 1;
            (0..RESOURCE_COUNT).for_each(|i| self.global_available[i] += p.allocation[i]);
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

    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: gpui_component::table::ColumnSort,
        _window: &mut Window,
        _cx: &mut Context<TableState<Self>>,
    ) {
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            k if k.starts_with("allocation") => {
                let idx: usize = k.trim_start_matches("allocation_").parse().unwrap();
                match sort {
                    gpui_component::table::ColumnSort::Default => {
                        self.data.sort_unstable_by_key(|a| a.id)
                    }
                    gpui_component::table::ColumnSort::Ascending => self
                        .data
                        .sort_by(|a, b| a.allocation[idx].cmp(&b.allocation[idx])),
                    gpui_component::table::ColumnSort::Descending => self
                        .data
                        .sort_by(|a, b| b.allocation[idx].cmp(&a.allocation[idx])),
                }
            }
            k if k.starts_with("max") => {
                let idx: usize = k.trim_start_matches("max_").parse().unwrap();
                match sort {
                    gpui_component::table::ColumnSort::Default => {
                        self.data.sort_unstable_by_key(|a| a.id)
                    }
                    gpui_component::table::ColumnSort::Ascending => {
                        self.data.sort_by(|a, b| a.max[idx].cmp(&b.max[idx]))
                    }
                    gpui_component::table::ColumnSort::Descending => {
                        self.data.sort_by(|a, b| b.max[idx].cmp(&a.max[idx]))
                    }
                }
            }
            k if k.starts_with("need") => {
                let idx: usize = k.trim_start_matches("need_").parse().unwrap();
                match sort {
                    gpui_component::table::ColumnSort::Default => {
                        self.data.sort_unstable_by_key(|a| a.id)
                    }
                    gpui_component::table::ColumnSort::Ascending => {
                        self.data.sort_by(|a, b| a.need[idx].cmp(&b.need[idx]))
                    }
                    gpui_component::table::ColumnSort::Descending => {
                        self.data.sort_by(|a, b| b.need[idx].cmp(&a.need[idx]))
                    }
                }
            }
            _ => {}
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
    alert: Entity<alert::AlertView>,
    form: Entity<ResForm>,
    pub form_popover_open: bool,
    pub run: bool,
}

impl TableView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = Table::new();
        let table = cx.new(|cx| TableState::new(delegate, window, cx));

        let form = form::DataForm::view(cx.entity(), window, cx);

        let alert = alert::AlertView::view(form, window, cx);

        let form = ResForm::view(cx.entity(), window, cx);

        Self {
            table,
            alert,
            form_popover_open: false,
            form,
            run: false,
        }
    }

    pub fn push_proc(&mut self, data: Proc, cx: &mut Context<Self>) -> Result<(), SharedString> {
        self.table.update(cx, |table, cx| {
            let delegate = table.delegate_mut();

            for i in 0..RESOURCE_COUNT {
                if data.allocation[i] > delegate.global_available[i] {
                    return Err("Not enough resources".into());
                }
            }

            let cur = std::array::from_fn(|i| delegate.global_available[i] - data.allocation[i]);
            let data = Data {
                id: delegate.data.len(),
                name: data.name,
                allocation: data.allocation,
                max: data.max,
                need: data.need,
                finish: false,
            };

            // TODO: use tmp buffer to avoid cloning
            let mut safe = delegate.safe_sequence.clone();
            safe.push(data.clone());

            if algo::check_safety(&mut safe, &cur) {
                delegate.safe_sequence = safe;
                delegate.global_available = cur;
                delegate.data.push(data);
            } else {
                return Err("Deadlock detected! Could not allocate resources".into());
            }

            cx.notify();

            Ok(())
        })
    }

    pub fn modify_global_res(&mut self, res: [usize; RESOURCE_COUNT], cx: &mut Context<Self>) {
        self.table.update(cx, |table, cx| {
            let table = table.delegate_mut();

            table.total_resources = res;
            table.global_available = res;
            cx.notify();
        });
    }

    fn on_step(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.run = true;
        self.table.update(cx, |table, cx| {
            table.delegate_mut().step();
            cx.notify();
        });
    }

    fn on_reset(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.run = false;
        self.table.update(cx, |table, cx| {
            table.delegate_mut().reset();
            cx.notify();
        });
    }
}

impl Render for TableView {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let table = self.table.read(cx).delegate();
        let global_available = table.global_available;
        let items = &table.safe_sequence;
        let finished = table.finished_count;

        h_resizable("layout")
            .child(
                resizable_panel()
                    .v_flex()
                    .size_full()
                    .p_6()
                    .text_lg()
                    .gap_6()
                    .child(
                        h_flex()
                            .gap_4()
                            .items_center()
                            .child(
                                Button::new("step_btn")
                                    .icon(IconName::Play)
                                    .label("Step")
                                    .on_click(cx.listener(|this, _ev, window, cx| {
                                        this.on_step(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("reset_btn")
                                    .icon(IconName::Undo)
                                    .label("Reset")
                                    .on_click(cx.listener(|this, _ev, window, cx| {
                                        this.on_reset(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("rand")
                                    .icon(IconName::Cpu)
                                    .label("Random Gen")
                                    .on_click(cx.listener(|this, _ev, _window, cx| {
                                        let table = this.table.read(cx).delegate();
                                        let total_resources = table.total_resources;
                                        let id = table.data.len();
                                        let global_available = table.global_available;
                                        let _ = this.push_proc(
                                            Proc::random_data(
                                                id,
                                                &global_available,
                                                &total_resources,
                                            ),
                                            cx,
                                        );
                                    })),
                            )
                            .child(
                                div()
                                    .child(self.alert.clone())
                                    .when(self.run, |d| d.invisible()),
                            )
                            .child(
                                div()
                                    .ml_6()
                                    .text_xl()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .child(format!("Global Available: {:?}", global_available)),
                            )
                            .child(
                                div()
                                    .child(
                                        Popover::new("global_available")
                                            .trigger(
                                                Button::new("global_available_btn")
                                                    .outline()
                                                    .icon(IconName::Settings)
                                                    .label("Modify"),
                                            )
                                            .open(self.form_popover_open)
                                            .on_open_change(cx.listener(|this, open, _, cx| {
                                                this.form_popover_open = *open;
                                                cx.notify();
                                            }))
                                            .child(self.form.clone()),
                                    )
                                    .when(self.run, |d| d.invisible()),
                            ),
                    )
                    .child(
                        DataTable::new(&self.table)
                            .stripe(true)
                            .scrollbar_visible(true, true)
                            .with_size(Size::Large),
                    )
                    .children(dialog_layer),
            )
            .child(
                resizable_panel().size_range(px(100.)..px(150.)).child(
                    div().overflow_y_scrollbar().size_full().p_10().child(
                        Stepper::new("step")
                            .vertical()
                            .selected_index(finished.saturating_sub(1))
                            .items(items.iter().cloned().map(|p| {
                                StepperItem::new()
                                    .child(v_flex().items_center().child(p.name).h_40())
                                    .icon(IconName::Cpu)
                            })),
                    ),
                ),
            )
    }
}
