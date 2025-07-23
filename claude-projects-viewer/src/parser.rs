use crate::types::{ProjectFile, SessionMessage};
use glob::glob;
use home::home_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ProjectParser;

impl ProjectParser {
    pub fn load_all_projects() -> Result<Vec<ProjectFile>, Box<dyn std::error::Error>> {
        let home = home_dir().ok_or("Could not find home directory")?;
        let pattern = format!("{}/.claude/projects/**/*.jsonl", home.display());
        
        let mut projects = Vec::new();
        
        for path in glob(&pattern)?.flatten() {
            if let Ok(project) = Self::load_project_file(&path) {
                projects.push(project);
            }
        }
        
        Ok(projects)
    }
    
    pub fn load_project_file(path: &Path) -> Result<ProjectFile, Box<dyn std::error::Error>> {
        let mut project = ProjectFile::new(path.to_string_lossy().to_string());
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<SessionMessage>(&line) {
                Ok(message) => project.messages.push(message),
                Err(_) => {
                    // Skip parsing errors silently to avoid cluttering output
                    // Continue parsing other lines
                }
            }
        }
        
        Ok(project)
    }
    
    pub fn search_messages(
        projects: &[ProjectFile],
        query: &str,
    ) -> Vec<(String, Vec<SessionMessage>)> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        
        for project in projects {
            let mut matching_messages = Vec::new();
            
            for message in &project.messages {
                let text_contents = message.get_text_content();
                let matches = text_contents
                    .iter()
                    .any(|content| content.to_lowercase().contains(&query_lower));
                
                if matches {
                    matching_messages.push(message.clone());
                }
            }
            
            if !matching_messages.is_empty() {
                results.push((project.project_name.clone(), matching_messages));
            }
        }
        
        results
    }
}