use structopt::StructOpt;

pub mod result;
use result::Results;

#[derive(Debug, StructOpt)]
#[structopt(name = "jq", about = "commandline JSON processor")]
/// usages....
pub struct Opt {
    #[structopt(name = "c", short, help = "Compact instead of pretty-printed output;")]
    compact: bool,
    #[structopt(name = "n", short, help = "Use `null` as the single input value;")]
    use_null: bool,
    #[structopt(
        name = "e",
        short,
        help = "Set the exit status code based on the output;"
    )]
    exit_status: bool,

    #[structopt(
        name = "s",
        short,
        help = "Read (slurp) all inputs into an array; apply filter to it;"
    )]
    slurp: bool,
    #[structopt(name = "r", short, help = "Output raw strings, not JSON texts;")]
    output_raw: bool,
    #[structopt(name = "R", short, help = "Read raw strings, not JSON texts;")]
    read_raw: bool,
    #[structopt(name = "C", short, help = "Colorize JSON;")]
    colorize: bool,
    #[structopt(name = "M", short, help = "Monochrome (don't colorize JSON);")]
    monochrome: bool,
    #[structopt(name = "S", short, help = "Sort keys of objects on output;")]
    sort_kyes: bool,
    #[structopt(name = "tab", long, help = "Use tabs for indentation;")]
    use_tabs: bool,
    #[structopt(long, help = "Remaining arguments are string arguments, not files;")]
    args: bool,
    #[structopt(long, help = "Remaining arguments are JSON arguments, not files;")]
    jsonargs: bool,
    /*
    TODO(juhyun-nam): implements parse parts.
    #[structopt(long, help = "Set variable $a to value <v>;")]
    arg: Vec<(String, String)>,
    #[structopt(long, help = "Set variable $a to JSON value <v>;")]
    argjson: Vec<(String, String)>,
    #[structopt(
        long,
        help = "Set variable $a to an array of JSON texts read from <f>;"
    )]
    slurpfile: Vec<(String, String)>,
    #[structopt(
        long,
        help = "Set variable $a to a string consisting of the contents of <f>;"
    )]
    rawfile: Vec<(String, String)>,
    */
}

pub fn run() -> Result<String, Results> {
    Opt::from_args();
    Result::Ok(String::from("nothing"))
}

#[cfg(test)]
mod tests {
    use std::env::current_exe;
    use std::path::PathBuf;
    use std::process::Command;
    use std::process::Stdio;

    fn jq() -> PathBuf {
        let mut me = current_exe().expect("failed to get current executable");

        // Chop of the test name.
        me.pop();
        // Chop off `deps`.
        me.pop();

        // If we run `cargo test --release`, we might only have a release build.
        if cfg!(release) {
            // `../release/`
            me.pop();
            me.push("release");
        }
        me.push("jq");
        assert!(
            me.is_file() || me.with_extension("exe").is_file(),
            if cfg!(release) {
                "no jq bin, try running `cargo build --release` before testing"
            } else {
                "no jq bin, try running `cargo build` before testing"
            }
        );
        me
    }

    #[test]
    fn verify_print_usage() {
        for arg in ["-h", "--help"].iter() {
            Command::new(jq())
                .arg(arg)
                .stdout(Stdio::null())
                .status()
                .expect("failed to print usage");
        }
    }
    #[test]
    fn verify_print_version() {
        Command::new(jq())
            .arg("--version")
            .stdout(Stdio::null())
            .status()
            .expect("failed to print version");
    }
}
