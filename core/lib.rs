use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use std::{fs::File, path::Path, path::PathBuf};

mod cache;
mod command;
mod error;
mod git;
mod git_cmd;
mod line;
mod parser;
mod status;

pub use error::GitnuError;
pub use parser::parse;

use cache::Cache;
use error::ToGitnuError;
use std::io::Lines;
use Subcommand::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

/// Git sub-command.
///
/// Lets Gitnu know what output to expect, and whether or not to
/// read/write cache.
#[derive(Debug, PartialEq)]
pub enum Subcommand {
    /// Contained value represents if the status command is the
    /// regular variant.
    ///
    /// `gitnu status` with no flags gives Status(true).
    /// flags `-s`, `--short`, `--porcelain` gives Status(false).
    Status(bool),

    /// Gitnu will fetch cache in this state.
    Number,

    /// A special case where gitnu drops everything and prints its own
    /// version next to git's version.
    Version,

    /// Original state.
    Unset,
}

/// Gitnu's running state.
#[derive(Debug)]
pub struct App {
    /// Controls main flow (read/write/which parser to use)
    subcommand: Subcommand,

    /// The command that will be ran once all processing is complete.
    cmd: Command,

    /// Numer of arguments that came before the subcommand.
    /// Essentially these are Git's options, rather than Git's
    /// subcommand's options.
    argc: usize,

    /// Cache that came from latest run of `gitnu status`
    cache: Vec<String>,
    buffer: Option<Lines<BufReader<File>>>,
    /// Location that `git status` was last ran on
    cache_cwd: PathBuf,
}

impl App {
    /// Creates a new App instance.
    pub fn new(cwd: PathBuf) -> App {
        let mut cmd = Command::new("git");
        if atty::is(atty::Stream::Stdout) {
            cmd.args(["-c", "color.ui=always"]);
        }
        cmd.current_dir(&cwd);
        App {
            subcommand: Subcommand::Unset,
            cache: vec![],
            cmd,
            buffer: None,
            argc: 0,
            cache_cwd: PathBuf::new(),
        }
    }

    /// Get the current directory of the app
    pub fn cwd(&self) -> &Path {
        // Unwrap safety is guaranteed by App::new() always initializing
        // `self.cmd` with a value
        self.cmd.get_current_dir().unwrap()
    }

    /// Sets the subcommand of the App.
    pub fn set_subcommand(&mut self, s: Subcommand) {
        match (&self.subcommand, &s) {
            (Unset, _) | (Status(true), Status(false)) => self.subcommand = s,
            _ => (),
        }
    }

    /// Sets the pre-subcommand argument count.
    pub fn set_argc(&mut self) {
        let argc = self.cmd.get_args().count();
        self.argc = match self.subcommand {
            Unset => argc,
            _ => argc - 1,
        }
    }

    /// Appends an argument to the final command to be ran.
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) {
        self.cmd.arg(arg);
    }

    /// Runs Gitnu after all parsing is complete.
    pub fn run(&mut self) -> Result<(), GitnuError> {
        use command::CommandOps;
        match self.subcommand {
            Status(is_normal) => status::status(self, is_normal),
            Version => {
                let result = self.cmd.run();
                println!("gitnu version {}", VERSION.unwrap_or("unknown"));
                result
            }
            _ => self.cmd.run(),
        }
    }
}

impl App {
    /// Mocks a quick instance of `App` easily by allowing literals.
    pub fn mock(args: &[&str], cwd: &str, sc: Subcommand, argc: usize) -> App {
        let mut app = App::new(PathBuf::from(cwd));
        app.argc = app.cmd.get_args().count();
        app.cmd.args(args);
        app.subcommand = sc;
        app.argc += argc;
        app
    }
}

impl PartialEq for App {
    fn eq(&self, b: &Self) -> bool {
        let subcommand = self.subcommand == b.subcommand;
        let cmd = self.cmd.get_args().eq(b.cmd.get_args())
            && self.cmd.get_current_dir().eq(&b.cmd.get_current_dir());
        subcommand && cmd && self.argc == b.argc
    }
}

/// Conveniently converts either a `File` or `Output` into an iterator of
/// `String`s, dropping the invalid ones.
fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}
