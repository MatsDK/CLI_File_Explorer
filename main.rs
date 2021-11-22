use std::fs::read_dir;


fn main() {
    for entry_res in read_dir(".").unwrap() {
        let entry = entry_res.unwrap();
        let file_name_buf = entry.file_name();
        let file_name = file_name_buf.to_str().unwrap();

        if !file_name.starts_with(".") &&
            entry.file_type().unwrap().is_dir() {
                let m: &str = &format!("File {:?} has full path {:?}",file_name, entry.path());
                println!("{}", &m);
        }
    }
}
