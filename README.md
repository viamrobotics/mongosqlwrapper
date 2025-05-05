# MongoDB SQL Wrapper

A Rust library that converts SQL queries to MongoDB query language (MQL). This library is designed to be used as a static library in other projects that need to convert SQL queries to MQL.

## Features

- SQL to MQL conversion
- Support for basic SQL queries (SELECT, FROM, WHERE, etc.)
- C-compatible API for easy integration with other languages
- Cross-platform support (macOS ARM, macOS Intel, Linux x86_64)

## Building

### Prerequisites

- Rust toolchain (stable)
- Cargo

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

#### Download Script

Create a `download_lib.sh` script in your project:

```bash
#!/bin/bash

# Get the latest release version
LATEST_RELEASE=$(curl -s https://api.github.com/repos/viamrobotics/mongosqlwrapper/releases/latest | grep "tag_name" | cut -d '"' -f 4)

# Determine the platform
case "$(uname -s)" in
    Darwin)
        case "$(uname -m)" in
            arm64)
                PLATFORM="aarch64-apple-darwin"
                ;;
            x86_64)
                PLATFORM="x86_64-apple-darwin"
                ;;
            *)
                echo "Unsupported architecture"
                exit 1
                ;;
        esac
        ;;
    Linux)
        PLATFORM="x86_64-unknown-linux-gnu"
        ;;
    *)
        echo "Unsupported platform"
        exit 1
        ;;
esac

# Download the library
curl -L -o libmongosqlwrapper.a \
    "https://github.com/viamrobotics/mongosqlwrapper/releases/download/${LATEST_RELEASE}/libmongosqlwrapper-${PLATFORM}.a"
```

## API Usage

The library provides a C-compatible API for SQL to MQL conversion:

```c
// Function to compile SQL to MQL
MongoSQLWrapperResult* compile_sql_to_mql(const char* sql);

// Function to free the result
void free_result(MongoSQLWrapperResult* result);
```

### Example

```c
#include <stdio.h>

extern MongoSQLWrapperResult* compile_sql_to_mql(const char* sql);
extern void free_result(MongoSQLWrapperResult* result);

int main() {
    const char* sql = "SELECT * FROM users WHERE age > 18";
    MongoSQLWrapperResult* result = compile_sql_to_mql(sql);
    
    if (result->error) {
        printf("Error: %s\n", result->error);
    } else {
        printf("MQL: %s\n", result->mql);
    }
    
    free_result(result);
    return 0;
}
```

## Development

### Running Tests

```bash
cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details. 