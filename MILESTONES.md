# rust-physics-sandbox Milestones

## Project Overview

`rust-physics-sandbox` is a learning-oriented 2D rigid body physics sandbox written in Rust. The project prioritizes physics correctness, architectural clarity, and observability over rendering sophistication or feature breadth.

The first release, `v0.1`, focuses on a minimal but complete 2D simulation stack:

- fixed timestep simulation
- rigid bodies
- gravity and integration
- circle and AABB colliders
- narrow-phase collision detection
- contact generation
- impulse-based collision resolution
- restitution and friction
- simple sandbox controls and debug tooling
- demo scenes for each major behavior

The plan is intentionally incremental. Each milestone should leave the repository runnable, inspectable, and in a state where the next milestone can build on stable behavior instead of partial scaffolding.

Current repository status: Milestone 0 is effectively complete. The crate, module skeleton, and terminal entry point exist; later milestones describe the next implementation work.

## Development Principles

- Build the smallest coherent slice first.
- Prefer simple data structures over generic abstraction.
- Keep simulation code deterministic where practical.
- Make internal state easy to inspect in logs, HUDs, and debug draw.
- Add a demo scene for each major physics feature.
- Separate simulation, rendering, and input concerns early.
- Avoid premature optimization and advanced broad-phase design in `v0.1`.
- Keep module boundaries extensible so later versions can add new collider types, constraints, and scene formats without large rewrites.

## Milestone Dependency Graph

```text
M0 Project Skeleton
  -> M1 App Loop + Debug Rendering
    -> M2 Rigid Body Model
      -> M3 Gravity + Integration
        -> M4 Collision Detection
          -> M5 Contact Generation
            -> M6 Impulse Solver
              -> M7 Restitution + Friction
                -> M8 Sandbox Interaction
                  -> M9 Debugging Tools
                    -> M10 v0.1 Release
```

## Milestone 0: Project Skeleton

### Objective

Create the minimal repository, crate, and module layout needed to support iterative development.

### Scope

- initialize Cargo project
- define module folders
- add linting, formatting, and test defaults
- defer rendering/input library selection until the runtime shell in Milestone 1
- create a basic executable entry point

### Deliverables

