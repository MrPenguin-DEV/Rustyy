# AETHER Engine

A Rust-first, ECS-oriented real-time 3D engine prototype focused on predictable performance, a viewport-first workflow, and a hybrid rasterization + signed-distance-field (SDF) renderer.

> **Status:** early foundation. This repository is the engine project; it is not yet a production-ready game engine. Performance goals such as 200+ FPS are benchmark targets, not guarantees.

## Current foundation

- Rust workspace with small, independently testable crates.
- SDF math primitives: sphere, axis-aligned box, smooth union, finite-difference normals, and sphere tracing.
- Tests for core distance-field behavior.
- Architecture designed to keep render geometry, collision geometry, and simulation data independent.

## Planned architecture

| Area | Direction |
|---|---|
| Runtime | Rust, data-oriented ECS |
| Rendering | wgpu foundation; Vulkan-capable backend path |
| Materials | Unified physically based material model |
| Geometry | Mesh rendering + bounded SDF raymarching |
| Detail management | Adaptive Polygon Budget (screen-space LOD) |
| Textures | Mipmapping, streaming, screen-space resolution selection |
| Physics | Pluggable backend; evaluate Rapier and Jolt |
| Editor | Viewport-first, drag-and-drop assets and materials |
| Profiling | Frame-time budgets, GPU/CPU profiling, repeatable benchmarks |

## Build and test

```sh
cargo test --workspace
```

## Design principles

1. Prefer measurable frame-time budgets over marketing FPS claims.
2. Never couple visual LOD to collision or physics correctness.
3. Keep expensive features optional and quality-scalable.
4. Use bounded work, culling, streaming, and asynchronous asset loading.
5. Keep third-party licenses and attribution explicit.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the initial technical plan.
