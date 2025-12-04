pub struct InputProcessor {
    file_path: String,
}

impl InputProcessor {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
    
    pub fn read_comma_separated_string(&self) -> Vec<String> {
        let contents = std::fs::read_to_string(&self.file_path)
            .expect("Failed to read input.txt");
        
        return contents
            .split(',')
            .map(|range: &str| range.trim().to_string())
            .collect::<Vec<String>>();
    }

    pub fn read_lines(&self) -> Vec<String> {
        let contents = std::fs::read_to_string(&self.file_path)
            .expect("Failed to read input.txt");
        
        return contents
            .lines()
            .map(|line: &str| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect::<Vec<String>>();
    }
}
