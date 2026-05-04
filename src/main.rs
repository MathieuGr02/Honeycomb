use log::info;
use nix::{
    sched::{CloneFlags, unshare},
    unistd::{chdir, chroot, getpid, sethostname},
};
use tracing::{level_filters::LevelFilter, trace};

use std::{env, io, process::Command};

use honeycomb::logger::init_logger;

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
    info!("Setting hostname");
    let _ = sethostname("honeycomb").map_err(|e| println!("{}", e));
}

fn main() {
    init_logger(LevelFilter::TRACE);

    let args: Vec<String> = env::args().collect();
    if args.contains(&"run".to_string()) {
        bootstrap();
    } else if args.contains(&"child".to_string()) {
        child();
    }
}
