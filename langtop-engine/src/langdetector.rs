use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct LandDetector;

impl LandDetector {
    pub fn detect_by_filename(filename: &str) -> Option<&str> {
        // Get the file extension
        let ext = match filename.rfind('.') {
            Some(i) => &filename[i + 1..],
            None => return None,
        };

        match ext {
            "c" | "h" => Some("C"),
            "cpp" | "hpp" => Some("C++"),
            "rs" => Some("Rust"),
            "py" => Some("Python"),
            "java" => Some("Java"),
            "js" => Some("JavaScript"),
            "ts" => Some("TypeScript"),
            "html" => Some("HTML"),
            "css" => Some("CSS"),
            "go" => Some("Go"),
            "rb" => Some("Ruby"),
            "php" => Some("PHP"),
            "sh" => Some("Shell"),
            "pl" => Some("Perl"),
            "swift" => Some("Swift"),
            "kt" => Some("Kotlin"),
            "m" => Some("Objective-C"),
            "mm" => Some("Objective-C++"),
            "json" => Some("JSON"),
            "xml" => Some("XML"),
            "yml" | "yaml" => Some("YAML"),
            "toml" => Some("TOML"),
            "ini" => Some("INI"),
            "md" => Some("Markdown"),
            "txt" => Some("Text"),
            _ => None,
        }
    }

    fn detect_by_func_declaration(file_path: &str) -> Result<Option<&str>, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let patterns = [
            ("fn ", "Rust"),
            ("def ", "Python"),
            ("function ", "JavaScript"),
            ("public class ", "Java"),
            ("class ", "C++"),
            ("func ", "Go"),
        ];

        for line in reader.lines() {
            let line = line?;
            for &(pattern, lang) in &patterns {
                if line.contains(pattern) {
                    return Ok(Some(lang));
                }
            }
        }
        Ok(None)
    }

    pub fn detect_by_shebang(content: &str) -> Option<&str> {
        let shebang = match content.lines().next() {
            Some(line) => line,
            None => return None,
        };

        if shebang.starts_with("#!") {
            if shebang.contains("python") {
                Some("Python")
            } else if shebang.contains("bash") {
                Some("Shell")
            } else if shebang.contains("sh") {
                Some("Shell")
            } else if shebang.contains("ruby") {
                Some("Ruby")
            } else if shebang.contains("perl") {
                Some("Perl")
            } else if shebang.contains("php") {
                Some("PHP")
            } else if shebang.contains("node") {
                Some("JavaScript")
            } else if shebang.contains("java") {
                Some("Java")
            } else if shebang.contains("go") {
                Some("Go")
            } else if shebang.contains("rust") {
                Some("Rust")
            } else if shebang.contains("swift") {
                Some("Swift")
            } else if shebang.contains("kotlin") {
                Some("Kotlin")
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn detect<'a>(filename: &'a str, content: &'a str) -> Option<&'a str> {
        if let Some(lang) = LandDetector::detect_by_filename(filename) {
            return Some(lang);
        }

        if let Some(lang) = LandDetector::detect_by_shebang(content) {
            return Some(lang);
        }

        if let Ok(Some(lang)) = LandDetector::detect_by_func_declaration(content) {
            return Some(lang);
        }
        None
    }
}