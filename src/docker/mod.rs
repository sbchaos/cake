mod cli;

pub fn inspect(image_id: &str) -> Result<String, String> {
    cli::docker(vec!["inspect", image_id])
}

pub fn save(image_id: &str) {
    cli::docker(vec!["save", image_id, "-o", &format!("{}.tar", image_id)]).unwrap();
}

pub fn run(image: &str, args: Vec<&str>) -> Result<String, String> {
    let mut run = vec!["run", "--rm", "-it", image];
    run.extend(args);
    cli::docker(run)
}

pub fn image_id(image: &str) -> String {
    let args = vec!["images", image, "-q"];
    if let Ok(id) = cli::docker(args) {
        let trimmed = id.trim_end().to_string();
        if !trimmed.is_empty() {
            return trimmed;
        }
    }
    image.to_string()
}
