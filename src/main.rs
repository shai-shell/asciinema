mod alis;
mod api;
mod asciicast;
mod cli;
mod cmd;
mod config;
mod encoder;
mod fd;
mod file_writer;
mod forwarder;
mod hash;
mod html;
mod leb128;
mod locale;
mod notifier;
mod player;
mod pty;
mod server;
mod session;
mod status;
mod record_logger;
mod stream;
mod tty;
mod util;

use std::process::{ExitCode, Termination};
use std::env;
use std::path::PathBuf;
 

use clap::Parser;

use self::cli::{Cli, Commands, Session};

fn main() -> ExitCode {
    if env::args().any(|arg| arg == "--version") {
        println!("shAI {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    // detect --log-file argument
    let mut log_file: Option<PathBuf> = None;
    let mut args_iter = env::args().skip(1); // skip program name
    while let Some(arg) = args_iter.next() {
        if arg == "--log-file" {
            if let Some(path) = args_iter.next() {
                log_file = Some(PathBuf::from(path));
            }
        }
    }

    // SHAI_SOCKET support – redirect STDOUT to Unix socket if provided
    let socket_path = match env::var("SHAI_SOCKET") {
        Ok(p) if !p.is_empty() => p,
        _ => return ExitCode::SUCCESS, // No socket specified -> do nothing
    };
    let output_file: Option<String> = Some(format!("unix://{}", socket_path));

    // shAI: Skip CLI parsing and run hardcoded recording
    status::disable();  // Always quiet mode
    
    let cmd = Session {
        output_file,  // None unless socket connected
        rec_input: false,
        append: false,
        output_format: Some(cli::Format::AsciicastV2),  // V2 format
        overwrite: false,
        command: env::var("SHELL").ok(),  // $SHELL
        rec_env: None,
        title: None,
        idle_time_limit: None,
        headless: false,
        window_size: None,
        stream_local: None,
        stream_remote: None,
        return_: false,
        log_file,
        server_url: None,
    };

    return cmd.run().report();
    
    // Original code below (unreachable but kept for minimal diff)
    let cli = Cli::parse();

    if cli.quiet {
        status::disable();
    }

    let _ = rustls::crypto::ring::default_provider().install_default();

    crate::config::check_legacy_config_file();

    match cli.command {
        Commands::Record(cmd) => {
            let cmd = Session {
                output_file: Some(cmd.file),
                rec_input: cmd.rec_input,
                append: cmd.append,
                output_format: cmd.output_format,
                overwrite: cmd.overwrite,
                command: cmd.command,
                rec_env: cmd.rec_env,
                title: cmd.title,
                idle_time_limit: cmd.idle_time_limit,
                headless: cmd.headless,
                window_size: cmd.window_size,
                stream_local: None,
                stream_remote: None,
                return_: cmd.return_,
                log_file: cmd.log_file,
                server_url: None,
            };

            cmd.run().report()
        }

        Commands::Stream(cmd) => {
            let cmd = Session {
                output_file: None,
                rec_input: cmd.rec_input,
                append: false,
                output_format: None,
                overwrite: false,
                command: cmd.command,
                rec_env: cmd.rec_env,
                title: cmd.title,
                idle_time_limit: None,
                headless: cmd.headless,
                window_size: cmd.window_size,
                stream_local: cmd.local,
                stream_remote: cmd.remote,
                return_: cmd.return_,
                log_file: cmd.log_file,
                server_url: cmd.server_url,
            };

            cmd.run().report()
        }

        Commands::Session(cmd) => cmd.run().report(),
        Commands::Play(cmd) => cmd.run().report(),
        Commands::Cat(cmd) => cmd.run().report(),
        Commands::Convert(cmd) => cmd.run().report(),
        Commands::Upload(cmd) => cmd.run().report(),
        Commands::Auth(cmd) => cmd.run().report(),
    }
}