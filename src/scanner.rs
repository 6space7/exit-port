use std::net::IpAddr;

use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState};
use sysinfo::{ProcessesToUpdate, System};

use crate::{
    filter::plan_dev_ports,
    model::{ListenerPort, PortScope, Result},
    process_control::process_details,
};

pub fn scan_dev_ports() -> Result<Vec<ListenerPort>> {
    let listeners = scan_listening_tcp_ports()?;
    Ok(plan_dev_ports(listeners))
}

pub fn scan_listening_tcp_ports() -> Result<Vec<ListenerPort>> {
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::All, true);

    let sockets = get_sockets_info(AddressFamilyFlags::all(), ProtocolFlags::TCP)?;
    let mut listeners = Vec::new();

    for socket in sockets {
        let ProtocolSocketInfo::Tcp(tcp) = socket.protocol_socket_info else {
            continue;
        };

        if tcp.state != TcpState::Listen {
            continue;
        }

        let scope = scope_for_addr(tcp.local_addr);
        if socket.associated_pids.is_empty() {
            listeners.push(ListenerPort {
                port: tcp.local_port,
                pid: 0,
                process_name: "unknown".to_string(),
                scope,
                command: None,
            });
            continue;
        }

        for pid in socket.associated_pids {
            let (process_name, command) = process_details(&system, pid);
            listeners.push(ListenerPort {
                port: tcp.local_port,
                pid,
                process_name,
                scope,
                command,
            });
        }
    }

    Ok(listeners)
}

fn scope_for_addr(addr: IpAddr) -> PortScope {
    if addr.is_loopback() {
        PortScope::Loopback
    } else if addr.is_unspecified() {
        PortScope::AllInterfaces
    } else {
        PortScope::Public
    }
}
