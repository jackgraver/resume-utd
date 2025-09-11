use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Education {
    pub name: String,
    pub degree: String,
    pub years: String,
    pub points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub link: Option<String>,
    pub points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    pub company: String,
    pub position: String,
    pub years: String,
    pub points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resume {
    pub name: String,
    pub contact: String,
    pub website: Option<String>,
    pub education: Vec<Education>,
    pub projects: Vec<Project>,
    pub experience: Vec<Experience>,
}

#[derive(Debug)]
pub struct DataManager {
    pub resume: Resume,
    pub file_path: String,
}

impl DataManager {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let data_str = fs::read_to_string(file_path)?;
        let resume: Resume = match serde_json::from_str(&data_str) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to parse resume JSON: {}", e);
                std::process::exit(1);
            }
        };

        Ok(Self {
            resume,
            file_path: file_path.to_string(),
        })
    }


    pub fn save(&self) -> io::Result<()> {
        let json_str = serde_json::to_string_pretty(&self.resume)?;
        fs::write(&self.file_path, json_str)?;
        Ok(())
    }

    pub fn add_education(&mut self, education: Education) {
        self.resume.education.push(education);
    }

    pub fn add_experience(&mut self, experience: Experience) {
        self.resume.experience.push(experience);
    }

    pub fn add_project(&mut self, project: Project) {
        self.resume.projects.push(project);
    }

    pub fn update_education(&mut self, index: usize, education: Education) -> Result<(), String> {
        if index < self.resume.education.len() {
            self.resume.education[index] = education;
            Ok(())
        } else {
            Err("Education index out of bounds".to_string())
        }
    }

    pub fn update_experience(&mut self, index: usize, experience: Experience) -> Result<(), String> {
        if index < self.resume.experience.len() {
            self.resume.experience[index] = experience;
            Ok(())
        } else {
            Err("Experience index out of bounds".to_string())
        }
    }

    pub fn update_project(&mut self, index: usize, project: Project) -> Result<(), String> {
        if index < self.resume.projects.len() {
            self.resume.projects[index] = project;
            Ok(())
        } else {
            Err("Project index out of bounds".to_string())
        }
    }

    pub fn delete_education(&mut self, index: usize) -> Result<(), String> {
        if index < self.resume.education.len() {
            self.resume.education.remove(index);
            Ok(())
        } else {
            Err("Education index out of bounds".to_string())
        }
    }

    pub fn delete_experience(&mut self, index: usize) -> Result<(), String> {
        if index < self.resume.experience.len() {
            self.resume.experience.remove(index);
            Ok(())
        } else {
            Err("Experience index out of bounds".to_string())
        }
    }

    pub fn delete_project(&mut self, index: usize) -> Result<(), String> {
        if index < self.resume.projects.len() {
            self.resume.projects.remove(index);
            Ok(())
        } else {
            Err("Project index out of bounds".to_string())
        }
    }
}
