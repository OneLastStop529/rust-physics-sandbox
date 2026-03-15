# rust-physics-sandbox

`rust-physics-sandbox` is a learning-focused 2D rigid body physics project in Rust.

The goal is to understand physics simulation architecture end to end: fixed timestep stepping, collision detection, contact generation, and impulse-based resolution. Rendering is a support tool, not the product. Correctness, clarity, and observability take priority over feature count.

## Status

Milestone 3 is complete. The next implementation target is Milestone 4: Collision Detection.

Current contents:

- Macroquad application shell with fixed timestep accumulator
- pause and single-step controls
- debug rendering for lines, circles, AABBs, and points
- HUD with timing and body readouts
- rigid body and physics world scaffolding
- dynamic and static bodies with circle and AABB colliders
- gravity and semi-implicit Euler integration
- project planning docs and milestone breakdown

Current runtime behavior:

- a falling-body demo scene runs in a window
- dynamic bodies accelerate under gravity
- static bodies remain stationary
- the floor is visualized but does not block motion yet
- tests and strict clippy checks are passing

Collision detection and response are not implemented yet, so falling bodies currently pass through the floor. That is expected until Milestones 4 through 6.

## v0.1 Scope

Planned `v0.1` features:

- fixed timestep simulation
- physics world and rigid bodies
- dynamic and static bodies
- gravity and integration
- circle and AABB colliders
- collision detection for:
  - circle-circle
  - AABB-AABB
  - circle-AABB
- contact generation
- impulse-based collision resolution
- restitution and friction
- basic stacking stability
- debug rendering and HUD
- sandbox controls:
  - object spawning
  - reset world
  - pause
  - single-step
- demo scenes:
  - falling objects
  - box stack
  - bouncing balls
  - friction slide

Explicitly out of scope for early milestones:

- 3D physics
- ECS
- joints
- ragdoll
- CCD
- advanced broad phase
- SAT for oriented boxes
- scripting
- editor UI
- networking
- serialization
- GPU-based physics
- advanced renderer

## Project Priorities

- keep the simulation loop easy to reason about
- prefer simple data structures first
- make physics state easy to inspect
- add tests and demo scenes alongside each major feature
- avoid optimization work until profiling shows a need

## Planned Architecture

The intended structure is conventional and small:

```text
src/
  app/
  physics/
  render/
  input/
  math/
tests/
examples/
```

Module responsibilities are documented in [ARCHITECTURE.md](/Users/yizehu/Workspaces/rust-physics-sandbox/ARCHITECTURE.md).

## Repository Docs

- [MILESTONES.md](/Users/yizehu/Workspaces/rust-physics-sandbox/MILESTONES.md): staged delivery plan, acceptance criteria, and roadmap
- [ARCHITECTURE.md](/Users/yizehu/Workspaces/rust-physics-sandbox/ARCHITECTURE.md): internal structure, data model, and simulation pipeline
- [TODO.md](/Users/yizehu/Workspaces/rust-physics-sandbox/TODO.md): implementation checklist organized by milestone
- [AGENTS.md](/Users/yizehu/Workspaces/rust-physics-sandbox/AGENTS.md): repo-specific contributor instructions for Codex

## Development Workflow

Use the standard Cargo workflow:

```bash
cargo build
cargo run
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

## Near-Term Plan

1. implement Milestone 4 narrow-phase collision detection for circle-circle, AABB-AABB, and circle-AABB
2. surface overlap results in deterministic tests before adding response code
3. convert overlaps into contacts in Milestone 5
4. add impulse-based collision resolution in Milestone 6
5. expand sandbox controls, debug tools, and demo scenes after core collision behavior is stable

## Notes

This is a solo engineer project. The plan is intentionally incremental to keep the codebase understandable while still leaving room for future extensions such as better manifolds, additional shapes, or constraints after `v0.1`.
