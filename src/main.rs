use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use libbpf_rs::{TcHookBuilder, TC_EGRESS, TC_INGRESS};

mod prio {
    include!(concat!(env!("OUT_DIR"), "/prio.skel.rs"));
}

use prio::*;

#[derive(Debug, Parser)]
struct Command {
    #[clap(long)]
    ifindex_ingress: i32,
    #[clap(long)]
    ifindex_egress: i32,
    /// Verbose debug output
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let opts = Command::parse();

    // Install Ctrl-C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let mut skel_builder = PrioSkelBuilder::default();
    if opts.verbose {
        skel_builder.obj_builder.debug(true);
    }

    // Load into kernel
    let open_skel = skel_builder.open()?;
    let mut skel = open_skel.load()?;

    // Set up and attach ingress TC hook
    let mut ingress = TcHookBuilder::new()
        .fd(skel.progs().ingress().fd())
        .ifindex(opts.ifindex_ingress)
        .replace(true)
        .handle(1)
        .priority(1)
        .hook(TC_INGRESS);
    ingress
        .create()
        .context("Failed to create ingress TC qdisc")?;
    ingress
        .attach()
        .context("Failed to attach ingress TC prog")?;

    // Set up and attach egress TC hook
    let mut egress = TcHookBuilder::new()
        .fd(skel.progs().egress().fd())
        .ifindex(opts.ifindex_ingress)
        .replace(true)
        .handle(1)
        .priority(1)
        .hook(TC_EGRESS);
    egress
        .create()
        .context("Failed to create egress TC qdisc")?;
    egress.attach().context("Failed to attach egress TC prog")?;

    // Block until SIGINT
    while running.load(Ordering::SeqCst) {
        println!("set={}, found={}", skel.bss().nr_set, skel.bss().nr_found);
        sleep(Duration::new(1, 0));
    }

    if let Err(e) = ingress.detach() {
        eprintln!("Failed to detach ingress prog: {}", e);
    }
    if let Err(e) = ingress.destroy() {
        eprintln!("Failed to destroy ingress TC hook: {}", e);
    }
    if let Err(e) = egress.detach() {
        eprintln!("Failed to detach egress prog: {}", e);
    }
    if let Err(e) = egress.destroy() {
        eprintln!("Failed to destroy egress TC hook: {}", e);
    }

    Ok(())
}
