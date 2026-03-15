# rust-physics-sandbox TODO

This list tracks implementation work only. Documentation, cleanup, and release work are listed where they directly support a milestone.

## Milestone 0: Project Skeleton

Completed:

- create `Cargo.toml`
- create `src/main.rs`
- add `src/app`, `src/physics`, `src/render`, `src/input`, and `src/math`
- add placeholder `mod.rs` files
- wire `cargo fmt` and `cargo clippy` into local workflow
- update `README.md` with build and run commands
- add `.gitignore`

Deferred to Milestone 1:

- choose a lightweight rendering/input crate
- add a minimal smoke test target once the runtime shell has observable behavior

## Milestone 1: App Loop + Debug Rendering

Completed:

- create window and application bootstrap
- implement frame loop
- add frame timing capture
- add fixed timestep accumulator fields in app state
- implement debug draw API for lines
- implement debug draw API for circles
- implement debug draw API for AABBs
- add simple HUD text rendering
- render a static test scene with debug primitives
- display FPS and frame time

## Milestone 2: Rigid Body Model

Completed:

- define `BodyType` enum with `Dynamic` and `Static`
- define `RigidBody` struct
- define `PhysicsWorld` struct
- add body ID or handle type
- implement body creation API
- implement body storage as `Vec<RigidBody>`
- define `Collider` enum
- add `Circle` collider variant
- add `Aabb` collider variant
- add material fields for restitution and friction
- add scene setup helper to spawn static floor and dynamic objects

## Milestone 3: Gravity + Integration

Completed:

- define fixed simulation `step(dt)` entry point
- add gravity vector to world config
- implement gravity application for dynamic bodies
- implement velocity integration
- implement position integration
- ensure static bodies do not move
- add falling circle demo scene
- add test for gravity affecting only dynamic bodies

Deferred to Milestone 4+:

- add additional demo scenes once collision behavior exists
- add an explicit fixed-step consistency test at the world/app boundary if needed

## Milestone 4: Collision Detection

- implement naive pair generation over all bodies
- skip static-static pairs
- implement circle-circle overlap check
- implement AABB-AABB overlap check
- implement circle-AABB overlap check
- define narrow-phase collision result type
- add unit tests for all overlap and separation cases
- add scene for circle-circle collision
- add scene for AABB-AABB collision
- add scene for circle-AABB collision
- surface pair and collision counts in debug HUD

## Milestone 5: Contact Generation

- define `Contact` struct
- document contact normal convention
- generate contact for circle-circle collisions
- generate contact for AABB-AABB collisions
- generate contact for circle-AABB collisions
- store world-space contact point
- store penetration depth
- store mixed restitution and friction values per contact
- add debug rendering for contact points
- add debug rendering for contact normals
- add unit tests for expected contact normals and depths

## Milestone 6: Impulse Solver

- add solver iteration count to world config
- implement relative velocity calculation
- implement normal impulse solve
- apply impulses only to dynamic bodies
- implement positional correction
- integrate positions after solver completes
- add dynamic-static collision scene
- add dynamic-dynamic collision scene
- add box stack scene
- add regression test for bodies not tunneling through static floor at moderate speeds
- add regression test for stable resting contact in simple stack

## Milestone 7: Restitution + Friction

- implement restitution threshold or resting-contact guard
- apply restitution in normal impulse solve
- compute tangent vector from relative velocity
- implement friction impulse solve
- clamp friction impulse against normal impulse
- define simple material mixing rule
- add bouncing balls demo scene
- add friction slide demo scene
- add test for bounce height being nonzero with restitution
- add test for horizontal velocity decreasing under friction

## Milestone 8: Sandbox Interaction

- define input action mapping
- add pause toggle
- add single-step action
- add reset-world action
- add scene switching action
- add object spawn action for circles
- add object spawn action for AABBs
- ensure spawns are applied outside active solver loop
- add current scene label to HUD
- add pause state to HUD

## Milestone 9: Debugging Tools

- add toggle for collider debug draw
- add toggle for contact debug draw
- add toggle for normal debug draw
- add body count to HUD
- add contact count to HUD
- add solver iteration count to HUD
- add selected body or nearest body debug print helper
- add optional world bounds/grid overlay
- add logging helper for collision events
- add simple frame diagnostics output for difficult scenes

## Milestone 10: v0.1 Release

- review all milestones for missing acceptance checks
- verify all required demo scenes exist
- run `cargo fmt`
- run `cargo test`
- run `cargo clippy --all-targets --all-features -- -D warnings`
- document known limitations
- update `README.md` feature list
- update `ARCHITECTURE.md` if implementation drifted
- update `MILESTONES.md` with completed state if needed
- prepare release notes for `v0.1`

## Debugging Checklist

- verify world coordinates and render coordinates match expected axes
- verify fixed timestep remains constant during runtime
- verify gravity is applied once per simulation step
- verify static bodies always keep zero velocity and zero movement
- verify contact normals point from body A to body B consistently
- verify penetration depth is positive for overlapping shapes
- verify impulses are not applied twice per contact per iteration by mistake
- verify friction is skipped safely when tangent length is near zero
- verify positional correction does not introduce visible jitter in resting scenes
- verify pause and single-step do not consume extra accumulated time
- verify debug draw reflects current-step contacts, not stale data

## Testing Checklist

- unit test vector and geometry helpers
- unit test circle-circle overlap cases
- unit test AABB-AABB overlap cases
- unit test circle-AABB closest-point cases
- unit test contact generation normals and penetration depths
- unit test solver behavior for dynamic-static collision
- unit test solver behavior for dynamic-dynamic collision
- integration test falling object resting on floor
- integration test simple box stack stability over several steps
- integration test bouncing ball with restitution
- integration test sliding body slowing under friction
- smoke test scene reset and single-step behavior

## Future Ideas

- oriented boxes after axis-aligned boxes are stable
- persistent manifolds and warm starting
- broad-phase acceleration after profiling
- joint constraints
- simple scene file loading
- record/replay of simulation inputs
- deterministic replay validation
- body sleep/awake support
- profiling overlay for collision and solver time
- benchmark scenes under `benches/`
