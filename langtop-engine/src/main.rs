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
     * Get the total files
     * @return u32 : The total files
     */
    fn get_total_files(&self) -> u32 {
        self.total_files
    }

    /*
     * Get the total lines
     * @return u32 : The total lines
     */
    fn get_total_lines(&self) -> u32 {
        self.total_lines
    }

    /*
     * Get the total bytes
     * @return u32 : The total bytes
     */
    fn get_total_bytes(&self) -> u32 {
        self.total_bytes
    }

    /*
     * Get the total directories
     * @return u32 : The total directories
     */
    fn get_total_dirs(&self) -> u32 {
        self.total_dirs
    }

    /*
     * Get the total empty lines
     * @return u32 : The total empty lines
     */
    fn get_total_empty_lines(&self) -> u32 {
        self.total_empty_lines
    }

    /*
     * Get the total comments
     * @return u32 : The total comments
     */
    fn get_total_comments(&self) -> u32 {
        self.total_comments
    }

    /*
     * Get the total code lines
     * @return u32 : The total code lines
     */
    fn get_total_code_lines(&self) -> u32 {
        self.total_code_lines
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
    println!("***** Langtop Engine *****");

    // Create a new Langtop instance
    let langtop = Langtop::new("./hello/abc");

    println!("Folder name: {}", langtop.get_folder_name());
    println!("Folder path: {}", langtop.get_folder_path().display());
    println!("Is valid folder: {}", langtop.is_valid);
    println!("Total files: {}", langtop.get_total_files());
    println!("Total lines: {}", langtop.get_total_lines());
    println!("Total bytes: {}", langtop.get_total_bytes());
    println!("Total directories: {}", langtop.get_total_dirs());
    println!("Total empty lines: {}", langtop.get_total_empty_lines());
    println!("Total comments: {}", langtop.get_total_comments());
    println!("Total code lines: {}", langtop.get_total_code_lines());
    

    println!("***** END *****");
}
