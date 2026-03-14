# Repository Guidelines

## Project Structure & Module Organization
This repository is currently a minimal Rust sandbox: only [README.md](/Users/yizehu/Workspaces/rust-physics-sandbox/README.md) exists today. As the project grows, keep the crate layout conventional so contributors can navigate it quickly:

- `src/`: application and library code, split by physics domain (`collision.rs`, `integrator.rs`, etc.).
- `tests/`: integration tests covering public behavior.
- `assets/`: sample scenes, data files, or reference media if the sandbox needs them.
- `examples/`: small runnable experiments that demonstrate one concept at a time.

Prefer small modules with clear responsibilities over a single large simulation file.

## Build, Test, and Development Commands
Use standard Cargo commands once `Cargo.toml` is added:

- `cargo build`: compile the project in debug mode.
- `cargo run`: run the default binary locally.
- `cargo test`: run unit and integration tests.
- `cargo fmt`: apply Rust formatting.
- `cargo clippy --all-targets --all-features -D warnings`: catch lint issues before review.

If you add benchmarks, keep them under `benches/` and run them with `cargo bench`.

## Coding Style & Naming Conventions
Follow Rust defaults: 4-space indentation, `snake_case` for functions and modules, `CamelCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Keep public APIs explicit and document any non-obvious math or units inline. Run `cargo fmt` before committing; if project-specific formatter or lint config is added later, follow that instead of ad hoc styling.

## Testing Guidelines
Write unit tests next to the code they validate and integration tests in `tests/`. Name tests for observable behavior, such as `applies_gravity_each_step` or `resolves_elastic_collision`. Focus on deterministic cases and numeric edge conditions; avoid flaky floating-point assertions by using tolerances.

## Commit & Pull Request Guidelines
Git history currently starts with `Initial commit`, so keep the same simple pattern: short, imperative commit subjects under 72 characters. Each pull request should explain the behavior change, note any physics assumptions or constants introduced, and list validation steps such as `cargo test` and `cargo clippy`. Include screenshots or recordings only if you add visualization output.
