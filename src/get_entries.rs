/*
use std::fs::read_dir;

fn get_entries() -> Vec<String> {
    let mut entries: Vec<String> = vec![];

    for entry_res in read_dir("/").unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();

        if !file_name.starts_with(".") {
                entries.push(format!("File {:?} has full path {:?}",file_name, entry.path()));
        }
    }

    entries
}

fn main() {
    let entries: Vec<String> = get_entries(); 

    for (i, entry) in entries.iter().enumerate() {
        println!("{}{}", i, entry);
    }
}
*/


fn get_entries(path: &str) -> Vec<tree::Entry> {
    let mut entries: Vec<tree::Entry> = Vec::new(); 

    for entry_res in read_dir(path).unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();

        if !file_name.starts_with(".") {
            let curr_path = format!("{}", entry.path().display());

            entries.push(tree::Entry {
                name: String::from(file_name),
                path: curr_path,
                r#type: String::from(if entry.path().is_dir() { "dir" } else { "file" }),
                _children: Vec::new()
            });
        }
    }

    entries
}

