#[derive(Debug)]
pub struct RuntimeOptions {
    pub debug_mode: bool,
    pub help: bool,
    pub quiet: bool,
    // more options coming soon
}

pub fn parse_arg(arg: &str, options: &mut RuntimeOptions) {
    match arg {
        "-d" | "--debug" => options.debug_mode = true,
        "-h" | "--help" => options.help = true,
        "-q" | "--quiet" => options.quiet = true,
        _ => unimplemented!(),
    }
}
