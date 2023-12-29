# Log Collection Backend

Add additional log sources from to be sent to a centralized server

## Table of Contents

- [Getting Started](#getting-started)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

### Creating Issues
TODO

## Getting Started
Install Rust on your local machine. Use the following link to get setup quickly:
[rust setup](https://www.rust-lang.org/tools/install)

Clone the client and server repositories to start testing:
- [Client](https://github.com/SecurityLogMiner/log-collection-client)
- [Server](https://github.com/SecurityLogMiner/log-collection-server)

The client will read the configuration file and begin processing and sending 
log data from the given PATH to the server.

When running the client for the first time on a linux system, a directory will 
be created at:
- /var/log/logminer/logs/

If you do not have a system service that you are able to read log data from, you
can create one with a combination of a shell script and cronjob:

script.sh:
```
#!/bin/bash
for ((i = 1; i <= 60; i++)); do
    echo "test $(date)" >> /var/log/logminer/logs/test.log
    sleep 1
done
echo "" > /var/log/logminer/logs/test.log
```

cronjob:
```
* * * * * <path_to_your_script>
0 * * * * echo "" > /var/log/logminer/logs/test.log
```

If you do have a known path of streaming data to read from, supply that path to
the config file and start the server and client service - in that order.

Server:
```
cd <server_repo_dir>
cargo install
cargo run
```
Client:
```
cd <client_repo_dir>
cargo install
cargo run
```

You should see streaming data. This documentation will evolve as the project
progresses.

## License
Apache 2.0

## Acknowledgments
Syn Ack Fin

## Contact
Discord, if you know, you know

[Back to top](#table-of-contents)



