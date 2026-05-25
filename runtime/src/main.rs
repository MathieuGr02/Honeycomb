fn main() {
    println!("Hello, world!");
}
/*
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

    /*let result = mount(
        Some(OsStr::new("proc")),
        "proc",
        Some(OsStr::new("proc")),
        MsFlags::MS_NOEXEC,
        Some(OsStr::new("proc")),
    );*/

    info!("Setting hostname");
    let _ = sethostname("honeycomb").map_err(|e| println!("{}", e));
}
 */
