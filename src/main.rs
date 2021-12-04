extern crate ncurses;

use ncurses::*;
use std::cmp::*;
use std::fs::*;
use std::path::Path;

mod tree;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

enum CommandType {
    NewFile,
    NewDir,
    Delete,
    Error(String),
    None,
}

struct Ui {
    curr_path: String,
    parent_path: String,
    command: CommandType,
    input_value: String,
    input_cursor: i32,
    tree: tree::Tree,
}

impl Ui {
    fn begin(&mut self, width: &i32, height: &i32) {
        mv(height - 1, 3);

        match &self.command {
            CommandType::NewFile => {
                let str = format!("New file name: {}", self.input_value);
                addstr(&str as &str);

                let cursor = self.input_cursor as usize;
                mv(height - 1, 18 + self.input_cursor);
                attron(COLOR_PAIR(HIGHLIGHT_PAIR));
                addstr(self.input_value.get(cursor..=cursor).unwrap_or(" "));
                attroff(COLOR_PAIR(HIGHLIGHT_PAIR));
            },
            CommandType::NewDir => {
                let str = format!("New folder name: {}", self.input_value);
                addstr(&str as &str);

                let cursor = self.input_cursor as usize;
                mv(height - 1, 20 + self.input_cursor);
                attron(COLOR_PAIR(HIGHLIGHT_PAIR));
                addstr(self.input_value.get(cursor..=cursor).unwrap_or(" "));
                attroff(COLOR_PAIR(HIGHLIGHT_PAIR));
            },
            CommandType::Delete => {
                addstr("Press enter to delete");
            },
            CommandType::Error(err) => {
                addstr(&err);
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

    fn handle_input(&mut self, c: &i32, entries: &mut Vec<tree::Entry>, file_curr: &usize, start_select: &Option<i32>) {
        match self.command {
            CommandType::Error(_) => {
                self.command = CommandType::None;
                self.input_value = String::from("");
                self.input_cursor = 0;
            },
            _ => {}
        }

        match c {
            127 => if self.input_value.len() != 0 { // BACKSPACE
                self.input_value.remove(self.input_value.len() - 1);
                self.input_cursor -= 1;
            }, 
            32..=126 => {
                self.input_value.push(*c as u8 as char);
                self.input_cursor += 1;
            },
            10 => { // ENTER
                match self.command {
                    CommandType::NewFile => {
                        let path = { 
                            if self.curr_path == "/" { 
                                format!("/{}", self.input_value) 
                            } else { 
                                format!("{}/{}", self.curr_path, self.input_value) 
                            }  
                        };

                        let _path = Path::new(&path);
                        if _path.exists() && !_path.is_dir() {
                            self.command = CommandType::Error(String::from("File already exists"));
                        } else {
                            match File::create(&path) {
                                Ok(_) => {
                                    entries.insert(0, tree::Entry {
                                        name: self.input_value.clone(),
                                        path,
                                        r#type: String::from("file"),
                                        _children: Vec::new()
                                        
                                    });
                                    self.command = CommandType::None;
                                },
                                Err(err) => self.command = CommandType::Error(err.to_string())
                            }
                        }
                    },
                    CommandType::NewDir => {
                        let path = { 
                            if self.curr_path == "/" { 
                                format!("/{}", self.input_value) 
                            } else { 
                                format!("{}/{}", self.curr_path, self.input_value) 
                            }  
                        };

                        match create_dir(&path) {
                            Ok(_) => {
                                entries.insert(0, tree::Entry {
                                    name: self.input_value.clone(),
                                    path,
                                    r#type: String::from("dir"),
                                    _children: Vec::new()
                                });
                                self.command = CommandType::None;
                            },
                            Err(err) => self.command = CommandType::Error(err.to_string())
                        }
                    },
                    CommandType::Delete => {
                        fn delete_entry(entry: &tree::Entry, command: &mut CommandType) {
                            if &entry.r#type == "dir" {
                                match remove_dir_all(&entry.path) {
                                    Ok(_) => {
                                    },
                                    Err(err) => *command = CommandType::Error(err.to_string())
                                }
                            } else {
                                match remove_file(&entry.path) {
                                    Ok(_) => {
                                    },
                                    Err(err) => *command = CommandType::Error(err.to_string())
                                }
                            }
                        }

                        match *start_select {
                            None => delete_entry(&entries[*file_curr], &mut self.command),
                            start => {
                                let diff: i32 = start.unwrap() - *file_curr as i32; 
                                for i in 0..=diff.abs() {
                                    let idx: i32 = i + min(start.unwrap(), *file_curr as i32);
                                    delete_entry(&entries[idx as usize], &mut self.command);
                                }
                            }
                        }

                        self.command = CommandType::None;

                    },
                    _ => {}
                }

                self.input_value = String::from("");
                self.input_cursor = 0;
            },
            _ => {}
        }
    }

    fn set_entries(&mut self, entries: &mut Vec<tree::Entry>) {
        *entries = vec![];

        for e in self.tree.root.iter() {
            entries.push(tree::Entry{
                name: (*e.name).to_string(),
                path: (*e.path).to_string(),
                r#type: (*e.r#type).to_string(),
                _children: vec![]
            });
        }
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

fn list_down(file_curr: &mut usize, top_offset: &mut i32, max_y: &i32, entries: &Vec<tree::Entry>) {
    if entries.len() > 0 {
        *file_curr = min(*file_curr + 1, entries.len() - 1);

        let x: i32 = max_y - 3 + *top_offset; 
        if (*file_curr as i32) > x.try_into().unwrap() {
            *top_offset += 1; 
        }
    }
}

fn move_back(ui: &mut Ui, entries: &mut Vec<tree::Entry>, top_offset: &mut i32, file_curr: &mut usize) {
        ui.set_entries(&mut *entries); 
        *top_offset = 0;
        *file_curr = 0;
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
        input_cursor: 0,
        input_value: String::from(""),
        tree: tree::parse_tree()
    };

    let mut quit = false;
    let mut file_curr: usize = 0;
    let mut select_start: Option<i32> = None;
    let mut top_offset: i32 = 0;
    let mut entries: Vec<tree::Entry> = Vec::new(); 

    ui.set_entries(&mut entries);
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

                let label = format!("{} {}", { if entry.r#type == "dir" { "d" } else { "f" } } , &entry.name);
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
                    'd' => if entries.len() != 0 { ui.command = CommandType::Delete },
                    'o' => ui.command = CommandType::NewFile,
                    'O' => ui.command = CommandType::NewDir,
                    'k' => list_up(&mut file_curr, &mut top_offset),
                    'j' => list_down(&mut file_curr, &mut top_offset, &max_y, &entries),
                    'v' => select_start = Some(file_curr as i32),
                    'h' => {
                            ui.curr_path = ui.parent_path.to_string();
                            move_back(&mut ui, &mut entries, &mut top_offset, &mut file_curr);
                            ui.set_parent_path();
                            select_start = None;
                    },
                    '\n' => if entries.len() != 0 && entries[file_curr].r#type == "dir" {
                            ui.curr_path = entries[file_curr].path.to_string(); 
                            ui.set_entries(&mut entries);
                            top_offset = 0;
                            file_curr = 0;
                            select_start = None; 

                            ui.set_parent_path();
                    }
                    'l' => if entries.len() != 0 && entries[file_curr].r#type == "dir" {
                            ui.curr_path = entries[file_curr].path.to_string(); 
                            ui.set_entries(&mut entries);
                            top_offset = 0;
                            file_curr = 0;
                            select_start = None;

                            ui.set_parent_path();
                    },
                    _ => {}

                }
            },
            _ => ui.handle_input(&c, &mut entries, &file_curr, &select_start) 
        }
    }

    endwin();
}

