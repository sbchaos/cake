use log::trace;
use std::io::{BufReader, Read};
use std::process::{Child, Command, Stdio};

pub(crate) fn docker(args: Vec<&str>) -> Result<String, String> {
    trace!("Calling docker-cli: docker {:?}", &args[0..2]);

    let mut child: Child = Command::new("docker")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");

    let mut reader = BufReader::new(stdout);
    let mut input = String::new();

    let result = match reader.read_to_string(&mut input) {
        Ok(_) => Ok(input),
        Err(err) => Err(err.to_string()),
    };

    trace!("Waiting for command to finish");
    let _ = child.wait();

    result
}
