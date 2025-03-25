use crate::constants::{HIGHLIGHED_PAIR, REGULAR_PAIR};
use ncurses::{addstr, attroff, attron, mv, COLOR_PAIR};

type Id = usize;

#[derive(Default)]
pub struct UI {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl UI {
    pub fn new() -> Self {
        UI {
            list_curr: None,
            row: 0,
            col: 0,
        }
    }
    pub fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn end(&mut self) {}

    pub fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);

        attron(COLOR_PAIR(pair));
        addstr(text).expect("Something wrong with addstr in label method");
        attroff(COLOR_PAIR(pair));

        self.row += 1;
    }

    pub fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested lists are not allowed");
        self.list_curr = Some(id);
    }

    pub fn list_element(&mut self, label: &str, id: Id) -> bool {
        let id_curr = self
            .list_curr
            .expect("Not allowed to create list element outside of lists");

        let pair = {
            if id_curr == id {
                HIGHLIGHED_PAIR
            } else {
                REGULAR_PAIR
            }
        };

        self.label(label, pair);

        false
    }

    pub fn end_list(&mut self) {
        self.list_curr = None;
    }
}
