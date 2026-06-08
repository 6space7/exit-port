use std::collections::HashSet;

use crate::model::ListenerPort;

pub fn plan_dev_ports(ports: Vec<ListenerPort>) -> Vec<ListenerPort> {
    let mut seen = HashSet::new();
    let mut planned = ports
        .into_iter()
        .filter(is_likely_dev_port)
        .filter(|port| seen.insert((port.port, port.pid, port.process_name.to_ascii_lowercase())))
        .collect::<Vec<_>>();

    planned.sort_by(|left, right| {
        left.port
            .cmp(&right.port)
            .then_with(|| left.process_name.cmp(&right.process_name))
            .then_with(|| left.pid.cmp(&right.pid))
    });

    planned
}

fn is_likely_dev_port(port: &ListenerPort) -> bool {
    if port.port < 1024 {
        return false;
    }

    let process = port.process_name.to_ascii_lowercase();
    if is_infra_process(&process) || is_infra_port(port.port) {
        return false;
    }

    if is_common_dev_process(&process) {
        return true;
    }

    if let Some(command) = &port.command {
        if command_looks_dev(command) {
            return true;
        }
    }

    process == "unknown" && is_common_dev_port(port.port)
}

fn is_infra_port(port: u16) -> bool {
    matches!(
        port,
        1433 | 1521
            | 2049
            | 2181
            | 2375
            | 2376
            | 3306
            | 4369
            | 50070
            | 5432
            | 5672
            | 5900
            | 5984
            | 6379
            | 7000
            | 7001
            | 7199
            | 9042
            | 9160
            | 11211
            | 15672
            | 27017
            | 27018
            | 27019
    )
}

fn is_infra_process(process: &str) -> bool {
    matches!(
        process,
        "postgres"
            | "postgresql"
            | "mysqld"
            | "mariadbd"
            | "redis-server"
            | "mongod"
            | "memcached"
            | "rabbitmq-server"
            | "couchdb"
            | "dockerd"
            | "containerd"
            | "sshd"
    )
}

fn is_common_dev_port(port: u16) -> bool {
    matches!(
        port,
        1024..=9999 | 19000..=19006 | 24678 | 30000..=30010 | 49152..=65535
    )
}

fn is_common_dev_process(process: &str) -> bool {
    matches!(
        process,
        "air"
            | "astro"
            | "bun"
            | "cargo"
            | "deno"
            | "django"
            | "dotnet"
            | "flask"
            | "go"
            | "gunicorn"
            | "http-server"
            | "java"
            | "node"
            | "nodejs"
            | "npm"
            | "nuxt"
            | "parcel"
            | "php"
            | "pnpm"
            | "puma"
            | "python"
            | "python3"
            | "rails"
            | "ruby"
            | "serve"
            | "tsx"
            | "turbo"
            | "uvicorn"
            | "vite"
            | "webpack"
            | "yarn"
    )
}

fn command_looks_dev(command: &str) -> bool {
    let command = command.to_ascii_lowercase();
    [
        " astro ",
        " bun ",
        " cargo ",
        " deno ",
        " django",
        " flask",
        " http.server",
        " next ",
        " npm ",
        " nuxt ",
        " parcel ",
        " pnpm ",
        " rails ",
        " serve ",
        " tsx ",
        " turbo ",
        " uvicorn",
        " vite",
        " webpack",
        " yarn ",
    ]
    .iter()
    .any(|needle| command.contains(needle))
}
