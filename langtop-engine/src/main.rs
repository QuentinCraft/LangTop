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
    total_code_lines: u32,
    is_valid: bool,
}

impl Langtop {
    /*
     * Constructor
     * @param path: &str
     * @return Langtop
     */
    fn new(path: &str) -> Langtop {
        Langtop {
            folder: Langtop::extract_folder_name(path),
            path: PathBuf::from(path),
            total_files: 0,
            total_lines: 0,
            total_bytes: 0,
            total_dirs: 0,
            total_empty_lines: 0,
            total_comments: 0,
            total_code_lines: 0,
            is_valid: Langtop::is_valid_folder(path)
        }
    }

    /*
     * Extract the folder name from a path
     * @param path: &str : The path to the folder
     * @return String : The folder name
     */
    fn extract_folder_name(path: &str) -> String {
        let path: &Path = Path::new(path);
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    }

    /*
     * Get the folder path
     * @return &PathBuf
     */
    fn get_folder_path(&self) -> &PathBuf {
        &self.path
    }

    /*
     * Get the folder name
     * @return &str
     */
    fn get_folder_name(&self) -> &str {
        &self.folder
    }

    /*
     * Get the total files in a specific folder
     * @param path: &str : The path to the folder
     * @return u32
     */
    fn get_total_files(path: &str) -> u32 {
        let mut total_files = 0;
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_file() {
                        total_files += 1;
                    }
                }
            }
        }
        total_files
    }

    /*
     * Check if a folder is valid
     * @param path: &str : The path to the folder
     * @return bool
     */
    fn is_valid_folder(path: &str) -> bool {
        let path: &Path = Path::new(path);
        path.exists() && path.is_dir()
    }
    
}

fn main() {
    println!("Hello, world!");

    // Create a new Langtop instance
    let langtop = Langtop::new("./hello/abc");

    println!("Folder name: {}", langtop.get_folder_name());
    println!("Folder path: {}", langtop.get_folder_path().display());
    println!("Is valid folder: {}", langtop.is_valid);
    println!("Total files: {}", Langtop::get_total_files("./hello/abc"));
}
