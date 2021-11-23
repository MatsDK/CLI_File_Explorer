extern crate ncurses;

use ncurses::*;
use std::cmp::*;
use std::fs::{read_dir};

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;


fn get_entries() -> Vec<String> {
    let mut entries: Vec<String> = Vec::new(); 

    for entry_res in read_dir("/").unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();

        if !file_name.starts_with(".") {
                entries.push(format!("{} {}", { if entry.path().is_dir() {
                    "dir"
                } else {
                    "file"
                }}, file_name));
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
    let entries: Vec<String> = get_entries(); 

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
            addstr(&entry as &str);
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
            _ => {}

        }

    }


    endwin();
}
