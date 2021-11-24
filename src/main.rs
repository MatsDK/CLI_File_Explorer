extern crate ncurses;

use ncurses::*;
use std::cmp::*;
use std::fs::{read_dir};
use std::path::Path;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

struct Entry {
    name: String,
    path: String,
    parent_path: String,
    is_dir: bool,
}

fn get_entries(path: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new(); 

    let parent = Path::new(path);

    for entry_res in read_dir(path).unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();

        if !file_name.starts_with(".") {
                let curr_path = format!("{}", entry.path().display());

                let parent_path = { 
                    if parent.parent().is_none() { 
                        String::from("/") 
                    } else { 
                        format!("{}", parent.parent().unwrap().display())
                    }
                };

                entries.push(Entry {
                    name: String::from(file_name),
                    path: curr_path,
                    parent_path: parent_path,
                    is_dir: entry.path().is_dir()
                });
        }
    }

    entries
}


fn main() {
    initscr();
    //noecho();

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut file_curr: usize = 0;
    let mut entries: Vec<Entry> = get_entries("/"); 

    let mut max_x = 0;
    let mut max_y = 0;

    while !quit {
        clear();
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        for (i, entry) in entries.iter().enumerate() {
            let pair = { 
                if file_curr == i {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            attron(COLOR_PAIR(pair));
            mv(i as i32, 0);
            addstr(&entry.name as &str);
            attroff(COLOR_PAIR(pair));
        }

        mv(max_y - 1, 3);
        let s = format!("height: {} width: {}", max_y.to_string(), max_x.to_string());
        addstr(&s as &str);

        mv(max_y - 1, 0);
        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'k' => if file_curr > 0 {
                    file_curr -= 1
            },
            'j' => file_curr = min(file_curr + 1, entries.len() - 1), 
            'h' => {
                    entries = get_entries(&entries[file_curr].parent_path);
                    file_curr = 0;
            },
            '\n' => if entries[file_curr].is_dir {
                    entries = get_entries(&entries[file_curr].path);
                    file_curr = 0;
            }
            ,
            _ => {}

        }

    }


    endwin();
}
