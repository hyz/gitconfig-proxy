#![feature(exit_status_error)]
//#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
// cargo-deps: async-recursion, async-process, futures-lite, time
// You can also leave off the version number, in which case, it's assumed
// to be "*".  Also, the `cargo-deps` comment *must* be a single-line
// comment, and it *must* be the first thing in the file, after the
// shebang.
// Multiple dependencies should be separated by commas:
// // cargo-deps: time="0.1.25", libc="0.2.5"
use anyhow::{Context, Error, Result}; // type Result = std::result::Result<(), Box<dyn std::error::Error>>;
                                      //use async_process::{Command, Stdio};
use std::path::{Path, PathBuf};

//use futures_lite::{future, io, prelude::*};
//use futures::stream::Stream;
//use futures_async_stream::stream;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};
use tokio::process::Command;

use std::process::{ExitStatus, Stdio};
use url::{Host, Position, Url};

//static NOT_FOUND: Error = Error::from(io::Error::from(io::ErrorKind::NotFound));

async fn git_clone(url: Url, args: &[&str]) -> tokio::io::Result<ExitStatus> {
    Command::new("git")
        .args(&["clone"])
        .arg(url.as_str())
        .args(args)
        .status()
        .await
}
async fn git_fetch() -> tokio::io::Result<ExitStatus> {
    Command::new("git").args(&["fetch", "-p"]).status().await
}
async fn git_merge() -> tokio::io::Result<ExitStatus> {
    Command::new("git").arg("merge").status().await
}
async fn git_config_get(path: &str) -> tokio::io::Result<std::process::Output> {
    Command::new("git")
        .args(&["config", "--local", "--get", path])
        .output()
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    git_script().await?;
    //https2git().await

    //echo "#... `pwd`    $origin"
    //#git fetch -p #--rebase && git submodule update --init --recursive
    //#git merge # pull --rebase && git submodule update --init --recursive

    // echo "#~~~ `pwd`    $origin"

    Ok(())
}

async fn git_config_remote_origin_url(url: Url) -> Result<()> {
    //git config --local remote.origin.url ...
    let mut cmd = Command::new("git");
    cmd.args(&["config", "--local", "remote.origin.url", url.as_str()]); //.arg(url.as_str());
    println!("{:?}", cmd);
    let status = cmd.status().await?;
    let x = status;
    //TODO: Error::from(std::io::Error::from(std::io::ErrorKind::NotFound))

    Ok(())
}

async fn config_get_remote_origin_url() -> Result<Url> {
    // git config --local --get remote.origin.url
    // git remote get-url origin
    async fn origin_url() -> Result<Url> {
        let output = git_config_get("remote.origin.url").await?;
        if !output.status.success() {
            panic!("{:?}", output);
        }
        let output = output.stdout.as_slice(); // output.lines()
        let output = std::str::from_utf8(output)?;
        Url::parse(output).context(String::from(output)) //.map_err(|e| e.into())
    }
    //async fn config_get_2() -> Result<Url> {
    //    let output = Command::new("git")
    //        .args(&["config", "--local", "--get", "remote.origin.url"])
    //        .stdout(Stdio::piped())
    //        .spawn()?;

    //    let output = output.stdout.take()?;
    //    output.

    //    if !output.status.success() {
    //        panic!("{:?}", output);
    //    }
    //    let url = Url::parse(std::str::from_utf8(output.stdout.as_slice()).unwrap())?;
    //    return Ok(Url::from(""));
    //}
    origin_url().await
}

async fn git_script() -> Result<()> {
    fn fix_url(mut url: Url) -> Url {
        if url.scheme() == "git" && url.host() == Some(Host::Domain("github.com")) {
            //url.set_scheme("https").expect();
            let mut u = Url::parse("https://github.com").unwrap();
            u.set_path(url.path());
            url = u;
        }
        //https://gh.api.99988866.xyz/https://github.com/rust-lang/crates.io-index
        let mut nurl = Url::parse("https://gh.api.99988866.xyz/").unwrap();
        nurl.set_path(url.as_str());
        nurl
    }
    //let target = std::env::args().nth(1); //.map(PathBuf::from);
    if let Some(giturl) = std::env::args().nth(1) {
        let url = Url::parse(&giturl)?; //.map(PathBuf::from);
        assert!(url.host() == Some(Host::Domain("github.com")));
        git_clone(url, &["--depth", "1"]).await?.exit_ok()?;
    } else {
        let url = config_get_remote_origin_url().await?;
        match url.host() {
            Some(Host::Domain("github.com")) => git_config_remote_origin_url(fix_url(url)).await?,
            Some(Host::Domain("gh.api.99988866.xyz")) => {}
            _ => {
                panic!("{}: gh.api.99988866.xyz/github.com/", url)
                //Error::from(std::io::Error::from(std::io::ErrorKind::NotFound))
            }
        }
        git_fetch().await?.exit_ok()?;
        git_merge().await?.exit_ok()?;
    }
    Ok(())

    // let not_found = Error::from(io::Error::from(io::ErrorKind::NotFound));
    // let conf = resolv_config(&target).ok_or(not_found)?;

    // let home = std::env::var("HOME").map(PathBuf::from)?;
    // if !conf.exists() || conf.canonicalize()? == home {
    //     panic!("invalid: {}", target.display());
    // }

    // let mut gitx = Command::new("sed");
    // gitx.args(&[
    //     "-i",
    //     r"/^\s*url\s*=\s*https:/s/https:/git:/",
    //     conf.join(".git/config").to_string_lossy().as_ref(),
    // ]); // sed -p -i '/^\s*url\s*=\s*https:/s/https:/git:/' $1/.git/config

    // run(&mut gitx, None).await
}
//  async fn https2git() -> Result<()> {
//  let target = std::env::args_os()
//  .nth(1)
//  .map(PathBuf::from)
//  .expect("... <config>"); //Path::new(&arg1);
//  if !target.exists() {
//  panic!("not exists: {}", target.display());
//  }
//  let not_found = Error::from(io::Error::from(io::ErrorKind::NotFound));
//  let conf = resolv_config(&target).ok_or(not_found)?;

