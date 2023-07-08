use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use clap::{command, Arg, ArgAction};
use colored::Colorize;
use openapiv3::OpenAPI;

use crate::config::config::Config;
use crate::openapi_joiner::openapi_joiner::OpenAPIJoiner;

mod config;
mod openapi_joiner;

fn main() {
    let args = command!()
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .required(true)
                .help("configuration file")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(false)
                .default_value("openapi.json")
                .help("output destination")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .required(false)
                .help("quiet mode")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let quiet = args.get_flag("quiet");

    if !quiet {
        let version: &str = env!("CARGO_PKG_VERSION");
        println!("{} {}", "OpenAPI Joiner".bold(), version);
    }

    if let Some(config_file) = args.get_one::<String>("config") {
        let config = Config::read_from(config_file.as_str());
        let validation_result = config.validate();
        if validation_result.is_err() {
            error_exit(
                format!(
                    "Invalid configuration: {}",
                    validation_result.err().unwrap()
                ),
                quiet,
            );
        }
        let mut openapi_joiner = OpenAPIJoiner::new();
        config.applications.into_iter().for_each(|application| {
            let specification = read_specification(application.spec.as_str());
            if let Some(spec) = specification {
                openapi_joiner.add(spec, application.path.as_str(), application.prefix.as_str());
            }
        });
        let output = args.get_one::<String>("output").unwrap();
        openapi_joiner.write_to(output.as_str());
        success_exit(output, quiet);
    }
}

fn success_exit(output: &str, quiet: bool) {
    if !quiet {
        let destination = match output {
            "-" => "stdout".to_string(),
            _ => {
                let canon_result = PathBuf::from(output).canonicalize();
                match canon_result {
                    Ok(path) => path.into_os_string().into_string().unwrap(),
                    _ => output.to_string(),
                }
            }
        };
        println!(
            "{}: The merged OpenAPI specification was written to {}.",
            "Success".green(),
            destination
        );
    }
    exit(0);
}

fn error_exit(message: String, quiet: bool) {
    if !quiet {
        let error = format!("{}: {}", "Error".red(), message);
        println!("{}", error);
    }
    exit(1);
}

fn read_specification(path: &str) -> Option<OpenAPI> {
    let extension = PathBuf::from(path)
        .extension()
        .and_then(OsStr::to_str)?
        .to_lowercase();
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    return match extension.as_str() {
        "yaml" => Some(
            serde_yaml::from_str(contents.as_str())
                .expect(&format!("Could not deserialize file {}", path)),
        ),
        "json" => Some(
            serde_json::from_str(contents.as_str())
                .expect(&format!("Could not deserialize file {}", path)),
        ),
        _ => None,
    };
}
