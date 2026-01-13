# Sentinel

**Sentinel** is a collection of host-level monitoring and security tools designed
to give users deep visibility into what is happening on their system.

The goal of Sentinel is simple:

> **Know what your system is doing — even when you’re not looking.**

This repository will evolve over time as new tools are added.


## Vision

Modern operating systems hide a lot of activity behind abstractions.
Sentinel aims to remove that opacity by providing tools that help you:

- Inspect active system behavior
- Detect unexpected or unauthorized changes
- Understand network and file system activity at a low level

Sentinel is focused on **local system awareness**, not cloud monitoring.



## Planned Tooling (Early Stage)

> These tools are under active development and may change.

### Network Monitoring
- View established network connections
- Resolve remote IPs to domain names
- Identify which application/process owns each connection

### File Integrity Monitoring
- Scan the entire file system
- Generate cryptographic hashes for files
- Store hashes in a local database
- Detect:
  - File modifications
  - Newly created files
  - Deleted files
 


## Tech Stack

The implementation language and architecture will be chosen per tool
based on performance, reliability, and access to low-level system APIs.

Possible technologies:
- Python (rapid development, cross-platform)
- Rust (safety + performance)
- SQLite (local persistence)



## Disclaimer

Sentinel is intended for **educational, defensive, and system-monitoring purposes** only.

You are responsible for how you use these tools.
Do not deploy Sentinel on systems you do not own or have permission to monitor.



## Status

 **Early development / planning phase**

Expect breaking changes, refactors, and experimentation.



## License

MIT License
