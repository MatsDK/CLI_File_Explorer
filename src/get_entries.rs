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
