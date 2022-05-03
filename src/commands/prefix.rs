//! `copu` subcommand - example of how to write a subcommand
//#![feature(generators, proc_macro_hygiene, stmt_expr_attributes)]
// cargo-deps: async-recursion, async-process, futures-lite, time
// You can also leave off the version number, in which case, it's assumed
// to be "*".  Also, the `cargo-deps` comment *must* be a single-line
// comment, and it *must* be the first thing in the file, after the
// shebang.
// Multiple dependencies should be separated by commas:
// // cargo-deps: time="0.1.25", libc="0.2.5"

//use anyhow::{Context, Error, Result}; // type Result = std::result::Result<(), Box<dyn std::error::Error>>;
//use async_process::{Command, Stdio};
use std::path::{Path, PathBuf};

//use futures_lite::{future, io, prelude::*};
//use futures::stream::Stream;
//use futures_async_stream::stream;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};
use tokio::process; //::Command;

use std::process::{ExitStatus, Stdio};
use url::{Host, Position, Url};

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::Myex2Config;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::{Parser, ValueHint};

type Result<T, E = crate::error::Error> = std::result::Result<T, E>;

/// `copu` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>

#[derive(Debug)]
struct PrefixUrl(Option<Url>);
fn parse_prefixurl(s: &str) -> Result<PrefixUrl> {
    Ok(PrefixUrl(if s.is_empty() {
        None
    } else {
        Some(Url::parse(s)?)
    }))
}

#[derive(Command, Debug, Parser)]
pub struct Subcommand {
    /// prefix url
    #[clap(long, parse(try_from_str = parse_prefixurl), value_hint = ValueHint::Url)]
    prefixurl: Option<PrefixUrl>,

    /// To whom are we saying copu?
    //#[clap(parse(try_from_str = url_parse), value_hint = ValueHint::Url)]
    //url: Url,
    #[clap(parse(from_os_str), value_hint = ValueHint::DirPath)]
    repo: PathBuf,
}
fn url_parse(s: &str) -> Result<Url> {
    let url = Url::parse(s)?;
    Ok(url)
}

impl Runnable for Subcommand {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        println!("Clone, {:?} {:?}", &config, self);

        futures::executor::block_on(self.main()).unwrap();
        //clone_or_pull().await?;
        //https2git().await
    }
}

impl config::Override<Myex2Config> for Subcommand {
    fn override_config(
        &self,
        mut config: Myex2Config,
    ) -> std::result::Result<Myex2Config, FrameworkError> {
        // if !self.recipient.is_empty() {
        //     config.copu.recipient = self.recipient.join(" ");
        // }
        Ok(config)
    }
}

impl Subcommand {
    async fn fixurl(&self) -> Result<Url> {
        let mut useurl = git_config_get_remote_origin_url().await?;
        if let Some(PrefixUrl(prefixurl)) = self.prefixurl.as_ref() {
            let path = useurl.path();
            let origin = if path.starts_with("/https:") || path.starts_with("/http:") {
                Url::parse(path.strip_prefix('/').unwrap())?
            } else {
                useurl.clone()
            };

            if let Some(proxy) = prefixurl {
                if useurl.host() != proxy.host() {
                    let url = prefix_url(proxy.clone(), &origin);
                    useurl = git_config_remote_origin_url(url).await;
                }
            } else {
                if useurl.host() != origin.host() {
                    useurl = git_config_remote_origin_url(origin).await;
                }
            }
        }
        Ok(useurl)
    }

    async fn main(&self) -> Result<()> {
        let cwd = std::env::current_dir().unwrap();
        let (cwd, useurl) = if self.repo == cwd {
            (cwd, self.fixurl().await?)
        } else {
            let _scope = scopeguard::guard(cwd.clone(), |cwd| {
                std::env::set_current_dir(&cwd).unwrap();
            });
            std::env::set_current_dir(&self.repo).unwrap();
            (self.repo.clone(), self.fixurl().await?)
        };

        println!("###=== {}\t{}", cwd.display(), useurl);
        Ok(())
    }

    // async fn clone_or_pull(&self) -> Result<()> {
    //     // let _guard = scopeguard::guard(std::env::current_dir().unwrap(), |cwd| {
    //     //     std::env::set_current_dir(&cwd).unwrap();
    //     // });

