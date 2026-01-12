**netwatch** is a Windows network inspection tool that shows active TCP connections,
the owning process, and optional reverse DNS information.

It is part of the **Sentinel** toolkit.



## Features

- List active **TCP ESTABLISHED** connections
- Map connections to **process name & PID**
- Optional **reverse DNS resolution**
- Fast by default (DNS is opt-in)
- Machine-readable **JSON output**
- Persistent DNS cache for faster repeated runs



## Installation

Download the latest `netwatch.exe` from the **Sentinel GitHub Releases** page.

No installation required.



## Usage

### Default (fast, no DNS)

```bash
netwatch
```

Output:
```
PID    Process        Local Address        Remote Address        Domain
----------------------------------------------------------------------------
6460   chrome.exe     80.0.0.3:52765       140.82.113.25:443     -
```



### Resolve DNS (uses cache)

```bash
netwatch --resolve
```

- Loads cached DNS entries
- Resolves only missing IPs
- Updates cache



### Force fresh DNS resolution

```bash
netwatch --resolve-refresh
```

- Ignores cache
- Re-resolves all remote IPs
- Updates cache with fresh results



### JSON output (for scripts & tools)

```bash
netwatch --json
```

Example:
```json
[
  {
    "pid": 6460,
    "process": "chrome.exe",
    "local_addr": "30.0.0.4:535",
    "remote_addr": "120.82.111.25:443",
    "domain": "lb-140-82-113-25-iad.github.com"
  }
]
```

Works with:
- Python
- PowerShell
- Bash
- Other Sentinel tools



## Command-line Options

| Flag | Description |
|---|---|
| `--json` | Output machine-readable JSON |
| `--resolve` | Resolve remote IPs using DNS cache |
| `--resolve-refresh` | Force fresh DNS resolution (ignore cache) |
| `--help` | Show help |



## DNS Cache

- Stored per-user in:
  ```
  %LOCALAPPDATA%\Sentinel\netwatch\dns_cache.json
  ```
- Best-effort cache
- Used only when `--resolve` is specified
- Can be safely deleted at any time



## Notes

- Administrator privileges may be required to see all connections
- Reverse DNS is informational only
- Domain names should not be treated as security proof



## License

MIT License