- `Cargo.toml`
- `src/` with placeholder modules
- `README.md` updated with build/run instructions
- `cargo fmt`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` wired into normal workflow

### Acceptance Criteria

- project builds cleanly
- project runs and opens a window or terminal app shell
- module structure reflects intended architecture
- formatting and linting pass without warnings

### Common Pitfalls

- overdesigning abstractions before the first simulation step exists
- choosing a heavy framework that obscures the app loop
- mixing physics code into rendering bootstrap code

### Optional Stretch Goals

- GitHub Actions for formatting, linting, and tests
- basic scene configuration constants module

## Milestone 1: App Loop + Debug Rendering

### Objective

Establish the runtime shell: window, frame loop, timing, and basic debug drawing.

### Scope

- application loop
- frame timing
- camera or screen-space convention
- primitive debug rendering for circles, rectangles, and lines
- on-screen frame stats

### Deliverables

- app loop with update/render separation
- debug renderer capable of drawing:
  - circles
  - AABBs
  - lines
  - points
- placeholder HUD text

### Acceptance Criteria

- application runs continuously
- debug primitives can be drawn in stable positions
- frame delta and simulation accumulator are visible or inspectable
- render path is separate from simulation state mutation

### Common Pitfalls

- tying simulation directly to render delta time
- mixing screen coordinates and world coordinates inconsistently
- hiding timing logic inside the renderer

### Optional Stretch Goals

- simple camera pan/zoom
- world grid drawing

## Milestone 2: Rigid Body Model

### Objective

Introduce the core rigid body data model and world ownership model.

### Scope

- physics world container
- rigid body struct
- body handles or indices
- body types:
  - dynamic
  - static
- collider attachment
- material properties placeholders

### Deliverables

- `PhysicsWorld`
- `RigidBody`
- `BodyType`
- body creation and storage APIs
- simple scene initialization with a few bodies

### Acceptance Criteria

- bodies can be created and stored in the world
- static and dynamic bodies are represented differently
- world update can iterate bodies deterministically
- collider and material data live on bodies or in clearly owned structures

### Common Pitfalls

- introducing complex ownership patterns too early
- storing transient solver state permanently without need
- conflating body transform with collider-specific data

### Optional Stretch Goals

- body naming or tags for debug output
- body removal/reset helpers

## Milestone 3: Gravity + Integration

### Objective

Advance bodies through time with a fixed timestep and basic force integration.

### Scope

- fixed timestep simulation
- gravity
- linear velocity
- position integration
- basic angular fields may exist but can remain unused in `v0.1` if rotation is deferred

### Deliverables

- fixed-step `step(dt)` entry point
- gravity application for dynamic bodies
- semi-implicit Euler integration
- simple falling-body scene

### Acceptance Criteria

- dynamic bodies fall under gravity at a stable rate
- static bodies remain stationary
- simulation behavior is independent of render frame rate
- pause and manual stepping are possible at code level

### Common Pitfalls

- variable timestep integration
- applying gravity to static bodies
- integrating position before resolving solver state design

### Optional Stretch Goals

- configurable gravity vector
- per-body gravity scale

## Milestone 4: Collision Detection

### Objective

Detect overlapping collider pairs using simple narrow-phase checks.

### Scope

- naive `O(n^2)` pair generation
- shape support:
  - circle-circle
  - AABB-AABB
  - circle-AABB
- overlap tests separated from response logic

### Deliverables

- pair iteration logic
- narrow-phase collision functions
- collision result type indicating hit/no-hit
- demo scenes for each supported pair type

### Acceptance Criteria

- overlapping pairs are detected correctly
- non-overlapping pairs do not produce false contacts in normal cases
- each supported shape pair has at least one deterministic test
- collision detection is independent of debug rendering

### Common Pitfalls

- embedding impulse resolution into detection code
- unclear conventions for normals and penetration depth
- inconsistent treatment of circle-AABB closest-point logic

### Optional Stretch Goals

- collision statistics counter in HUD
- shape pair dispatch table cleanup

## Milestone 5: Contact Generation

### Objective

Convert overlap results into solver-ready contacts.

### Scope

- contact manifold representation
- contact normal
- penetration depth
- contact point generation
- one-contact simplification where appropriate

### Deliverables

- `Contact` struct
- contact generation from each narrow-phase result
- contact collection per step
- debug draw for contact points and normals

### Acceptance Criteria

- each collision pair produces consistent contact data
- contact normals point in a stable, documented direction
- contact points appear visually correct in debug draw
- penetration depth is reasonable enough for positional correction and impulse solving

### Common Pitfalls

- unstable normal orientation between frames
- generating contacts in world/local coordinates inconsistently
- skipping documentation for contact conventions

### Optional Stretch Goals

- manifold support for multi-point box contacts
- contact IDs for debug tracing

## Milestone 6: Impulse Solver

### Objective

Resolve interpenetration and relative velocity using an impulse-based solver.

### Scope

- sequential impulse solver
- normal impulse resolution
- iterative solver loop
- velocity update on dynamic bodies
- basic positional correction to reduce sinking

### Deliverables

- solver iteration loop
- impulse computation for dynamic-static and dynamic-dynamic pairs
- stack and bounce demo scenes
- tunable iteration count

### Acceptance Criteria

- falling objects collide and stop instead of passing through
- stacked objects are mostly stable under simple scenes
- dynamic-static and dynamic-dynamic collisions both work
- solver iteration count affects stability in expected ways

### Common Pitfalls

- applying impulses to static bodies
- failing to recompute relative velocity correctly
- relying entirely on positional correction instead of velocity impulses

### Optional Stretch Goals

- warm starting placeholders for future work
- split impulse experimentation notes

## Milestone 7: Restitution + Friction

### Objective

Add basic material response to make collisions behave less uniformly.

### Scope

- restitution
- static or dynamic friction approximation
- tangent calculation
- material combination rules

### Deliverables

- restitution in normal impulse solve
- friction tangent impulse
- bounce and friction demo scenes
- tunable material coefficients

### Acceptance Criteria

- high-restitution objects visibly bounce
- low-restitution objects settle quickly
- sliding bodies slow down with friction
- friction behavior is stable enough for simple box stacks and ramps/planes

### Common Pitfalls

- friction impulses exceeding normal impulse bounds
- applying restitution on resting contacts
- unstable tangent direction handling at very low relative velocity

### Optional Stretch Goals

- separate static and dynamic friction
- per-scene material presets

## Milestone 8: Sandbox Interaction

### Objective

Make the sandbox usable for rapid manual testing and iteration.

### Scope

- object spawning
- reset world
- pause
- single-step simulation
- scene switching

### Deliverables

- keyboard/mouse input mapping
- scene reset logic
- object spawn helpers
- demo scene registry:
  - falling objects
  - box stack
  - bouncing balls
  - friction slide

### Acceptance Criteria

- user can reset and replay scenes without restarting
- paused simulation can advance exactly one fixed step
- object spawning works without corrupting world state
- all major features are demonstrated through scenes

### Common Pitfalls

- mutating world state mid-solver
- scene setup code coupled tightly to app bootstrap
- spawn/reset logic bypassing body initialization rules

### Optional Stretch Goals

- spawn mode selector
- random stress test scene

## Milestone 9: Debugging Tools

### Objective

Improve observability so simulation errors can be inspected quickly.

### Scope

- debug HUD
- contact count display
- solver iteration display
- body state inspection output
- toggles for debug draw layers

### Deliverables

- HUD showing:
  - FPS
  - fixed timestep
  - body count
  - contact count
  - active scene
  - pause state
- toggles for collider, contacts, and normals draw
- optional logging hooks for selected bodies

### Acceptance Criteria

- key runtime stats are visible without attaching a debugger
- debug overlays can be enabled and disabled independently
- contact points and normals help explain solver behavior
- common failure modes are inspectable in a single run

### Common Pitfalls

- making debug tools mutate simulation state
- burying useful state in verbose logs only
- adding too many overlays without toggles

### Optional Stretch Goals

- body picking and inspection panel
- frame capture of contact data to file

## Milestone 10: v0.1 Release

### Objective

Stabilize, document, and package the first complete sandbox release.

### Scope

- cleanup
- test pass
- docs pass
- demo validation
- release notes

### Deliverables

- `v0.1` feature-complete implementation
- repository documentation updated
- known limitations documented
- release checklist completed

### Acceptance Criteria

- all scoped features are implemented
- all demo scenes run correctly
- `cargo fmt`, `cargo test`, and `cargo clippy --all-targets --all-features -- -D warnings` pass
- architecture and milestone docs reflect shipped behavior
- out-of-scope features are explicitly deferred

### Common Pitfalls

- expanding scope late with nonessential systems
- shipping without deterministic regression tests
- leaving coordinate system and solver assumptions undocumented

### Optional Stretch Goals

- simple benchmark scene with timing output
- short design notes on lessons learned from `v0.1`

## Release Roadmap

### v0.1

Target scope:

- complete 2D rigid body sandbox
- fixed timestep simulation
- circle and AABB collisions
- contact generation and impulse resolution
- restitution and friction
- debug rendering and HUD
- sandbox controls and demo scenes

### v0.2

Likely follow-up work after `v0.1`:

- improved contact manifold quality
- better stacking stability
- cleaner material system
- scene loading helpers
- expanded automated testing
- optional broad-phase acceleration if profiling justifies it

### v0.3

Longer-range exploration:

- oriented boxes or additional convex shapes
- joints and constraints
- replay/logging tools
- richer inspection tooling
- architecture cleanup based on `v0.1` and `v0.2` lessons

## Non-Goals for v0.1

The following remain explicitly out of scope for early milestones:

- 3D physics
- ECS
- joints
- ragdoll
- CCD
- broad phase optimization beyond naive `O(n^2)`
- SAT for oriented boxes
- scripting
- editor UI
- networking
- serialization
- GPU-based physics
- advanced renderer
