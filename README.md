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

Getting data into your systems shouldn't require building an ingestion platform first.
Most teams face the same painful choice: accept dirty data and fix it downstream,
pay enterprise prices for managed solutions, or spend months building custom validation layers.

Bastion takes a different approach. It sits at the point of data entry — a lightweight,
high-performance gateway that validates, transforms, and routes your data before it
touches your infrastructure. Whether that infrastructure already exists or you're
building it from scratch.

### The Problem

Data pipelines break silently. A mobile app sends a malformed payload at 3 AM, a partner
API changes its schema without notice, a developer pushes a typo in a field name. By the
time you notice, you have millions of bad records in your lake and a weekend of cleanup ahead.

Traditional solutions address this after the fact. Bastion addresses it at the gate.

### No Infrastructure? No Problem.

Bastion ships with S3/GCS output as a first-class citizen. No Spark jobs, no Glue pipelines,
no scheduled transformations. Bastion writes Parquet natively at ingestion time — the conversion
happens at the gate, not in a separate processing layer. Send your events in, get clean
Parquet files out — ready for BigQuery, Athena, Snowflake, or dbt without configuring a
single Kafka cluster.

When you're ready to add Kafka, a message queue, or any other destination, Bastion routes
to all of them natively. Your stack grows with you; your ingestion layer doesn't need to change.

```
[Apps]     -->  +----------+  -->  S3 / GCS (Parquet)
[Sensors]  -->  | Bastion  |  -->  Kafka / Redpanda
[Webhooks] -->  +----------+  -->  BigQuery / Webhook
     validate -> transform -> route
```

### How Bastion Compares

Bastion is not a replacement for Kafka or any streaming platform. It's the layer that
sits **in front** of your infrastructure — or replaces the need to have it on day one.

|                           | Bastion              | Confluent REST Proxy   | Kafka Connect       | Custom Solution |
| ------------------------- | -------------------- | ---------------------- | ------------------- | --------------- |
| Memory footprint          | ~20 MB               | 512 MB+ (JVM)          | 512 MB+ (JVM)       | Varies          |
| Schema validation         | Built-in             | Schema Registry (sep.) | Limited             | Manual          |
| Data transformation       | Bronze→Silver→Gold   | None                   | SMTs (limited)      | Manual          |
| Multi-destination fan-out | Native               | Single cluster         | Single cluster      | Manual          |
| S3/GCS output             | Native               | ✗                      | Via connector       | Manual          |
| Parquet output            | Native, at ingestion | ✗                      | Requires Spark/Glue | Manual          |
| Requires Kafka            | No                   | Yes                    | Yes                 | Depends         |
| Edge deployable           | Yes                  | No                     | No                  | Depends         |
| Deployment                | Single binary        | JVM + Schema Registry  | JVM + Kafka cluster | Varies          |

### Built for the Edge

Bastion compiles to a single binary under 20 MB. No JVM, no runtime dependencies, no
garbage collection pauses. It runs anywhere — from a cloud VM to a Raspberry Pi on a
factory floor.

In IoT or distributed environments, Bastion acts as a local hub: devices publish events
over HTTP, the gateway validates and buffers them locally, and forwards clean data to your
central infrastructure when connectivity allows. If the network goes down, Bastion buffers
locally. When it recovers, it catches up.

### Native Fan-Out

Because Bastion sits at the point of entry, it can publish the same event to multiple
destinations in a single pass — no replication tools, no offset reconciliation, no 2 AM
pages because MirrorMaker fell behind.

```
                            +--> S3 (Parquet archive)
[Data source] --> [Bastion] +--> Kafka (us-east)
                            +--> Kafka (eu-west)
                            +--> Webhook (alerts)
```

### When Bastion Might Not Be the Right Fit

Bastion is designed for HTTP-based data ingestion with validation and routing. It is not
a general-purpose streaming platform, a database CDC tool, or a replacement for Kafka itself.

If you need to replicate data from existing databases, Kafka Connect with Debezium is the
right tool. If you already have clean, well-structured data flowing through a mature
pipeline, you may not need an ingestion gateway at all.

Bastion shines when data enters your system from external or untrusted sources — mobile
apps, IoT devices, partner integrations, user-generated events — and you need confidence
that what lands in your lake is valid, clean, and where it belongs. Whether you have
Kafka or not.

---

> 🚧 Project under active development

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
