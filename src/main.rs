use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use ncurses::*;

mod constants;
mod handler;
mod models;
mod ui;

use handler::{list_down, list_transfer, list_up, parse_item, remove_item, save_state};
use models::Status;
use ui::UI;

fn main() {
    let mut args = args();
    args.next().unwrap();

    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            eprintln!("Usage: todo-rust <file-path>");
            eprint!("ERROR: File path is not provided");
            process::exit(1);
        }
    };

    let mut todos: Vec<String> = vec![];
    let mut dones: Vec<String> = vec![];

    let mut todo_curr: usize = 0;
    let mut done_curr = 0;

    {
        let file = File::open(&file_path).unwrap();

        for (index, line) in BufReader::new(file).lines().enumerate() {
            match parse_item(&line.unwrap()) {
                Some((Status::Todo, title)) => todos.push(title.to_string()),
                Some((Status::Done, title)) => dones.push(title.to_string()),
                None => {
                    eprintln!("{}:{}: ill-formed item line", file_path, index + 1);
                    process::exit(1);
                }
            }
        }
    }

    initscr();
    noecho();

    // make cursor invisible
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // init colors for selected/default todos
    start_color();
    init_pair(constants::REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(constants::HIGHLIGHED_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut tab = Status::Todo;
    let mut ui = UI::new();

    while !quit {
        erase();

        ui.begin(0, 0);
        {
            match tab {
                Status::Todo => {
                    ui.label("[TODO]  DONE (use TAB to switch)", constants::REGULAR_PAIR);

                    // rendres todo items
                    ui.begin_list(todo_curr);
                    for (index, todo) in todos.iter().enumerate() {
                        ui.list_element(&format!("- [ ] {}", todo), index);
                    }
                    ui.end_list();
                }
                Status::Done => {
                    ui.label(" TODO  [DONE] (use TAB to switch)", constants::REGULAR_PAIR);
                    // rendres done items
                    ui.begin_list(done_curr);
                    for (index, done) in dones.iter().enumerate() {
                        ui.list_element(&format!("- [X] {}", done), index);
                    }
                    ui.end_list();
                }
            }
        }
        ui.end();

        refresh();

        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            's' => save_state(&todos, &dones, &file_path),
            'd' => match tab {
                Status::Todo => remove_item(&mut todos, &mut todo_curr),
                Status::Done => remove_item(&mut dones, &mut done_curr),
            },
            'k' => match tab {
                Status::Todo => list_up(&mut todo_curr),
                Status::Done => list_up(&mut done_curr),
            },
            'j' => match tab {
                Status::Todo => list_down(&todos, &mut todo_curr),
                Status::Done => list_down(&dones, &mut done_curr),
            },
            '\n' => match tab {
                Status::Todo => list_transfer(&mut dones, &mut todos, &mut todo_curr),
                Status::Done => list_transfer(&mut todos, &mut dones, &mut done_curr),
            },
            '\t' => {
                tab = tab.toggle();
            }
            _ => {
                todos.push(format!("{}", key));
            }
        }
    }

    save_state(&todos, &dones, &file_path);

    endwin();
}
