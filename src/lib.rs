use std::collections::HashSet;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};
use std::{env::current_dir, fs::File, process::Stdio};
use std::{path::PathBuf, process::Command};
mod git_cmd;

#[derive(Debug, PartialEq)]
pub enum Op {
    Status(bool),    // bool: { true: normal, false: short }
    Number(PathBuf), // PathBuf contains command
}

pub struct Opts {
    pub op: Op,
    pub cwd: PathBuf,
    pub gcs: bool, // git-command set? ('add'/'status'/...)
}

impl Opts {
    pub fn cache(&self, create: bool) -> Option<File> {
        let p = PathBuf::from(
            String::from_utf8_lossy(
                &Command::new("git")
                    .args(["rev-parse", "--git-dir"])
                    .current_dir(&self.cwd)
                    .output()
                    .ok()?
                    .stdout,
            )
            .trim(),
        )
        .join("gitnu.txt");
        if create { File::create(p) } else { File::open(p) }.ok()
    }

    pub fn set_op(&mut self, op: Op) {
        match (&self.gcs, &self.op, &op) {
            (false, _, _) => (self.op, self.gcs) = (op, true),
            (_, Op::Status(true), Op::Status(false)) => self.op = op,
            _ => (),
        }
    }
}

pub fn parse(args: Vec<String>) -> (Vec<String>, Opts) {
    // load git commands
    let mut git_cmd = HashSet::with_capacity(git_cmd::GIT_CMD.len());
    git_cmd.extend(git_cmd::GIT_CMD.iter().map(|v| v.to_string()));
    let mut iter = args.iter().skip(1);
    let mut res = Vec::new();
    let mut opts = Opts {
        cwd: current_dir().unwrap_or(".".into()),
        op: Op::Number("git".into()),
        gcs: false,
    };
    while let Some(arg) = iter.next() {
        if !opts.gcs && git_cmd.contains(arg) {
            match arg.as_str() {
                "status" => opts.set_op(Op::Status(true)),
                _ => opts.set_op(Op::Number("git".into())),
            }
        }
        match arg.as_str() {
            "status" => opts.set_op(Op::Status(true)),
            "--short" | "-s" | "--porcelain" => opts.set_op(Op::Status(false)),
            "-x" | "-C" => match opts.gcs {
                false => {
                    if let Some(v) = iter.next() {
                        match arg.as_str() {
                            "-x" => opts.set_op(Op::Number(v.into())),
                            _ => opts.cwd = v.into(),
                        }
                        continue;
                    }
                }
                true => (),
            },
            _ => (),
        }
        res.push(arg.to_string());
    }
    (res, opts)
}

pub fn load(args: Vec<String>, opts: &Opts) -> Vec<PathBuf> {
    fn get_range(arg: &str) -> Option<[usize; 2]> {
        arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
            let (a, b) = arg.split_once("-")?;
            let a = a.parse().ok()?;
            let b = b.parse().unwrap_or(a);
            Some(if a < b { [a, b] } else { [b, a] })
        })
    }
    let mut skip = false;
    let c: Vec<String> =
        opts.cache(false).map(|v| lines(v).collect()).unwrap_or_default();
    args.iter().fold(Vec::new(), |mut r, a| {
        let isf = a.starts_with('-') && !a.starts_with("--"); // is short flag
        match (!skip && !isf, get_range(a)) {
            (true, Some([s, e])) => (s..e + 1)
                .map(|n| (n.checked_sub(1).map(|v| c.get(v)), n.to_string()))
                .for_each(|(o, s)| r.push(o.flatten().unwrap_or(&s).into())),
            _ => r.push(PathBuf::from(a)),
        }
        skip = isf;
        r
    })
}

fn uncolor(f: &str) -> String {
    f.replace("\x1b[31m", "").replace("\x1b[32m", "").replace("\x1b[m", "")
}

fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    return BufReader::new(src).lines().filter_map(|v| v.ok());
}

pub fn status(args: &Vec<PathBuf>, o: Opts, is_normal: bool) -> Option<()> {
    let count = &mut 1;
    let mut su = false;
    let mut git = Command::new("git");
    git.args(["-c", "color.status=always"])
        .current_dir(&o.cwd)
        .args(args)
        .stdout(Stdio::piped());
    let mut writer = o.cache(true).map(LineWriter::new);
    let mut git = git.spawn().ok()?;
    let b = lines(git.stdout.as_mut()?);
    b.filter_map(|line| {
        su |= line.contains("Untracked files:");
        match (is_normal, line.starts_with('\t')) {
            (true, false) => println!("{}", line),
            (true, true) => {
                println!("{}{}", *count, line);
                *count += 1;
                let f = uncolor(&line);
                let mut f = f.split_once('\t')?.1;
                f = if su { f } else { f.split_once(':')?.1.trim_start() };
                return Some(o.cwd.join(f));
            }
            _ => {
                println!("{: <3}{}", *count, line);
                *count += 1;
                return Some(o.cwd.join(&uncolor(&line)[3..]));
            }
        };
        return None;
    })
    .for_each(|v| {
        writer.as_mut().map(|lw| writeln!(lw, "{}", v.display()));
    });
    git.wait().ok();
    writer.map(|mut v| v.flush().ok()).flatten()
}

pub fn core(args: Vec<String>) -> (Vec<PathBuf>, Opts) {
    let (args, opts) = parse(args);
    (load(args, &opts), opts)
}

pub fn run(a: Vec<PathBuf>, opts: Opts) -> Option<()> {
    let sp = |c| Command::new(c).args(&a).spawn().ok()?.wait().map(|_| ()).ok();
    match opts.op {
        Op::Status(normal) => status(&a, opts, normal),
        Op::Number(cwd) => sp(cwd),
    }
}

#[cfg(test)]
mod tests;
