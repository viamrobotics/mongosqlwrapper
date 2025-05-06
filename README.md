# MongoDB SQL Wrapper

A Rust library that converts SQL queries to MongoDB query language (MQL). This library is designed to be used as a static library in other projects that need to convert SQL queries to MQL.

## Features
- SQL to MQL conversion
- Support for basic SQL queries (SELECT, FROM, WHERE, etc.)
- C-compatible API for easy integration with other languages
- Cross-platform support (macOS ARM, macOS Intel, Linux x86_64)

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/viamrobotics/mongosqlwrapper.git
cd mongosqlwrapper
```

2. Build the library:
```bash
cargo build --release
```

The static library will be generated at `target/release/libmongosqlwrapper.a`.

### Using in Other Projects

The library is available as a static library (.a file) for different platforms. You can download the appropriate version from the latest GitHub release.
