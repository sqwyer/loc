extern crate walkdir;
// extern crate same_file;
use walkdir::WalkDir;
use std::{fs::metadata, fs::File, path::{Path}};
use quicli::prelude::*;
use structopt::StructOpt;
use std::io::Read;
// use same_file::is_same_file;

#[derive(Debug, StructOpt)]
struct Cli {
    dir: String,
}

// fn contains_loop<P: AsRef<Path>>(path: P) -> std::io::Result<Option<(PathBuf, PathBuf)>> {
//     let path = path.as_ref();
//     let mut path_buf = path.to_path_buf();
//     while path_buf.pop() {
//         if is_same_file(&path_buf, path)? {
//             return Ok(Some((path_buf, path.to_path_buf())));
//         } else if let Some(looped_paths) = contains_loop(&path_buf)? {
//             return Ok(Some(looped_paths));
//         }
//     }
//     return Ok(None);
// }

fn _read_file(path: &Path) -> Vec<u8> {
    let mut file_content = Vec::new();
    let mut file = File::open(&path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}

fn main() -> CliResult {
    // let invalid: Vec<Option<&str>> = vec![Some(".git/"), Some("target/"), Some("node_modules/")];
    let args = Cli::from_args();
    let mut files_cnt: i64 = 0;
    let mut lines_cnt: i64 = 0;
    for file in WalkDir::new(&args.dir).into_iter().filter_map(|file| file.ok()) {
        let path = &file.path();
        if metadata(path).unwrap().is_file() {
            // match contains_loop(&path).unwrap() {
            //     Ok(_) => _,
            //     Err(_) => continue,
            // }
            // if let Some(".git/") = &path.to_str() continue;
            // if !invalid.contains(&path.to_str()) {
            let buf = _read_file(&path);
            let s = match std::str::from_utf8(&buf) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let mut cnt: i64 = 0;
            for _ in s.lines() {
                cnt += 1;
            }
            lines_cnt+=cnt;
            files_cnt+=1;
            // }
        }
    }
    println!("{} lines of code accross {} file(s).", lines_cnt, files_cnt);
    Ok(())
}