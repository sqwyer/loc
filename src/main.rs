extern crate walkdir;
use walkdir::WalkDir;
use std::fs::metadata;
use quicli::prelude::*;
use structopt::StructOpt;
use std::ffi::OsStr;

#[derive(Debug, StructOpt)]
struct Cli {
    dir: String,
}

fn main() -> CliResult {
    let invalid: Vec<Option<&str>> = vec![Some("exe"),Some("mem"),Some("png"),Some("node"),Some("jpg"),Some("jpeg"),Some("bin"),Some("md"),Some("json"),Some("txt"),Some("gz"),Some("zip"),Some("o")];
    let args = Cli::from_args();
    let mut files_cnt: i64 = 0;
    let mut lines_cnt: i64 = 0;
    for file in WalkDir::new(&args.dir).into_iter().filter_map(|file| file.ok()) {
        let path = &file.path();
        if metadata(path).unwrap().is_file() && !path.starts_with("./.git") && !path.starts_with("./node_modules") && !path.starts_with("./target") {
            if !invalid.contains(&path.extension().and_then(OsStr::to_str)) {
            let file = read_file(&path)?;
            let mut cnt: i64 = 0;
            for _ in file.lines() {
                cnt += 1;
            }
            lines_cnt+=cnt;
            files_cnt+=1;
            }
        }
    }
    println!("{} lines of code accross {} file(s).", lines_cnt, files_cnt);
    Ok(())
}