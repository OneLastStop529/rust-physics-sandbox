# rust-physics-sandbox Architecture

## System Overview

The project is a small 2D application with a physics simulation core, thin rendering layer, and minimal input-driven sandbox controls.

The intended design separates the code into these responsibilities:

- `app`: owns application lifecycle, scene setup, state transitions, and the outer frame loop
- `physics`: owns world state, collision detection, contact generation, integration, and solver logic
- `render`: owns debug visualization and HUD rendering
- `input`: translates user input into sandbox commands
- `math`: owns small 2D math helpers and geometry primitives

The simulation should not depend on rendering. Rendering reads world state and debug state after simulation steps complete.

## Current Implementation Snapshot

As of Milestone 5, the repository currently includes:

- a Macroquad app shell with fixed timestep accumulation and pause/single-step controls
- `PhysicsWorld` body storage plus gravity and semi-implicit Euler integration
- `Circle` and axis-aligned `Aabb` colliders attached directly to bodies
- narrow-phase collision detection for circle-circle, AABB-AABB, and circle-AABB
- step-local `Contact` generation with world-space points, normals, penetration, restitution, and friction
- contact point and contact normal debug visualization plus per-step collision/contact counts in the HUD

What is intentionally not implemented yet:

- collision response or impulse solving
- restitution and friction behavior

## Simulation Loop

The application runs a real-time frame loop, but the physics simulation advances on a fixed timestep.

High-level behavior:

1. poll input
2. update app state
3. accumulate real frame time
4. run zero or more fixed simulation steps
5. render current world state and debug overlays

This keeps simulation behavior stable across different render frame rates and makes debugging easier.

## Fixed Timestep Design

Use a fixed simulation delta such as `1.0 / 60.0`.

Key rules:

- render `delta_time` must not drive physics directly
- frame time is accumulated into an accumulator
- while accumulator exceeds fixed `dt`, run one simulation step
- clamp very large frame times to avoid spiral-of-death behavior
- pause mode stops automatic stepping but still allows manual single-step execution

Suggested app-side fields:

- `fixed_dt: f32`
- `accumulator: f32`
- `max_frame_time: f32`
- `paused: bool`
- `step_requested: bool`

Recommended defaults for `v0.1`:

- `fixed_dt = 1.0 / 60.0`
- `max_frame_time = 0.25`

Suggested frame logic:

- accumulate clamped frame time
- if paused, only step when requested
- otherwise consume accumulator in fixed-size chunks

## Module Structure

Suggested repository layout:

```text
src/
  main.rs
  app/
    mod.rs
    app.rs
    scene.rs
    timing.rs
  physics/
    mod.rs
    world.rs
    body.rs
    collider.rs
    collision.rs
    contact.rs
    solver.rs
    integrate.rs
    material.rs
  render/
    mod.rs
    debug_draw.rs
    hud.rs
  input/
    mod.rs
    controls.rs
  math/
    mod.rs
    vec2.rs
    aabb.rs
    scalar.rs
tests/
  physics_smoke.rs
  collision_pairs.rs
  solver_stability.rs
examples/
  falling_objects.rs
  box_stack.rs
  bouncing_balls.rs
  friction_slide.rs
```

## Data Model

The data model should stay explicit and direct in `v0.1`.

Core runtime objects:

- `PhysicsWorld`
- `RigidBody`
- `Collider`
- `Contact`
- `Material`
- `Scene`
- `DebugSettings`

The world owns bodies and produces temporary per-step collision/contact data. Contacts should usually be step-local unless persistent contacts are deliberately introduced later.

Suggested ownership:

- `App` owns `SceneManager`, `PhysicsWorld`, `DebugSettings`, and timing state
- `PhysicsWorld` owns a `Vec<RigidBody>`
- `PhysicsWorld` also owns the current step's `Vec<Contact>` for debug draw and upcoming solver work
- `RigidBody` owns transform, velocities, collider, and material data
- solver and collision systems operate on mutable world state plus temporary contact buffers

## Rigid Body Structure

A minimal `RigidBody` for `v0.1` can look like this conceptually:

```text
RigidBody
- id
- body_type
- position
- velocity
- force_accumulator
- inverse_mass
- restitution
- friction
- collider
```

Suggested notes:

- represent mass as `inverse_mass` so static bodies can use `0.0`
- store `body_type` explicitly even if `inverse_mass == 0.0`
- use a force accumulator only if external forces beyond gravity are expected soon
- keep transforms simple; if rotation is not part of `v0.1`, avoid fake angular state
- if angle is introduced early, ensure only shape support that matches it is enabled

Suggested body categories:

- `Dynamic`
- `Static`

## Collider Representation

For `v0.1`, use a simple enum:

```text
enum Collider {
    Circle { radius: f32 },
    Aabb { half_extents: (f32, f32) },
}
```

Notes:

- collider data is local shape data
- body position defines world placement
- AABBs are axis-aligned only
- avoid designing for arbitrary convex shapes yet

Helper responsibilities:

- compute world-space bounds if needed
- dispatch narrow-phase checks by collider pair
- provide shape-specific debug draw primitives

## Contact Structure

A `Contact` should carry only the information needed by the solver and debug draw.

Suggested fields:

