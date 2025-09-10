use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

#[derive(Debug, Serialize, Deserialize)]
struct Education {
    name: String,
    degree: String,
    years: String,
    points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    link: Option<String>,
    points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Experience {
    company: String,
    position: String,
    years: String,
    points: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resume {
    name: String,
    contact: String,
    education: Vec<Education>,
    projects: Vec<Project>,
    experience: Vec<Experience>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load JSON data
    let data_str = fs::read_to_string("data/resume.json")?;
    let resume: Resume = serde_json::from_str(&data_str)?;

    // Load template
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("resume", "templates/resume.tex.hbs")?;
    handlebars.register_helper("bold", Box::new(bold_helper));
    handlebars.register_helper("href", Box::new(href_helper));

    // Render LaTeX
    let tex_output = handlebars.render("resume", &resume)?;

    // Write to .tex
    let out_path = Path::new("output/resume.tex");
    fs::create_dir_all("output")?;
    fs::write(out_path, tex_output)?;

    println!("Generated {}", out_path.display());
    Ok(())
}

fn bold_helper(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let text = param.value().as_str().unwrap_or("");
        out.write(&format!("\\textbf{{{}}}", text))?; 
    }
    Ok(())
}

fn href_helper(
    h: &Helper<'_>,
    _: &Handlebars<'_>,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let text = param.value().as_str().unwrap_or("");
        out.write(&format!("\\href{{{}}}{{GitHub Repo Link}}", text))?;
    }
    Ok(())
}