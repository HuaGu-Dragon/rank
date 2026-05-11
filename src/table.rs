use gpui::{App, AppContext, Context, Entity, ParentElement, Render, Styled, Window};
use gpui_component::{
    Sizable, Size,
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
    available: Option<[usize; RESOURCE_COUNT]>,
    finish: bool,
}

struct Table {
    data: Vec<Data>,
    columns: Vec<Column>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            data: vec![Data {
                id: 1,
                name: "process_01".to_string(),
                allocation: [0; RESOURCE_COUNT],
                max: [0; RESOURCE_COUNT],
                need: [0; RESOURCE_COUNT],
                available: None,
                finish: false,
            }],
            columns: {
                let mut cols = vec![
                    Column::new("id", "Process").width(80.),
                    Column::new("name", "Name").width(150.),
                ];

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
                        seq_macro::seq!(NUM in 0..$nproc {
                            cols.push(Column::new(concat!("available_", stringify!(NUM)), stringify!(NUM)).width(50.).sortable());
                        });
                    }
                }

                pass_nproc!(push_resource_columns);

                cols.push(Column::new("state", "state").width(100.));
                cols
            },
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
            ColumnGroup::new("Available", RESOURCE_COUNT),
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
            k if k.starts_with("available") => {
                if let Some(a) = row.available {
                    let idx: usize = k.trim_start_matches("available_").parse().unwrap();
                    a[idx].to_string()
                } else {
                    "".into()
                }
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
}

impl Render for TableView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex().size_full().text_sm().gap_4().child(
            DataTable::new(&self.table)
                .stripe(false)
                .scrollbar_visible(true, true)
                .with_size(Size::Large),
        )
    }
}
