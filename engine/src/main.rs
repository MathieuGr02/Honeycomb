use actix_web::{App, HttpServer, web};
use log::{error, info};
use nix::{
    mount::{MsFlags, mount},
    sched::{CloneFlags, unshare},
    unistd::{chdir, chroot, getpid, sethostname},
};
use tracing::{level_filters::LevelFilter, trace};

use std::{
    ffi::OsStr,
    io::{self},
    process::Command,
};

use honeycomb_engine::{logger::init_logger, routes::containers::containers_scope};

const SOCKET: &str = "./honeycomb.socket";

async fn api_server() {
    HttpServer::new(|| App::new().service(containers_scope()))
        .bind_uds(SOCKET)
        .unwrap()
        .run()
        .await;
}

fn bootstrap() {
    info!("Initialising");
    trace!("Run pid: {}", getpid());

    let _ = unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS)
        .map_err(|e| panic!("Unable to detach process {}", e));

    let _ = Command::new("/usr/local/bin/Honeycomb")
        .arg("child")
        .stderr(io::stderr())
        //.stdin(io::stdin())
        .stdout(io::stdout())
        .output()
        .map_err(|e| panic!("{}", e));
}

fn child() {
    trace!("Child pid: {}", getpid());

    info!("chroot ./fs");
    let _ = chroot("./fs").map_err(|e| println!("{}", e));
    info!("chdir /");
    let _ = chdir("/").map_err(|e| println!("{}", e));

    let result = mount(
        Some(OsStr::new("proc")),
        "proc",
        Some(OsStr::new("proc")),
        MsFlags::MS_NOEXEC,
        Some(OsStr::new("proc")),
    );

    info!("Setting hostname");
    let _ = sethostname("honeycomb").map_err(|e| println!("{}", e));
}

#[actix_web::main]
async fn main() {
    init_logger(LevelFilter::TRACE);
    api_server().await;
    /*
    let args: Vec<String> = env::args().collect();
    if args.contains(&"run".to_string()) {
        bootstrap();
    } else if args.contains(&"child".to_string()) {
        child();
    }
    */
}