    //     let fixed_url = if self.url.host() == Some(Host::Domain("github.com")) && true {
    //         if self.prefixurl {
    //             proxy_github(self.url.clone())
    //         } else {
    //             self.url.clone()
    //         }
    //     } else {
    //         self.url.clone()
    //     };

    //     git_clone(&["--depth", "1"], &fixed_url, self.repo.as_ref())
    //         .await?
    //         .exit_ok()?;

    //     let lcd = self.repo.as_ref().map_or_else(
    //         || self.url.path_segments().unwrap().last().unwrap().into(),
    //         |x| x.to_string_lossy(),
    //     );
    //     println!("#=== {}\t{}", lcd, self.url);
    //     Ok(())

    //     // //let target = std::env::args().nth(1); //.map(PathBuf::from);
    //     // if let Some(url) = std::env::args_os().nth(1) {
    //     //     let url = url.to_string_lossy();
    //     //     let url = url.as_ref();
    //     //     let lcd = std::env::args_os().nth(2);
    //     //     let lcd = lcd.as_ref().map(Path::new);

    //     //     if let Ok(mut url) = Url::parse(url) {
    //     //         assert!(url.scheme() == "https");
    //     //         assert!(matches!(
    //     //             url.host_str(),
    //     //             Some("github.com") | Some("gitlab.com")
    //     //         ));
    //     //         if url.host() == Some(Host::Domain("github.com")) {
    //     //             url = proxy_github(url);
    //     //         }

    //     //         //git_clone::<_, _, &Path>(&["--depth", "1"], url.clone(), None) // None::<&Path>
    //     //         git_clone(&["--depth", "1"], &url, lcd).await?.exit_ok()?;

    //     //         let lcd = lcd.map_or_else(
    //     //             || url.path_segments().unwrap().last().unwrap().into(),
    //     //             |x| x.to_string_lossy(),
    //     //         );
    //     //         println!("#=== {}\t{}", lcd, url);
    //     //         //#git fetch -p #--rebase && git submodule update --init --recursive
    //     //         //#git merge # pull --rebase
    //     //     } else {
    //     //         let path = Path::new(url);
    //     //         if !path.exists() {
    //     //             panic!("{}", url);
    //     //         }
    //     //         std::env::set_current_dir(&path).unwrap();
    //     //         git_pull().await?;
    //     //     }
    //     //     Ok(())
    //     // } else {
    //     //     git_pull().await
    //     // }

    //     // let not_found = Error::from(io::Error::from(io::ErrorKind::NotFound));
    //     // let conf = resolv_config(&target).ok_or(not_found)?;

    //     // let home = std::env::var("HOME").map(PathBuf::from)?;
    //     // if !conf.exists() || conf.canonicalize()? == home {
    //     //     panic!("invalid: {}", target.display());
    //     // }

    //     // let mut gitx = Command::new("sed");
    //     // gitx.args(&[
    //     //     "-i",
    //     //     r"/^\s*url\s*=\s*https:/s/https:/git:/",
    //     //     conf.join(".git/config").to_string_lossy().as_ref(),
    //     // ]); // sed -p -i '/^\s*url\s*=\s*https:/s/https:/git:/' $1/.git/config

    //     // run(&mut gitx, None).await
    // }
}
fn prefix_url(mut pfx: Url, url: &Url) -> Url {
    pfx.set_path(url.as_str());
    pfx
}

//static NOT_FOUND: Error = Error::from(io::Error::from(io::ErrorKind::NotFound));

const PROXY_HOST_99988866: &str = "gh.api.99988866.xyz";
const PROXY_HOST_JYWWW: &str = "cfwka.jywww.workers.dev";
const PROXY_HOST: &str = PROXY_HOST_JYWWW;

//const PROXY_URL: &str = "https://gh.api.99988866.xyz/";
//https://gh.api.99988866.xyz/https://github.com/rust-lang/crates.io-index
const PROXY_URL: &str = "https://{PROXY_HOST_JYWWW}/";

//#[tokio::main]
//async fn main() -> Result<()> {
//    clone_or_pull().await?;
//    //https2git().await
//    Ok(())
//}

