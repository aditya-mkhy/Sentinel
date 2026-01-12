use crate::models::Connection;

#[cfg(windows)]
use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, TCP_TABLE_OWNER_PID_ALL,
};

#[cfg(windows)]
pub fn scan_active_connections() -> Vec<Connection> {
    let mut size: u32 = 0;

    unsafe {
        // First call: get required buffer size
        let result = GetExtendedTcpTable(
            None,
            &mut size,
            false, // ✅ FIXED
            2,     // AF_INET (IPv4)
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if result != 0 && size == 0 {
            return vec![];
        }

        let mut buffer = vec![0u8; size as usize];

        let result = GetExtendedTcpTable(
            Some(buffer.as_mut_ptr() as _),
            &mut size,
            false, // ✅ FIXED
            2,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if result != 0 {
            return vec![];
        }

        let table = buffer.as_ptr()
            as *const windows::Win32::NetworkManagement::IpHelper::MIB_TCPTABLE_OWNER_PID;

        let count = (*table).dwNumEntries;

        let rows = std::slice::from_raw_parts(
            (*table).table.as_ptr(),
            count as usize,
        );

        let mut connections = Vec::new();

        for row in rows {
            // 5 = ESTABLISHED
            if row.dwState != 5 {
                continue;
            }

            let local_ip = ipv4_from_u32(row.dwLocalAddr);
            let remote_ip = ipv4_from_u32(row.dwRemoteAddr);

            let local_port = u16::from_be(row.dwLocalPort as u16);
            let remote_port = u16::from_be(row.dwRemotePort as u16);

            connections.push(Connection {
                pid: row.dwOwningPid,
                process: "-".into(),
                local_addr: format!("{}:{}", local_ip, local_port),
                remote_addr: format!("{}:{}", remote_ip, remote_port),
                domain: "-".into(),
            });
        }

        connections
    }
}

#[cfg(windows)]
fn ipv4_from_u32(ip: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        ip & 0xFF,
        (ip >> 8) & 0xFF,
        (ip >> 16) & 0xFF,
        (ip >> 24) & 0xFF
    )
}