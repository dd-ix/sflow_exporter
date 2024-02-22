# sflow_exporter

Serving [sflow](https://sflow.org/)/[RFC3176](https://datatracker.ietf.org/doc/html/rfc3176) traffic metrics as
Prometheus endpoints.

## Exposed Prometheus Metrics

```prometheus
sflow_agent_bytes{agent=<label>,ether_type=<label>} <bytes>
sflow_agent_drops{agent=<label>} <droped frames, which should have been sampled>
sflow_router_bytes{agent=<label>,ether_type=<label>,in=<label>,out=<label>} <globaly deduplicated bytes>
```

- sflow_agent_bytes - amount of bytes, a given agent processed
- sflow_agent_drops - amount of samples that were dropped due to missing resources
- sflow_router_bytes - amount of bytes that were routet global form in to out, data is deduplicated amount all agents (
  e.g. a packet take a path over multiple agents)

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
  - { mac: 00:00:00:00:00:01, agent: sw01, interface: 1, label: 1234 }
  - { mac: 00:00:00:00:00:02, agent: sw02, interface: 2, label: 4321 }
agents:
  sw01:
    label: sw01.domain.tld
    source: fe44::1
  sw02:
    label: sw02.domain.tld
    source: fe44::2
ether_types:
  0x0800:
    label: IPv4
  0x86DD:
    label: IPv6
```

### Routers

The router property describes all entities that are sending and recvieving packages.

- the **mac address** is used to identify who send a packet, and who should recvieve it.
- the **agent** and **interface** is used to deduplicate global traffic accounting. E.g. a packet is taking a path over
  multiple switches from source to destination router, therefore the bytes should only be counted once.
- the **label** the the property is the identification thats passed over to prometheus.

### Agents

Agents are used to monitor the traffic on a per agent basis and needed to apply global accounting deduplication as
described in the "Router" section.

- the **source** address is the ip address that send the sflow packet to sflow_exporter. It is used to identify the
  current location of the packet.
- the **label** the the property is the identification thats passed over to prometheus.

### Ether Types

The ether types property lists all for you relevant ether types. All not defined ether types are going to be grouped
as `other`.

- the **label** the the property is the identification thats passed over to prometheus.
