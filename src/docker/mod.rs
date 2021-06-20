mod cli;

pub fn inspect(image_id: &str) -> String {
    cli::docker(vec!["inspect", image_id]).unwrap()
}

pub fn save(image_id: &str) {
    cli::docker(vec!["save", image_id, "-o", &format!("{}.tar", image_id)]).unwrap();
}

pub fn run(image: &str, args: Vec<&str>) -> Result<String, String> {
    let mut run = vec!["run", "--rm", "-it", image];
    run.extend(args);
    cli::docker(run)
}
