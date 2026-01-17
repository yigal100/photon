# Photon: The Multi-Phase Language

**Photon (`.photon`)** is a systems programming language forged in the spirit of exploration and strategic efficiency. Inspired by the performance of **Zig** and the compile-time safety of **Rust**, Photon is designed for low-level control, high-stakes performance, and unwavering security, featuring a unique **multi-phase compilation model**.

Our mission is to equip engineers with a tool that demands precision, rewards insight, and prevents catastrophic failures before launch.

## 🚀 Key Features

* **🛡️ Strong Static Safety:** Built-in safeguards and a comprehensive type system ensure mission-critical stability, reducing runtime errors to near-zero.
* **⚙️ Low-Level Control:** Get closer to the metal than ever before. Photon offers explicit control over memory layout and system resources, without the overhead of a garbage collector.
* **📐 Multi-Phase Compilation:** Our advanced compilation pipeline is designed to perform complex optimizations and compile-time execution across multiple stages, delivering peak performance and flexibility.
* **⚡ Modern Simplicity:** Achieve power without complexity. Photon's syntax is designed to be clear, concise, and immediately readable.
* **🔧 Metaprogramming:** Powerful compile-time code generation and evaluation with WASM-sandboxed execution for safety.

## 📚 Documentation (TBD)

All project documentation is organized under **[docs/](./docs/)** with clear separation by audience:

### 🎯 [Language Documentation](./docs/language/)
**For Photon users and language learners**
- Language design principles and philosophy
- Complete syntax specification and grammar
- Programming patterns and best practices
- Comprehensive examples and tutorials
- Code organization guidelines

### 🔧 [Contributing Guide](./docs/contributing/)
**For compiler developers and contributors**
- Compiler architecture and implementation
- Development guidelines and workflows
- Testing strategies and specifications
- API design and phase interfaces
- Performance considerations

## 🌌 The Exploration Mandate

Photon is not just a language; it's a vehicle for discovery. We invite engineers to push the boundaries of performance and reliability. Whether you're building embedded systems for deep space probes or high-frequency trading platforms, Photon is your secure, high-warp engine.

## 🛠️ Getting Started

### For Language Users
1. Start with **[Language Documentation](./docs/language/)** to understand Photon's design and syntax
2. Explore **[Language Examples](./docs/language/Language%20Examples.md)** to see Photon in action
3. Learn **[Code Organization](./docs/language/Code%20Organization%20Design.md)** patterns for structuring projects

### For Contributors
1. Read the **[Contributing Guide](./docs/contributing/)** to understand the compiler architecture
2. Review **[Code Organization](./docs/contributing/Code%20Organization.md)** for implementation patterns
3. Study the **[Compilation Pipeline](./docs/contributing/Compilation%20Pipeline.md)** design

## 🏗️ Project Status

🚧 **Under Active Development** - Photon is currently in the design and early implementation phase.

**Current Focus:**
- Finalizing language design and specifications
- Building core compiler infrastructure
- Implementing the five-phase compilation pipeline

**Architecture Highlights:**
- **Multi-phase Programming Model** with first class Meta-Object Protocol (MOP) support and compile-time introspection 
- **Capability-based security** for controlled system access
- **WASM-first design** enabling modular, distributed compilation

## 🤝 Contributing

We welcome contributions from developers passionate about systems programming and language design!

**Ways to Contribute:**
- **Language Design**: Help refine syntax, semantics, and features
- **Compiler Implementation**: Build the five-phase compilation pipeline
- **Documentation**: Improve specifications, examples, and guides
- **Testing**: Develop comprehensive test suites and benchmarks

See the **[Contributing Guide](./docs/contributing/)** for detailed information on getting involved.

## 📖 Core Principles

Photon is built on these fundamental principles:

- **Explicit Over Implicit** - All behavior should be clear and unambiguous
- **Local Reasoning** - Code should be understandable without distant context
- **One Way to Do It** - Prefer canonical approaches to reduce cognitive load
- **Fail Fast** - Detect and report errors as early as possible
- **Security Through Capability** - Grant minimum necessary privileges

## 💬 Join the Fleet

We welcome all who are dedicated to the pursuit of excellence and the mastery of systems. 

- **Issues**: Report bugs, request features, ask questions
- **Discussions**: Share ideas and engage with the community
- **Pull Requests**: Contribute code, documentation, and improvements

*Live Long and Compile.*

---

## 📁 Repository Structure

```
Photon/
├── docs/                   # All project documentation
│   ├── language/          # Language design and user guide
│   │   ├── Design Principles.md
│   │   ├── Grammar Specification.md
│   │   ├── Language Examples.md
│   │   └── Code Organization Design.md
│   └── contributing/      # Compiler implementation guide
│       ├── Compilation Pipeline.md
│       ├── AST Specification.md
│       ├── Testing Strategy.md
│       └── Code Organization.md
├── src/                    # The All-In-One beam CLI tool
│   ├── main.rs             # CLI entry point
│   ├── commands            # The sub-commands for beam
│   │   ├── run.rs
│   │   ├── build.rs
│   │   └── ...
│   └── ...
├── compiler/               # The Phaser compiler implementation
├── tests/                  # Test suites
└── examples/               # Example Photon programs
```