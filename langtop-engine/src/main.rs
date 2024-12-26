use std::path::{Path, PathBuf};

struct Langtop {
    folder: String,
    path: PathBuf,
    total_files: u32,
    total_lines: u32,
    total_bytes: u32,
    total_dirs: u32,
    total_empty_lines: u32,
    total_comments: u32,
    total_code_lines: u32
}

impl Langtop  {
    /*
     * Constructor
     * @param folder: String
     * @return Langtop
     */
    fn new(path: &str) -> Langtop {
        Langtop {
            folder: Langtop::get_folder_name(path),
            path: PathBuf::from(path),
            total_files: 0,
            total_lines: 0,
            total_bytes: 0,
            total_dirs: 0,
            total_empty_lines: 0,
            total_comments: 0,
            total_code_lines: 0,
        }
    }

    /*
     * Get the folder name
     * @param path: &str : The path to the folder
     * @return String : The folder name
     */
    fn get_folder_name(path: &str) -> String {
        let path: &std::path::Path = std::path::Path::new(path);
        path.file_name().unwrap().to_str().unwrap().to_string()
    }
}

fn main() {
    println!("Hello, world!");
    // Create a new Langtop instance
    let langtop = Langtop::new("./hello/abc");
    println!("Folder: {}", langtop.folder);
}
