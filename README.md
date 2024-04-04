# sflow_exporter

Serving [sflow](https://sflow.org/)/[RFC3176](https://datatracker.ietf.org/doc/html/rfc3176) traffic metrics as
Prometheus endpoints.

## Exposed Prometheus Metrics

```prometheus
sflow_agent_drops{agent=<label>} <droped frames, which should have been sampled>
sflow_router_bytes{ether_type=<label>,in=<label>,out=<label>} <globaly deduplicated bytes>
```

- sflow_agent_drops - `counter` of samples that were dropped due to missing resources
- sflow_router_bytes - `counter` of bytes that were transfered between mac addresses

## Deployment

- [Nix Flake](flake.nix)
- Docker [`ghcr.io/dd-ix/sflow_exporter`](ghcr.io/dd-ix/sflow_exporter)
- [Binary Releases](https://github.com/MarcelCoding/zia/releases/)

## Configuration

- Pass a meta file using the `-m/--meta` flag or using the `SFLOW_EXPORTER_META` environment variable.
- Configure the mode:
  - the **check** subcommand is used to validate a given meta file.
  - the **listen** subcommand is used to start the sflow and prometheus listener.
    The ports can be configured using `--sflow_addr`/`metrics_addr` and the environment
    variables `SFLOW_EXPORTER_SFlOW_LISTEN_ADDR` and `SFLOW_EXPORTER_METRICS_LISTEN_ADDR`

## Meta Configuration

The meta configuration is used to enrich the data received through sflow. It describes the inventory of your
infrastructure.

sflow_exporter monitors the meta file for changes on the file system. If a change is detected it validates the config
and if that was successful, applies the new configuration.

```yaml
# meta.yaml
routers:
  - { mac: 00:00:00:00:00:01, label: 1234 }
  - { mac: 00:00:00:00:00:02, label: 4321 }
agents:
  - { label: sw01.domain.tld, source: fe44::1 }
  - { label: sw02.domain.tld, source: fe44::2 }
ether_types:
  0x0800: { label: IPv4 }
  0x86DD: { label: IPv6 }
```

### Routers

The router property describes all entities that are sending and recvieving packages.

- the **mac address** is used to identify who send a packet, and who should recvieve it.
- the **label** the the property is the identification thats passed over to prometheus.

### Agents

Agents are used to monitor the amount of droped samples. If this number grows to big, try tuning the sample rate.

- the **source** address is the ip address that send the sflow packet to sflow_exporter.
- the **label** the the property is the identification thats passed over to prometheus.

### Ether Types

The ether types property lists all for you relevant ether types. All not defined ether types are going to be grouped
as `other`.

- the **label** the the property is the identification thats passed over to prometheus.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
