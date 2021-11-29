extern crate ncurses;

use ncurses::*;
use std::cmp::*;
use std::fs::{read_dir, create_dir, File};
use std::path::Path;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

enum CommandType {
    NewFile,
    NewDir,
    None,
}

struct Entry {
    name: String,
    path: String,
    is_dir: bool,
}

struct Input {
    value: String,
    cursor: i32,
}

impl Input {
    fn handle_input(&mut self, cmd: &mut CommandType, c: &i32, curr_path: &str, entries: &mut Vec<Entry>) {
        match c {
            127 => if self.value.len() != 0 { // BACKSPACE
                self.value.remove(self.value.len() - 1);
                self.cursor -= 1;
            }, 
            32..=126 => {
                self.value.push(*c as u8 as char);
                self.cursor += 1;
            },
            10 => { // ENTER
                match cmd {
                    CommandType::NewFile => {
                        let path = { 
                            if curr_path == "/" { 
                                format!("/{}", self.value) 
                            } else { 
                                format!("{}/{}", curr_path, self.value) 
                            }  
                        };

                        File::create(&path).unwrap();
                        entries.insert(0, Entry {
                            name: self.value.clone(),
                            path,
                            is_dir: false
                        });
                    },
                    CommandType::NewDir => {
                        let path = { 
                            if curr_path == "/" { 
                                format!("/{}", self.value) 
                            } else { 
                                format!("{}/{}", curr_path, self.value) 
                            }  
                        };

                        match create_dir(&path) {
                            Ok(_) => {
                                entries.insert(0, Entry {
                                    name: self.value.clone(),
                                    path,
                                    is_dir: true
                                });
                            },
                            Err(_) => {
                                //println!("{}", error);
                            }
                        }
                    },
                    _ => {}
                }

                self.value = String::from("");
                self.cursor = 0;
                *cmd = CommandType::None;
            },
            _ => {}
        }
    }
}

struct Ui {
    curr_path: String,
    parent_path: String,
    command: CommandType,
    input: Input
}

impl Ui {
    fn begin(&mut self, width: &i32, height: &i32) {
        mv(height - 1, 3);

        match self.command {
            CommandType::NewFile => {
                let str = format!("New file name: {}", self.input.value);
                addstr(&str as &str);

                let cursor = self.input.cursor as usize;
                mv(height - 1, 18 + self.input.cursor);
                attron(COLOR_PAIR(HIGHLIGHT_PAIR));
                addstr(self.input.value.get(cursor..=cursor).unwrap_or(" "));
                attroff(COLOR_PAIR(HIGHLIGHT_PAIR));
            },
            CommandType::NewDir => {
                let str = format!("New folder name: {}", self.input.value);
                addstr(&str as &str);

                let cursor = self.input.cursor as usize;
                mv(height - 1, 20 + self.input.cursor);
                attron(COLOR_PAIR(HIGHLIGHT_PAIR));
                addstr(self.input.value.get(cursor..=cursor).unwrap_or(" "));
                attroff(COLOR_PAIR(HIGHLIGHT_PAIR));
            },
            CommandType::None => {
                let bottom = format!("height: {} width: {}", height.to_string(), width.to_string());
                addstr(&bottom as &str);
            }
        }

        mv(0, 0);
        addstr(&self.curr_path);
    }

    fn list_item(&mut self, label: &str, color_pair: i16, row: &i32) {
            attron(COLOR_PAIR(color_pair));
            let idx = row + 1;

            mv(idx as i32, 1);
            addstr(label);
            attroff(COLOR_PAIR(color_pair));
    }

    fn set_parent_path(&mut self) {
        let parent = Path::new(&self.curr_path);

        self.parent_path = { 
            if parent.parent().is_none() { 
                String::from("/") 
            } else { 
                format!("{}", parent.parent().unwrap().display())
            }
        };
    }
}

fn list_up(file_curr: &mut usize, top_offset: &mut i32) {
        if *file_curr > 0 {
            *file_curr -= 1
        }

        if 0 > (*file_curr as i32) - *top_offset {
            *top_offset -= 1; 
        }
}

