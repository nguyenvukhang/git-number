// parses opts from arguments
// (determines the state of the app from CLI args)

use crate::shell::get_stdout;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::process::{Command, ExitStatus};

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Status, // gitnu status
    Read,   // gitnu add 2-4
    Bypass, // gitnu log --oneline
    Xargs,  // gitnu -c nvim 2
}

#[derive(Debug, PartialEq, Clone)]
pub struct Opts {
    pub xargs_cmd: Option<String>,
    pub git_dir: Option<String>,
    pub op: OpType,
}

impl Opts {
    /// get git command loaded with git_dir
    fn git_cmd(&self) -> Command {
        let mut git = Command::new("git");
        if let Some(git_dir) = &self.git_dir {
            git.arg("-C");
            git.arg(git_dir);
        }
        git
    }
    // get xargs cmd loaded with cwd
    fn xargs_cmd(&self) -> Result<Command, Error> {
        let cmd = self.xargs_cmd.as_ref().ok_or(Error::new(
            ErrorKind::NotFound,
            "xargs command not found",
        ))?;
        let mut cmd = Command::new(cmd);
        if let Some(git_dir) = &self.git_dir {
            cmd.current_dir(git_dir);
        }
        Ok(cmd)
    }
    pub fn cmd(&self) -> Result<Command, Error> {
        use OpType::*;
        match self.op {
            Read | Status | Bypass => Ok(self.git_cmd()),
            Xargs => self.xargs_cmd(),
        }
    }
    pub fn run(&self, args: Vec<String>) -> Result<ExitStatus, Error> {
        self.cmd()?.args(args).spawn()?.wait()
    }
    fn cache_dir(&self) -> Result<std::path::PathBuf, Error> {
        let mut git = self.git_cmd();
        git.args(["rev-parse", "--path-format=absolute", "--git-dir"]);
        Ok(std::path::PathBuf::from(get_stdout(&mut git)?))
    }
    fn cache_file(&self) -> Result<std::path::PathBuf, Error> {
        Ok(self.cache_dir()?.join("gitnu.txt"))
    }
    pub fn write_cache(&self, content: String) -> Result<(), Error> {
        return std::fs::write(self.cache_file()?, content);
    }
    pub fn read_cache(&self) -> Result<Vec<String>, Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let cache_file = self.cache_file()?;
        let file = File::open(cache_file)?;
        let res: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|v| v.ok())
            .map(|v| v.to_string())
            .collect();
        Ok(res)
    }
    pub fn read_cache2(&self) -> Result<HashMap<u16, String>, Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let cache_file = self.cache_file()?;
        let file = File::open(cache_file)?;
        let mut res: HashMap<u16, String> = HashMap::new();
        let mut count = 1;
        BufReader::new(file)
            .lines()
            .filter_map(|v| v.ok())
            .map(|v| v.to_string())
            .for_each(|v| {
                res.insert(count, v);
                count += 1;
            });
        Ok(res)
    }
}

pub fn get(args: &Vec<String>) -> (Opts, Vec<String>) {
    let mut opts = Opts { op: OpType::Bypass, xargs_cmd: None, git_dir: None };
    let mut set_op = |new_op: OpType| {
        if opts.op == OpType::Bypass {
            opts.op = new_op;
        }
    };
    let mut res: Vec<String> = Vec::new();
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        let mut push = || res.push(String::from(arg));
        match arg.as_str() {
            "add" | "reset" | "diff" | "checkout" => {
                set_op(OpType::Read);
                push()
            }
            "status" => {
                set_op(OpType::Status);
                push()
            }
            "-c" => match it.next() {
                Some(cmd) => {
                    set_op(OpType::Xargs);
                    opts.xargs_cmd = Some(cmd.to_owned());
                }
                None => push(),
            },
            "-C" => match it.next() {
                Some(dir) => opts.git_dir = Some(dir.to_owned()),
                None => push(),
            },
            _ => push(),
        }
    }
    return (opts, res);
}

#[cfg(test)]
fn expected(
    git_dir: Option<&str>,
    xargs_cmd: Option<&str>,
    op: OpType,
) -> Opts {
    let stringify = |v: Option<&str>| match v {
        None => None,
        Some(v) => Some(String::from(v)),
    };
    Opts { git_dir: stringify(git_dir), xargs_cmd: stringify(xargs_cmd), op }
}

#[cfg(test)]
fn received(args: &[&str]) -> Opts {
    let a: Vec<String> = args.iter().map(|v| v.to_string()).collect();
    let (opts, _) = get(&a);
    opts
}

#[test]
fn test_get_opts() {
    // set git_dir
    let rec = received(&["-C", "/dev/null"]);
    let exp = expected(Some("/dev/null"), None, OpType::Bypass);
    assert_eq!(rec, exp);

    // set xargs_cmd
    let rec = received(&["-c", "nvim"]);
    let exp = expected(None, Some("nvim"), OpType::Xargs);
    assert_eq!(rec, exp);

    // set both git_dir and xargs_cmd
    let rec = received(&["-C", "/etc", "-c", "nvim"]);
    let exp = expected(Some("/etc"), Some("nvim"), OpType::Xargs);
    assert_eq!(rec, exp);

    // set both xargs_cmd and git_dir
    let rec = received(&["-c", "nvim", "-C", "/etc"]);
    let exp = expected(Some("/etc"), Some("nvim"), OpType::Xargs);
    assert_eq!(rec, exp);

    // status mode
    let rec = received(&["status", "--short"]);
    let exp = expected(None, None, OpType::Status);
    assert_eq!(rec, exp);

    // read mode
    let rec = received(&["add", "2-4"]);
    let exp = expected(None, None, OpType::Read);
    assert_eq!(rec, exp);

    // read mode with git_dir
    let rec = received(&["-C", "/tmp", "add", "2-4"]);
    let exp = expected(Some("/tmp"), None, OpType::Read);
    assert_eq!(rec, exp);

    // -C flag without value
    let rec = received(&["-C"]);
    let exp = expected(None, None, OpType::Bypass);
    assert_eq!(rec, exp);

    // -C flag with unexpected value
    // (pass on to git)
    let rec = received(&["-C", "status"]);
    let exp = expected(Some("status"), None, OpType::Bypass);
    assert_eq!(rec, exp);

    // -c flag without value
    let rec = received(&["-c"]);
    let exp = expected(None, None, OpType::Bypass);
    assert_eq!(rec, exp);

    // -c flag with unexpected value
    // (just run in anyways)
    let rec = received(&["-c", "status"]);
    let exp = expected(None, Some("status"), OpType::Xargs);
    assert_eq!(rec, exp);
}
