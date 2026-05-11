use gpui::{App, AppContext, Context, Entity, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    Sizable, Size,
    button::Button,
    h_flex,
    table::{Column, ColumnGroup, DataTable, TableDelegate, TableState},
    v_flex,
};

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

const RESOURCE_COUNT: usize = pass_nproc!(literal_identity_macro);

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
                    Column::new("id", "Process").width(80.),
                    Column::new("name", "Name").width(150.),
                ]);

                macro_rules! push_resource_columns {
                    ($nproc:literal) => {
                        seq_macro::seq!(NUM in 0..$nproc {
                            cols.push(Column::new(concat!("allocation_", stringify!(NUM)), stringify!(NUM)).width(50.).sortable());
                        });
                        seq_macro::seq!(NUM in 0..$nproc {
                            cols.push(Column::new(concat!("max_", stringify!(NUM)), stringify!(NUM)).width(50.).sortable());
                        });
                        seq_macro::seq!(NUM in 0..$nproc {
                            cols.push(Column::new(concat!("need_", stringify!(NUM)), stringify!(NUM)).width(50.).sortable());
                        });
                    }
                }

                pass_nproc!(push_resource_columns);

                cols.push(Column::new("state", "state").width(100.));
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
        _cx: &mut gpui::Context<gpui_component::table::TableState<Self>>,
    ) -> impl gpui::IntoElement {
        let row = &self.data[row_ix];
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "id" => row.id.to_string(),
            "name" => row.name.clone(),
            k if k.starts_with("allocation") => {
                let idx: usize = k.trim_start_matches("allocation_").parse().unwrap();
                row.allocation[idx].to_string()
            }
            k if k.starts_with("max") => {
                let idx: usize = k.trim_start_matches("max_").parse().unwrap();
                row.max[idx].to_string()
            }
            k if k.starts_with("need") => {
                let idx: usize = k.trim_start_matches("need_").parse().unwrap();
                row.need[idx].to_string()
            }
            "state" => {
                if row.finish {
                    "Finished".to_string()
                } else {
                    "Running".to_string()
                }
            }
            _ => "".to_string(),
        }
    }
}

pub struct TableView {
    table: Entity<TableState<Table>>,
}

impl TableView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let delegate = Table::new();
        let table = cx.new(|cx| TableState::new(delegate, window, cx));

        Self { table }
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
    }
}