//  let home = std::env::var("HOME").map(PathBuf::from)?;
//  if !conf.exists() || conf.canonicalize()? == home {
//  panic!("invalid: {}", target.display());
//  }

//  let mut gitx = Command::new("sed");
//  gitx.args(&[
//  "-i",
//  r"/^\s*url\s*=\s*https:/s/https:/git:/",
//  conf.join(".git/config").to_string_lossy().as_ref(),
//  ]); // sed -p -i '/^\s*url\s*=\s*https:/s/https:/git:/' $1/.git/config

//  run(&mut gitx, None).await
//  }

//  #[async_recursion::async_recursion]
//  async fn run(gitx: &mut Command, opt: Option<String>) -> Result<()> {
//  if let Some(_) = opt {
//  return run(gitx, None).await;
//  }
//  println!("{:?}", gitx);

//  let mut child = gitx.stdout(Stdio::piped()).spawn()?;
//  //println!("{:?}", child);

//  let mut lines = io::BufReader::new(child.stdout.take().unwrap()).lines();
//  while let Some(line) = lines.next().await {
//  println!("{}", line?);
//  }

//  std::process::exit(child.status().await.unwrap().code().unwrap()) //Ok(())
//  }

// fn replace_https(uri: String) -> String {
//     if uri.starts_with("https") {
//         uri.replacen("https", "git", 1)
//     } else if uri.starts_with("http") {
//         uri.replacen("http", "git", 1)
//     } else {
//         uri
//     }
// }
// // cargo-deps: anyhow, glob

// // fn copy2<P: AsRef<Path>, S: AsRef<str>>(pat: S, dir: P) -> Result<()> {
// //     for entry in glob::glob(pat.as_ref())? {
// //         let path = entry?;
// //         println!("{:?}", path.display());
// //         //let ft = path.metadata()?.file_type();
// //         std::fs::copy(&path, dir.as_ref().join(path.file_name().unwrap()))?;
// //     }
// //     Ok(())
// // }
// fn copy2<P: AsRef<Path>, S: AsRef<str>>(pat: S, dir: P) -> Result<()> {
//     for entry in glob::glob(pat.as_ref())? {
//         let path = entry?;
//         let dest = dir.as_ref().join(path.file_name().unwrap());
//         println!("{:?}", path.display()); // path.metadata()?.file_type();
//         std::fs::copy(&path, &dest)?;
//     }
//     Ok(())
// }

// fn _1main() -> Result<()> {
//     let mut args = std::env::args_os().skip(1);
//     let pat = args.next().expect("... <pattern> <dest>");
//     let dest = args
//         .next()
//         .map(PathBuf::from)
//         .expect("... <pattern> <dest>"); //Path::new(&arg1);

//     //safe_check(src.canonicalize()?, dest.canonicalize()?);
//     copy2(pat.to_string_lossy(), dest)
// }
// fn _2main() -> Result<()> {
//     let mut args = std::env::args_os().skip(1).map(PathBuf::from);
//     let src_dir = args.next().expect("... <src> <dest>"); //Path::new(&arg1);
//     let mut dest_dir = args.next().expect("... <src> <dest>");

//     if !src_dir.exists() {
//         panic!("not exists: {}", src_dir.display());
//     }
//     safe_check(src_dir.canonicalize()?, dest_dir.canonicalize()?);

//     let endv: Vec<_> = std::env::args().map(|x| x.ends_with("/")).collect();
//     if !endv[1] && endv[2] {
//         dest_dir.push(src_dir.file_name().unwrap());
//         std::fs::create_dir_all(&dest_dir)?;
//     }
//     recur_tree(&src_dir, 0, &|from, _ft, _lev| {
//         let p = from.strip_prefix(&src_dir).unwrap();
//         Some(dest_dir.join(p))
//     })
// }

