use crate::timer::{Stoppable, Timer};
use crate::types::ContainerMetadata;
use crate::util;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;

use bus::BusReader;
use shiplift::rep::Container;
use shiplift::Docker;
use tokio::runtime::Runtime;

/// Thread function that updates the container list each second by default
pub fn run(tx: Sender<Vec<ContainerMetadata>>, term_rx: BusReader<()>, interval: u64) -> () {
    let docker = Docker::new();
    let (timer, stop_handle) = Timer::new(Duration::from_millis(interval));
    let has_stopped = Arc::new(AtomicBool::new(false));

    // Handle SIGINT/SIGTERMs by stopping the timer
    let mut term_rx = term_rx;
    let has_stopped_c = Arc::clone(&has_stopped);
    std::thread::spawn(move || {
        term_rx.recv().unwrap();
        println!("Stopping polling");
        stop_handle.stop();
        has_stopped_c.store(true, Ordering::SeqCst);
    });

    for _ in timer {
        let containers: Vec<Container> = match get_containers(&docker) {
            Ok(containers) => containers,
            Err(_) => {
                eprintln!("Error: could not poll the docker daemon to get a list of containers");
                Vec::with_capacity(0)
            }
        };
        // Reduce container vector to list of ids
        let to_collect: Vec<ContainerMetadata> = containers
            .into_iter()
            .filter_map(|c| match should_collect_stats(&c) {
                false => None,
                true => Some(ContainerMetadata {
                    info: format_info(&c),
                    id: c.id,
                }),
            })
            .collect::<Vec<_>>();
        // Make sure the collection hasn't been stopped
        if !has_stopped.load(Ordering::SeqCst) {
            // If sending fails, then stop the collection thread
            if let Err(err) = tx.send(to_collect) {
                eprintln!(
                    "Error: could not send polled docker data to collector thread: {}",
                    err
                );
                break;
            }
        }
    }
}

/// Determines whether the monitoring process can connect to the dockerd socket
pub fn can_connect() -> bool {
    let docker = Docker::new();
    let future = docker.ping();
    // Block on the future and match on the result
    match Runtime::new().unwrap().block_on(future) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Gets all containers running locally on the docker daemon via shiplift
fn get_containers(docker: &Docker) -> Result<Vec<Container>, shiplift::errors::Error> {
    let future = docker.containers().list(&Default::default());
    // Block on the future and return the result
    Runtime::new().unwrap().block_on(future)
}

/// Whether radvisor should collect statistics for the given container
fn should_collect_stats(_c: &Container) -> bool {
    true
}

/// Formats container info used for the header row
fn format_info(c: &Container) -> String {
    format!(
        // Use debug formatting because this function is invoked relatively infrequently
        "# ID: {}\n# Names: {:?}\n# Command: {}\n# Image: {}\n# Status: {}\n# Labels: {:?}\n# Ports: {:?}\n# Created: {}\n# Size: {:?}\n# Root FS Size: {:?}\n# Poll time: {}\n",
        c.id, c.names, c.command, c.image, c.status, c.labels, c.ports, c.created, c.size_rw, c.size_root_fs, util::nano_ts()
    )
}