fn list_down(file_curr: &mut usize, top_offset: &mut i32, max_y: &i32, entries: &Vec<Entry>) {
        *file_curr = min(*file_curr + 1, entries.len() - 1);

        let x: i32 = max_y - 3 + *top_offset; 
        if (*file_curr as i32) > x.try_into().unwrap() {
            *top_offset += 1; 
        }
}

fn move_back(ui: &Ui, entries: &mut Vec<Entry>, top_offset: &mut i32, file_curr: &mut usize) {
        *entries = get_entries(&ui.parent_path.to_string());
        *top_offset = 0;
        *file_curr = 0;
}

fn get_entries(path: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new(); 

    for entry_res in read_dir(path).unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();

        if !file_name.starts_with(".") {
                let curr_path = format!("{}", entry.path().display());


                entries.push(Entry {
                    name: String::from(file_name),
                    path: curr_path,
                    is_dir: entry.path().is_dir()
                });
        }
    }


    entries
}


fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut ui = Ui { 
        curr_path: String::from("/"),
        parent_path: String::from("/"),
        command: CommandType::None,
        input: Input {
            cursor: 0,
            value: String::from(""),
        }
    };

    let mut quit = false;
    let mut file_curr: usize = 0;
    let mut select_start: Option<i32> = None;
    let mut top_offset: i32 = 0;
    let mut entries: Vec<Entry> = get_entries(&ui.curr_path); 
    ui.set_parent_path();

    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    while !quit {
        erase();
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        ui.begin(&max_x, &max_y);
        for (i, entry) in entries.iter().enumerate() {
            if i >= top_offset.try_into().unwrap() && (i as i32) - top_offset < max_y - 2 {
                let mut pair = { 
                    if file_curr == i {
                        HIGHLIGHT_PAIR
                    } else {
                        REGULAR_PAIR
                    }
                };

                if !select_start.is_none() &&  
                     ((select_start.unwrap() <= i as i32 &&  file_curr as i32 >= i as i32) ||
                        (select_start.unwrap() >= i as i32 &&  file_curr as i32 <= i as i32)) {
                        pair = HIGHLIGHT_PAIR;
                }

                let label = format!("{} {}", { if entry.is_dir { "d" } else { "f" } } , &entry.name);
                ui.list_item(&label, pair, &((i as i32) - top_offset));
            }
        }

        mv(max_y - 1, 0);
        refresh();

        let c = getch();
        if c == 27 { // ESC
            ui.command = CommandType::None;
            select_start = None;
        }

        match ui.command {
            CommandType::None => {
                match c as u8 as char {
                    'q' => quit = true,
                    'o' => ui.command = CommandType::NewFile,
                    'O' => ui.command = CommandType::NewDir,
                    'k' => list_up(&mut file_curr, &mut top_offset),
                    'j' => list_down(&mut file_curr, &mut top_offset, &max_y, &entries),
                    'v' => select_start = Some(file_curr as i32),
                    'h' => {
                            move_back(&ui, &mut entries, &mut top_offset, &mut file_curr);
                            ui.curr_path = ui.parent_path.to_string();
                            ui.set_parent_path();
                            select_start = None;
                    },
                    '\n' => if entries.len() != 0 && entries[file_curr].is_dir {
                            ui.curr_path = entries[file_curr].path.to_string(); 
                            entries = get_entries(&ui.curr_path);
                            top_offset = 0;
                            file_curr = 0;
                            select_start = None; 

                            ui.set_parent_path();
                    }
                    'l' => if entries.len() != 0 && entries[file_curr].is_dir {
                            ui.curr_path = entries[file_curr].path.to_string(); 
                            entries = get_entries(&ui.curr_path);
                            top_offset = 0;
                            file_curr = 0;
                            select_start = None;

                            ui.set_parent_path();
                    },
                    _ => {}

                }
            },
            _ => ui.input.handle_input(&mut ui.command, &c, &ui.curr_path, &mut entries) 
        }
    }

    endwin();
}

