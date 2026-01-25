# Molten

> **A configurable, high-performance document and workflow management system written in Rust.**

![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![Status](https://img.shields.io/badge/status-Pre--Alpha-red)

## Overview

**Molten** is a modular platform designed to handle complex business processes through dynamic document management and state-machine-driven workflows. 

Unlike traditional hard-coded management systems, Molten allows for the definition of data structures (Forms) and logic (Workflows) via configuration, making it adaptable to a wide range of domains—from quality assurance and compliance to generic business process automation.

Built on the robust Rust ecosystem, Molten prioritizes type safety, memory safety, and high concurrency.

## Project Structure

Molten is organized as a cargo workspace containing several specialized crates:

| Crate | Description |
|-------|-------------|
| **`molten`** | The primary entry point. A facade crate re-exporting core functionality for standard usage. |
| **`molten-core`** | Defines the foundational domain models, traits, and interfaces used across the system. |
| **`molten-document`** | Handles dynamic schema validation, field typing, and document data structures. |
| **`molten-workflow`** | A dedicated state machine engine managing transition logic and phase rules. |
| **`molten-service`** | The orchestration layer connecting storage, documents, and workflows to apply business logic. |
| **`molten-config`** | Parsers for system definitions (YAML/TOML/JSON). |
| **`molten-storage-seaorm`** | Database persistence layer implemented using [SeaORM](https://www.sea-ql.org/SeaORM/). |
| **`molten-api`** | RESTful API endpoints built with [Axum](https://github.com/tokio-rs/axum). |

## Core Concepts

Molten is built around four pillars:
1.  **Applications:** Logical groupings of configurations.
2.  **Forms:** Dynamic schema definitions that map to storage entities.
3.  **Workflows:** Directed graphs defining the lifecycle of a document.
4.  **Fields:** The atomic data units (Text, Number, Date, etc.) that compose a document.

For a deep dive into how these interact, please read [ARCHITECTURE.md](ARCHITECTURE.md).

## Getting Started

### Prerequisites
- Rust (latest stable)
- A database supported by SeaORM (PostgreSQL, MySQL, or SQLite)

### Building from Source

```bash
git clone [https://github.com/LeeSomm/molten-rs.git](https://github.com/LeeSomm/molten-rs.git)
cd molten-rs
cargo build --workspace
```

### Contributing
Molten is strictly open-source. We welcome contributions of all kinds, from documentation improvements to core architectural changes.

Please note that this project is currently in the 0.0.1 (Pre-Alpha) stage. APIs are subject to change.

### License
This project is licensed under the Apache-2.0 License.