# alyx-core

Alyx Core is the core repository for Alyx. It contains `alyx-compiler`, `alyx-core`, `alyx-ir`, and `alyx-plan`.

For details about Alyx project, see [Alyx Overview](docs/alyx-overview.md).

## Architecture

This architecture is recommended, but not required. All developers may use any development approach they prefer, such as creating tools that directly convert User Code into Alyx IR.

User Code
 ↓
Alyx Widgets
 ↓
Alyx IR
 ↓ Converted by compiler
Alyx Plans (RP / EP)
 ↓
Alyx Executor

Also, runtime owns 

## License

Alyx Core (`alyx-core`) is licensed under either MIT or Apache-2.0, at your option. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

## Copyright

Copyright (c) 2026 naoshinn
