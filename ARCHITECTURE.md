# AETHER Architecture (initial)

## Runtime

AETHER is ECS-first. Entities are identifiers; components are plain data; systems own behavior. An editor hierarchy may provide an optional authoring view, but a mandatory node tree is not the runtime's source of truth.

## Rendering strategy

Start with a cross-platform `wgpu` renderer while preserving a backend boundary so Vulkan-specific capabilities can be used where available. The renderer is hybrid:

- Rasterize conventional meshes for cars, characters, and detailed imported assets.
- Raymarch SDF objects inside conservative bounds for procedural shapes, blended forms, and terrain/detail effects.
- Share one material model so mesh and SDF surfaces can use consistent lighting parameters.

The first SDF milestone is CPU-side math and tests. GPU integration follows after the renderer shell exists.

## SDF implementation notes

Use signed distance functions for primitives and composable operations. Smooth union should be explicit about its blend radius. Sphere tracing must have a maximum step count, maximum travel distance, and epsilon; it must not be an unbounded loop. Future GPU implementations should use conservative bounds and screen-space step limits.

Inigo Quilez's published articles are useful mathematical references. Reimplement concepts independently and review the license of every source snippet or shader before incorporating it. Do not assume ShaderToy code is automatically reusable in a commercial engine.

## Adaptive Polygon Budget (APB)

Select mesh LOD from projected screen-space size and a global geometry budget. Preserve silhouette quality, use simpler shadow meshes where appropriate, and consider impostors for distant objects. Collision meshes and physics state are independent from render LOD.

## Adaptive texture resolution

Use mipmaps and streaming first. Select mip levels from screen-space derivatives and available memory. Keep material channels coherent; allow lower-cost settings for distant or low-priority assets. Virtual texturing is a later open-world feature, not a prerequisite.

## Physics

Hide the physics backend behind an interface. Benchmark Rapier and Jolt against the same test scenes before committing. Use a fixed simulation timestep, render interpolation, continuous collision detection where needed, and automated regression tests. Vehicle dynamics require a dedicated model for suspension, tire forces, drivetrain, differential, braking, and load transfer.

## Performance target

A 200 FPS target implies a 5 ms total frame budget. Treat it as a target for defined hardware and a defined benchmark scene, not a universal guarantee. Track CPU, GPU, memory, streaming stalls, and simulation stability separately.