// fn recur_tree<P, Pred>(src_dir: P, lev: u32, pred: &Pred) -> Result<()>
// where
//     P: AsRef<Path>,
//     Pred: Fn(&Path, std::fs::FileType, u32) -> Option<PathBuf>,
// {
//     for entry in fs::read_dir(src_dir)?.map(Result::unwrap) {
//         let ft: std::fs::FileType = entry.metadata()?.file_type();
//         let path = entry.path();
//         if let Some(dest) = pred(&path, ft, lev) {
//             if ft.is_dir() {
//                 if !dest.exists() {
//                     fs::create_dir_all(&dest)?;
//                 }
//                 recur_tree(path, lev + 1, pred)?;
//             } else if ft.is_file() {
//                 println!("{} -> {}", path.display(), dest.display());
//                 fs::copy(path, dest)?;
//             } else if ft.is_symlink() {
//                 panic!("symlink {:?}: {}", ft, path.display(),);
//             } else {
//                 panic!("")
//             }
//         }
//     }
//     Ok(())
// }

// fn recur_tree2<P, Pred, Op>(src_dir: P, pred: &Pred, op: &Op, lev: u32) -> Result<()>
// where
//     P: AsRef<Path>,
//     Pred: Fn(&Path, std::fs::FileType, u32) -> Option<PathBuf>,
//     Op: Fn(&Path, std::fs::FileType, u32, &Path) -> std::io::Result<()>,
// {
//     for entry in fs::read_dir(src_dir)?.map(Result::unwrap) {
//         let ft: std::fs::FileType = entry.metadata()?.file_type();
//         let path = entry.path();
//         if let Some(dest) = pred(&path, ft, lev) {
//             op(&path, ft, lev, &dest)?;
//             if ft.is_dir() {
//                 recur_tree2(path, pred, op, lev + 1)?;
//             }
//         }
//     }
//     Ok(())
// }

// fn sync_tree<P: AsRef<Path>>(src_dir: P, dest_dir: PathBuf) -> Result<()> {
//     for entry in fs::read_dir(src_dir)?.map(Result::unwrap) {
//         let ft = entry.metadata()?.file_type();
//         let path = entry.path();
//         //println!("{:?} KB", dir.metadata()?.len() / 1000);
//         let dest = dest_dir.join(path.file_name().unwrap());
//         if ft.is_dir() || ft.is_file() {
//             println!("{} -> {}", path.display(), dest.display());
//             if ft.is_dir() {
//                 fs::create_dir_all(&dest)?;
//                 recur_copy_tree(&path, dest)?;
//             } else {
//                 fs::copy(path, dest)?;
//             }
//         } else {
//             eprintln!("!skipped {:?}: {}", ft, path.display(),);
//         }
//     }
//     Ok(())
// }

// fn safe_check(src: PathBuf, dest: PathBuf) {
//     println!("{} -> {}", src.display(), dest.display());
//     if dest.starts_with(&src) {
//         panic!("**dangerous**: `{}` '{}'", src.display(), dest.display());
//     }
// }

// fn __recur_tree(src_dir: &Path, partbuf: PathBuf, dest_dir: &Path) -> std::io::Result<()> {
//     for entry in fs::read_dir(src_dir)?.map(Result::unwrap) {
//         let meta = entry.metadata()?;
//         let path = entry.path();
//         //println!("{:?} KB", dir.metadata()?.len() / 1000);
//         let pb = partbuf.join(path.file_name().unwrap());
//         let dest = dest_dir.join(&pb);
//         if meta.is_dir() || meta.is_file() {
//             println!(".../{} -> {}", pb.display(), dest.display());
//             if meta.is_dir() {
//                 fs::create_dir_all(dest).expect("create dest dir");
//                 __recur_tree(&path, pb, dest_dir)?;
//             } else {
//                 //fs::copy(path, dest);
//             }
//         } else {
//             eprintln!(
//                 "!!!skipped {}/{} -> {}: {:?}",
//                 src_dir.display(),
//                 pb.display(),
//                 dest.display(),
//                 meta.file_type()
//             );
//             //panic!("Unknow");
//         }
//     }
//     Ok(())
// }

fn resolv_config(path: &Path) -> Option<PathBuf> {
    fn resolv(path: &Path) -> Option<PathBuf> {
        //path.components().count();
        for a in path.ancestors().take(2) {
            //println!("{} {}", a.display(), a.exists());
            if a.exists() && a.join(".git/config").exists() {
                return Some(a.into());
            }
        }
        return None;
    }

    if path.is_file() {
        if path.ends_with(".git/config") {
            let mut pb = path.to_path_buf();
            pb.pop();
            pb.pop();
            return Some(pb); //(path.into());
        } else {
            return path.parent().and_then(resolv);
        }
    }
    return resolv(path);
}