```text
Contact
- body_a
- body_b
- normal
- penetration
- point
- restitution
- friction
```

Optional step-local fields:

- accumulated impulse
- tangent impulse
- debug label or pair index

Conventions must be documented and consistent:

- `normal` points from `body_a` to `body_b`
- `penetration` is positive overlap depth
- `point` is in world coordinates

## Collision Pipeline

The collision pipeline for `v0.1` is intentionally simple.

### 1. Pair Generation

Use naive `O(n^2)` pair enumeration over bodies.

Skip pairs when:

- both bodies are static
- either body is inactive, if sleeping/activation exists later
- body has unsupported collider data

### 2. Narrow Phase

Dispatch by collider pair:

- circle-circle
- AABB-AABB
- circle-AABB

Each narrow-phase function should do one job:

- determine whether overlap exists
- return enough geometric information to build a contact

Avoid blending detection and impulse resolution logic.

### 3. Contact Generation

Convert narrow-phase results into `Contact` records.

For `v0.1`, one contact point per pair is acceptable. Multi-point manifolds can be deferred if single-point contacts are good enough for the first release.

For the current codebase, the M5 split is:

- keep pair generation and overlap checks in `collision.rs`
- construct `Contact` values after integration inside `PhysicsWorld::step`
- cache only the current step's contacts on the world for rendering and tests
- feed HUD overlap/contact counts from that cached contact buffer

## Solver Pipeline

The solver updates body velocities based on contacts and then applies positional correction.

Suggested order inside one fixed step:

1. integrate external forces into velocities
2. detect collisions
3. build contacts
4. iterate solver over contacts
5. apply positional correction
6. integrate positions from final velocities

This order works well with a simple sequential impulse design and keeps the data flow easy to inspect.

### Solver Notes

- apply impulses only to dynamic bodies
- compute relative velocity at contact
- solve normal impulse first
- solve friction impulse second
- clamp friction based on normal impulse magnitude
- use several solver iterations for stability
- apply small positional correction to reduce sinking and stacking drift

## Debug Rendering System

Debug rendering is a read-only visualization layer over simulation state.

It should support:

- collider outlines
- contact points
- contact normals
- body centers
- simple text HUD

Recommended design:

- `DebugDraw` consumes immutable references to world and contacts
- rendering primitives are generated from simulation state after stepping
- debug layers can be toggled independently

Suggested toggles:

- draw colliders
- draw contacts
- draw normals
- draw body IDs
- draw HUD

The renderer should not own simulation data or mutate it.

## Example Physics Step Pseudocode

```text
fn step(world, dt):
    integrate_velocities_from_forces(world, dt)

    collision_pairs = generate_pairs(world)
    contacts = []

    for pair in collision_pairs:
        if let Some(contact) = collide_and_generate_contact(world, pair):
            contacts.push(contact)

    for i in 0..solver_iterations:
        for contact in contacts:
            solve_normal_impulse(world, contact)
            solve_friction_impulse(world, contact)

    positional_correction(world, contacts)
    integrate_positions(world, dt)
```

An app-level fixed timestep loop can look like this:

```text
while app.running:
    frame_dt = clamp(get_frame_time(), 0.0, max_frame_time)
    input_state = poll_input()
    app.handle_input(input_state)

    if not app.paused:
        accumulator += frame_dt

    while accumulator >= fixed_dt:
        physics.step(fixed_dt)
        accumulator -= fixed_dt

    if app.paused and app.step_requested:
        physics.step(fixed_dt)
        app.step_requested = false

    render(world, debug_state)
```

## Responsibilities by Module

### `app`

Owns outer coordination.

Responsibilities:

- startup and shutdown
- frame loop
- scene loading and reset
- pause/step state
- fixed timestep accumulator
- top-level app state

### `physics`

Owns all simulation behavior.

Responsibilities:

- body storage
- world stepping
- gravity and integration
- collision detection
- contact generation
- impulse solving
- material mixing rules

### `render`

Owns visualization only.

Responsibilities:

- shape outlines
- contact and normal drawing
- HUD text
- optional world grid

### `input`

Owns translation from raw input to sandbox commands.

Responsibilities:

- pause toggle
- single-step request
- reset scene
- spawn commands
- scene switching

### `math`

Owns basic reusable math and geometry helpers.

Responsibilities:

- `Vec2`
- scalar helpers
- clamp/sign/epsilon helpers
- AABB geometry helpers
- distance and projection utilities

## Design Constraints for v0.1

The architecture should preserve these constraints:

- no ECS
- no advanced broad phase
- no CCD
- no joint system
- no editor stack
- no serialization requirements
- no renderer-driven simulation design

This keeps `v0.1` focused on understanding the rigid body pipeline end to end.

A useful rule for code review is that any module in `render` or `input` should be removable without changing physics results.

## Extension Paths After v0.1

The proposed architecture should support later additions without forcing immediate abstraction:

- additional collider types through `Collider` expansion
- better manifolds through richer `Contact` structures
- broad-phase insertion before narrow phase
- constraints and joints alongside contact solving
- replay/logging at the app or physics boundary
- scene loading separate from hardcoded demo scenes

The intent is not to prebuild these systems now, only to avoid blocking them structurally.
