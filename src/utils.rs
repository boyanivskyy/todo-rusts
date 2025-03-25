use std::io::Write;
use std::{cmp::min, fs::File};

use crate::models::Status;

pub fn parse_item(line: &str) -> Option<(Status, &str)> {
    let todo_prefix = "TODO: ";
    let done_prefix = "DONE: ";

    if line.starts_with(todo_prefix) {
        return Some((Status::Todo, &line[todo_prefix.len()..]));
    } else if line.starts_with(done_prefix) {
        return Some((Status::Done, &line[done_prefix.len()..]));
    }

    return None;
}

pub fn list_up(list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1
    }
}

pub fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    *list_curr = min(*list_curr + 1, list.len() - 1);
}

pub fn list_transfer(
    list_dest: &mut Vec<String>,
    list_src: &mut Vec<String>,
    list_src_curr: &mut usize,
) {
    // check if element exist as remove do not return an Option
    if list_src.len() > 0 {
        list_dest.push(list_src.remove(*list_src_curr));

        if *list_src_curr >= list_src.len() && list_src.len() > 0 {
            *list_src_curr -= 1
        }
    }
}

pub fn save_state(todos: &Vec<String>, dones: &Vec<String>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();

    for todo in todos.iter() {
        writeln!(file, "TODO: {}", todo).unwrap();
    }

    for done in dones.iter() {
        writeln!(file, "DONE: {}", done).unwrap();
    }
}
