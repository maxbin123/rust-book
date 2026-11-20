use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::Path;
use std::process::ExitCode;

use job_audit::audit;

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let (Some(path), None) = (args.next(), args.next()) else {
        eprintln!("Usage: job-audit <manifest>");
        return ExitCode::from(2);
    };

    let path = Path::new(&path);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let summary = match audit(BufReader::new(file)) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("{}: {error}", path.display());
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = writeln!(
        io::stdout().lock(),
        "jobs={} email={} report={}",
        summary.total,
        summary.emails,
        summary.reports,
    ) {
        eprintln!("could not write summary: {error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