async fn git_clone<I, S, P>(args: I, url: &Url, path: Option<P>) -> tokio::io::Result<ExitStatus>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
    P: AsRef<Path>,
{
    let mut cmd = process::Command::new("git");
    cmd.args(&["clone"]).args(args).arg(url.as_str());
    if let Some(path) = path {
        cmd.arg(path.as_ref().as_os_str());
    }
    cmd.status().await
}
async fn git_fetch() -> tokio::io::Result<ExitStatus> {
    process::Command::new("git")
        .args(&["fetch", "-p"])
        .status()
        .await
}
async fn git_merge() -> tokio::io::Result<ExitStatus> {
    process::Command::new("git").arg("merge").status().await
}
async fn git_config_get(path: &str) -> tokio::io::Result<std::process::Output> {
    process::Command::new("git")
        .args(&["config", "--local", "--get", path])
        .output()
        .await
}
async fn git_log_1() -> tokio::io::Result<ExitStatus> {
    process::Command::new("git")
        .arg("log")
        .args(&["-1"])
        .status()
        .await
}

fn proxy_github(url: Url) -> Url {
    // url.scheme() == "git" &&
    if url.host() == Some(Host::Domain("github.com")) {
        //url.set_scheme("https").expect();
        let mut upath = Url::parse("https://github.com").unwrap();
        upath.set_path(url.path());
        let mut nurl = Url::parse(&format!("https://{PROXY_HOST}/")).unwrap();
        nurl.set_path(upath.as_str());
        nurl
    } else {
        url
    }
}
fn remove_proxy(url: Url) -> Url {
    let path = url.path();
    if path.starts_with("/https:") || path.starts_with("/http:") {
        let url = path.strip_prefix('/').unwrap();
        return Url::parse(url).unwrap();
    }
    url
}
fn remove_gh_api_99988866_xyz(url: Url) -> Url {
    if url.host() == Some(Host::Domain(PROXY_HOST_99988866)) {
        //println!("{:?} {}", url.host(), url.path());
        return Url::parse(url.path().strip_prefix('/').unwrap()).unwrap();
    }
    url
}
fn schema_git_to_https(url: Url) -> Url {
    if url.scheme() == "git" && url.host() == Some(Host::Domain("github.com")) {
        let mut gh = Url::parse("https://github.com").unwrap();
        gh.set_path(url.path());
        return gh;
    }
    url
}

async fn git_config_get_remote_origin_url() -> Result<Url> {
    // git config --local --get remote.origin.url // git remote get-url origin
    async fn origin_url() -> Result<Url> {
        let output = git_config_get("remote.origin.url").await?;
        if !output.status.success() {
            panic!("{:?}", output);
        }
        let output = output.stdout.as_slice(); // output.lines()
        let output = std::str::from_utf8(output)?;
        Url::parse(output).map_err(|e| e.into()) //.context(String::from(output)) //.map_err(|e| e.into())
    }
    origin_url().await
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
}
async fn git_config_remote_origin_url(url: Url) -> Url {
    //git config --local remote.origin.url ...
    let mut cmd = process::Command::new("git");
    cmd.args(&["config", "--local", "remote.origin.url", url.as_str()]); //.arg(url.as_str());
                                                                         //println!("{:?}", cmd);
    let status = cmd.status().await.unwrap();
    let x = status;
    //TODO: Error::from(std::io::Error::from(std::io::ErrorKind::NotFound))
    url
}

async fn git_pull() -> Result<()> {
    let url = git_config_get_remote_origin_url().await?;

    let url = if url.scheme() == "git" && url.host() == Some(Host::Domain("github.com")) {
        git_config_remote_origin_url(schema_git_to_https(url)).await
    } else {
        url
    };

    let url = match url.host() {
        Some(Host::Domain(PROXY_HOST_99988866)) => {
            git_config_remote_origin_url(remove_gh_api_99988866_xyz(url)).await
        }
        //Some(Host::Domain(PROXY_HOST_JYWWW)) && ... => {
        //    git_config_remote_origin_url(remove_jywww(url)).await
        //}
        Some(Host::Domain("github.com")) => {
            git_config_remote_origin_url(proxy_github(url)).await ////////////////////  <<<===
        }
        _ => url,
    };

    //let url = git_config_get_remote_origin_url().await?;
    let current_dir = std::env::current_dir().unwrap();
    println!("# {}\t{}", current_dir.display(), url);
    //defer! {}
    let _guard = scopeguard::guard(url, |url| {
        println!("#=== {}\t{}", current_dir.display(), url);
        //#git fetch -p #--rebase && git submodule update --init --recursive
        //#git merge # pull --rebase && git submodule update --init --recursive
    });
    git_fetch().await?.exit_ok()?;
    git_merge().await?.exit_ok()?;
    git_log_1().await?.exit_ok()?;
    Ok(())
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
