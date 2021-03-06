# FEATURES V1

## ECS

[X] Make serialized able to be extended in modules (put it into introspect, make intropect better ...)
[X] Make the archetype able to store datas in plain array for performances
[X] Put a get_constructor function into introspect
[X] Implements entity deletion
[X] Resources and services containers should be merged
[X] Abstract resources
[X] Split abstractions and implementations for all crates
[X] Make native systems easy to use
[X] Move ECS structure to a struct of array instead an array of struct (more efficient, easier to implement)
[X] Implement ECS hierarchy
[X] Remove the "Entity" component to replace it by a more cache friendly solution
[X] Add a way to have multiple components of the same type on a single entity
[X] The use of SOA should allow us to have a more effective storage based on type and not on binary encoding
[X] There should be some memory leak in archetype
[X] Add component query selectors
[X] Create a way to query based on generics
[X] Change the begin/end system to a startup system that returns an optional end callback and add the startup that ignore pause
[X] Random crash when we delete an entity
[X] Add a way to extend components
[ ] Support for async systems/queries for_each with tokio
[ ] Find a way to move the introspect declaration into the associated traits

## Editor

[X] Base editor
[X] Implement a basic hook system
[X] Create a pseudo DOM to abstract the interface creation
[X] Wrap the DOM system with iced
[X] Components visualization
[X] Entity hierarchy visualisation
[X] Resources browser
[X] Sprite gizmos
[X] Run/Pause
[X] Save/Load
[X] Add component from editor
[X] Remove component from editor
[X] Fix resize gizmos
[X] Add a cool grid for 2D view
[ ] Expose the GUI API to javascript to create custom component editor
[ ] Resources visualisation, take the material as an example and try to make it easy to edit in sprite component
[ ] Select an object when clicking on it (set_scissor_rect)
[ ] Add a cool free camera for 2D view

## Graphics

[X] Material should store wgpu bind groups
[X] Supports Meshes
[X] Material fields should now be a string identifier
[X] Sprite vertex/indices should be shared accross all sprites
[X] Squad transform should be done in shader instead of CPU
[X] Proceed instantied rendering
[X] Make instances parametrizable in material/shader
[ ] Implements spritesheet
[ ] Implement rendering composers

## Animation

[ ] Add an optional way to interpolate between serialized values
[ ] Add an animation system with keyframes
[ ] Create an editor for keyframes
[ ] Create a state system
[ ] Create interpolation between two states
[ ] Create an editor for states

## Physics 2D

[X] Primitive physics collider components (shared accross every physics engine)
[ ] Mesh physics collider components (shared accross every physics engine)
[ ] Implements basic Rapier crate features
[ ] Implements a collision only physic engine

## Game tools

[X] Inputs
[X] Time service
[ ] Tiles editor (make something like RPG maker, as easy to use as possible)
[ ] Particles

## Nice to have

[ ] SplineBrush (something like spriteshape for lines but more easy to use, width should be modifiable, thought to be used with a graphic tablet)
[ ] ShapeBrush (something like spriteshape but more easy to use, thought to be used with a graphic tablet)
[ ] 2D skeletons (take inspiration with unity's one wich is realy nice)
[ ] Implements a complete physic engine
[ ] 2D lights

## Scripting

[X] Javascript module is highly unsafe cause i force to put a one thread runtime into a multi thread host
[X] Allow javascript to play with signals (depends on next)
[X] Fix javascript services
[X] Abstract everything (espescialy javascript and render)
[ ] Find a better way to store SerializedComponent, to have something more effective on read
[ ] Allow to handle size variations of SerializedComponent
[ ] There should be some memory leak when a js object is released
[ ] Implement web workers in js
[ ] Make javascript able to import modules via package.json
[ ] Test and make microjob library compatible with javascript
[ ] Typescript
[ ] Expose rust-like enums in serialized
[ ] Create a macro to expose the enums easely to serialized
[ ] Support for async systems/queries for_each

## Others

[X] Dynamic libraries rust
[X] Implements a profiling tool
[X] Hot reload js
[ ] Hot reload rust
[ ] Implement a basic sound features

## Code clean

[ ] Put an auto-analyser and clean the code
[ ] Tests everywhere
[ ] Rust doc everywhere
[ ] Remove as many unwrap as possible (ecs and javascript)
[ ] A lot of unsafe code were created to avoid lifetime issue in ecs, remove as many as possible
[ ] Use a tool that detect unused dependencies

# FEATURES V2

## For the future

[ ] Unreal released a new version of there engine with something called the Nanites, try to make some research about it (https://youtu.be/TMorJX3Nj6U)