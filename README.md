# Cluster
Cluster is message based communication decentralized console application. It is meant to be fast and easy to use, with no need of a graphical interface.

## Commands
Here is the lists of the available commands.
Brackets `[]` are used to represent optional values where angle brackets `<>` are used to repressent mandatory values.
```
help
- Shows the available commands
- Arguments:
	- [command_name]

host
- Host a server
- Arguments:
	- -p <port>
	- -pw <password>
	- -u <username>

join
- Join a server
- Arguments:
	- <address>
	- -pw <password>
	- -u <username>

leave
- Leave the current server

qui
- Quit the application
```

When executing Cluster, you can pass a command wich will be executed right after Cluster has opened.
```
cluster host -p 1234 -pw password
```

Otherwise, commands within Cluster are executed this way
```
 > /host -p 1234 -pw password
```

## Libraries used
* [terminal](https://github.com/Slazaa/Rust-Terminal)
* [serde_json](https://github.com/serde-rs/json)
* [getch](https://crates.io/crates/getch)
