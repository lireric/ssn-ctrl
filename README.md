# ssn-ctrl
## SSN messages main router and processor

This module performing proxing between different interfaces and objects via MQTT protocol.
Main goal of this module - sharing low level messages from/to SSN applications located in microcontrollers and integrate it into the high level logical infrastracture.

Messages from SSN applications connected by serial RS485 interface are processed, checked for SSN packages, routed if needed, published into MQTT broker and execute SSN commands.

Module is written in Rust and replaces previous version on Lua, Python and C.

# Main execution file: ssn-ctrl

### Optional parameters:
	-l 				set logging level  <DEBUG, INFO, WARN, ERROR>
	-c <path_to_config_file>	point to configuration file. Default value ssn_conf.yaml in current directory
	-d 				start in database storing mode

### Example:
	ssn-ctrl -l INFO
	ssn-ctrl -l WARN -c ssn_conf2.yaml -d

### Build
    cargo build --release
    cargo run -- -c ssn_conf.yaml
