<p align="center">
  <img src="assets/bastion_banner.png" alt="Bastion Logo" width="500"/>
</p>

<h1 align="center">Bastion, a smart data gateway</h1>

<p align="center">
  <em>The First Line of Defense for Your Data Lake.</em><br/>
  Built in Rust 🦀 • Scriptable in Python 🐍
</p>

---

## Why Bastion?

Getting data into your data lake shouldn't require a PhD in distributed systems. Most teams face the same painful choice: accept dirty data and fix it downstream, pay enterprise prices for managed solutions, or spend months building custom validation layers.

Bastion takes a different approach. It sits at the point of data entry — a lightweight, high-performance gateway that validates, transforms, and routes your data before it touches your infrastructure.

### The Problem

Data pipelines break silently. A sensor sends a malformed payload at 3 AM, a partner API changes its schema without notice, a developer pushes a typo in a field name. By the time you notice, you have millions of bad records in your lake and a weekend of cleanup ahead.

Traditional solutions address this after the fact. Bastion addresses it at the gate.

### How Bastion Compares

Bastion is not a replacement for Kafka, Kafka Connect, or any streaming platform. It's the layer that sits **in front** of them, ensuring your data is clean, validated, and properly routed before it enters your pipeline.

|                           | Bastion                | Confluent REST Proxy       | Kafka Connect         | Custom Solution       |
| ------------------------- | ---------------------- | -------------------------- | --------------------- | --------------------- |
| Memory footprint          | ~20 MB                 | 512 MB+ (JVM)              | 512 MB+ (JVM)         | Varies                |
| Schema validation         | Built-in, real-time    | Schema Registry (separate) | Limited               | Manual implementation |
| Data transformation       | Bronze → Silver → Gold | None                       | SMTs (limited)        | Manual implementation |
| Multi-destination fan-out | Native                 | Single cluster             | Single cluster        | Manual implementation |
| Edge deployable           | Yes                    | No                         | No                    | Depends               |
| Protocol                  | HTTP/REST              | HTTP/REST                  | Pull-based connectors | Varies                |
| Deployment                | Single binary          | JVM + Schema Registry      | JVM + Kafka cluster   | Varies                |

### Built for the Edge

Bastion compiles to a single binary under 20 MB. No JVM, no runtime dependencies, no garbage collection pauses. This means it runs anywhere — from a cloud VM to a Raspberry Pi on a factory floor.

In IoT environments, Bastion acts as a local hub: sensors publish events over HTTP, the gateway validates and buffers them locally, and forwards clean data to your central infrastructure when connectivity allows.

```
[Sensors] → [Bastion on edge] → [Central pipeline]
```

If the network goes down, Bastion buffers locally. When it recovers, it catches up. No data loss, no manual intervention.

### Native Fan-Out (or: How to Never Configure MirrorMaker Again)

Multi-region Kafka deployments typically require replicating data between clusters using tools like MirrorMaker 2 — a process that introduces complexity, latency, and a non-trivial amount of operational pain.

Bastion sidesteps this entirely. Because it sits at the point of entry, it can publish the same event to multiple destinations in a single pass:

```
                          ┌→ Kafka (us-east)
[Data source] → [Bastion] ├→ Kafka (eu-west)
                          ├→ S3 (archive)
                          └→ Webhook (analytics)
```

No cross-cluster replication. No offset reconciliation. No 2 AM pages because MirrorMaker fell behind. The data goes where it needs to go from the start.

### When Bastion Might Not Be the Right Fit

Bastion is designed for HTTP-based data ingestion with validation and routing. It is not a general-purpose streaming platform, a database CDC tool, or a replacement for Kafka itself.

If you need to replicate data from existing databases, Kafka Connect with Debezium is a great choice. If you already have clean, well-structured data flowing through a single Kafka cluster, you may not need an ingestion gateway at all.

Bastion shines when data enters your system from external or untrusted sources — APIs, IoT devices, partner integrations, user-generated events — and you need confidence that what lands in your lake is valid, clean, and where it belongs.

---

> 🚧 Project under active development

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
