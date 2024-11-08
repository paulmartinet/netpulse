# NetPulse

NetPulse is a network diagnostic tool designed to perform DNS resolution, ping tests, and traceroute operations. It is built using Rust and is suitable for both Windows and Linux environments.

## Features

- **DNS Resolution**: Resolve domain names using a specified DNS server.
- **Ping**: Test the reachability of a host and measure round-trip time.
- **Traceroute**: Trace the path packets take to reach a network host.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.
2. Clone the repository:

   ```bash

      git clone https://github.com/paulmartinet/netpulse.git
      cd netpulse

2.  Build the project: 

     ```bash
   
      cargo build --release

## Usage

Run the command with the desired options: 
   
      ./target/release/netpulse <domain> [--dns <dns_server>] [--ping] [--traceroute]

Example

      ./target/release/netpulse google.com --dns 8.8.8.8 --ping --traceroute
