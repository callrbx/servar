use std::io::ErrorKind;
use std::net::SocketAddr;

use build_html::*;
use once_cell::sync::OnceCell;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use tokio::signal::ctrl_c;
use warp::filters::BoxedFilter;
use warp::path::FullPath;
use warp::reject::not_found;
use warp::reply::{html, Reply};
use warp::Filter;

use crate::GlobalArgs;
use structopt::StructOpt;
use tokio::io;

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Serves a directory via HTTP")]
pub struct Args {
    #[structopt(
        short = "d",
        long = "directory",
        help = "directory to serve",
        default_value = "."
    )]
    directory: String,
}

static SERVE_DIR: OnceCell<String> = OnceCell::new();

/// Helper function to convert a path to a relative path
fn get_dir_rel(path: &PathBuf) -> Option<String> {
    let rel_path = format!("/{}", path.strip_prefix(SERVE_DIR.get()?).ok()?.to_str()?);
    Some(rel_path)
}

/// The routes used by the program
pub fn routes(serve_dir: String) -> BoxedFilter<(impl Reply,)> {
    // globalize module arguments for read use in async functions
    SERVE_DIR.set(serve_dir).unwrap();

    let logging = warp::log::custom(|info| {
        println!("Request: '{}',\tStatus: '{}'", info.path(), info.status())
    });

    let handle_files = warp::fs::dir(SERVE_DIR.get().unwrap());
    let handle_directories = warp::get()
        .and(warp::path::full())
        .and_then(dir_to_html)
        .map(html);

    handle_files.or(handle_directories).with(logging).boxed()
}

/// Converts the URL route of a folder to an HTML string of the contents
async fn dir_to_html(route: FullPath) -> Result<String, warp::reject::Rejection> {
    let path = PathBuf::from(SERVE_DIR.get().unwrap()).join(&route.as_str()[1..]);

    let content = HtmlPage::new()
        .with_title(format!(
            "Directory listing for {}",
            get_dir_rel(&path).unwrap()
        ))
        .with_container(
            Container::new(ContainerType::Main)
                .with_header(
                    1,
                    format!("Directory listing for {}", get_dir_rel(&path).unwrap()),
                )
                .with_raw("<hr>")
                .with_container(items_list(path.as_path(), &route).ok_or_else(not_found)?)
                .with_raw("<hr>"),
        )
        .to_html_string();

    Ok(content)
}

/// Generate a container with links to all items
fn items_list(path: &Path, route: &FullPath) -> Option<Container> {
    let mut links = Container::new(ContainerType::UnorderedList);

    if route.as_str() != "/" {
        let parent = path
            .parent()
            .and_then(|path| path.strip_prefix(SERVE_DIR.get()?).ok())
            .and_then(Path::to_str)
            .map(|s| format!("{}", s))?;
        links.add_link(parent, "..");
    }

    let mut entries: Vec<(String, String)> = read_dir(&path)
        .ok()?
        .filter_map(|res| res.ok().map(|x| x.path()))
        .filter_map(format_path)
        .collect();
    entries.sort_by_cached_key(|(name, _)| name.to_string());
    for (item, item_path) in entries {
        links.add_link(item_path, item);
    }

    Some(links)
}

/// Create item name and relative path from given path
fn format_path(path: PathBuf) -> Option<(String, String)> {
    let item = format!(
        "{}{}",
        path.file_name()?.to_str()?,
        match path.is_dir() {
            true => "/",
            false => "",
        }
    );

    Some((item, get_dir_rel(&path).unwrap()))
}

/// Main exec function for the HTTPDir server module
pub async fn exec(gargs: GlobalArgs, mode_args: Args) -> io::Result<()> {
    let addr = format!("{}:{}", gargs.ip, gargs.port);
    let sock: SocketAddr = match addr.parse() {
        Ok(s) => s,
        Err(_) => {
            return Err(io::Error::new(ErrorKind::AddrNotAvailable, addr));
        }
    };

    let serve_dir = mode_args.directory.clone();
    let handle = tokio::spawn(warp::serve(routes(serve_dir)).bind(sock));

    println!(
        "Serving HTTP on {} port {} ({})",
        gargs.ip, gargs.port, sock
    );
    ctrl_c().await.expect("Unalbe to get Ctrl+C signal");
    handle.abort();
    handle.await.unwrap_or(());

    return Ok(());
}
