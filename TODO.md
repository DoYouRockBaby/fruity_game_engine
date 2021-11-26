# FEATURES V1

## ECS

[X] Make serialized able to be extended in modules (put it into introspect, make intropect better ...)
[X] Make the archetype able to store datas in plain array for performances
[X] Add a "field changed" signal on every entities
[X] Put a get_constructor function into introspect
[X] Implements entity deletion
[X] Resources and services containers should be merged
[X] Abstract resources
[X] Split abstractions and implementations for all crates
[X] Make native systems easy to use
[X] Move ECS structure to a struct of array instead an array of struct (more efficient, easier to implement)
[X] Implement ECS hierarchy
[ ] Add an exception when you try to access a deleted entity, propagate it into js
[ ] There should be some memory leak in archetype
[ ] Test and stabilize ECS
[ ] Find a way to move the introspect declaration into the associated traits

## Editor

[X] Base editor
[X] Implement a basic hook system
[X] Create a pseudo DOM to abstract the interface creation
[X] Wrap the DOM system with iced
[ ] Expose the GUI API to javascript to create custom component editor
[X] Components visualization
[X] Entity hierarchy visualisation
[ ] Resources browser
[ ] Resources visualisation, take the material as an example and try to make it easy to edit in sprite component
[X] Sprite gizmos
[X] Run/Pause
[X] Save/Load

## Game tools

[X] Inputs
[X] Time service
[ ] Animations everywhere (including sprite animations)
[ ] State manager
[ ] Basic 2D physics (collision only)
[ ] Tiles editor (make something like RPG maker, as easy to use as possible)
[ ] Particles

### Nice to have

[ ] SplineBrush (something like spriteshape for lines but more easy to use, width should be modifiable, thought to be used with a graphic tablet)
[ ] ShapeBrush (something like spriteshape but more easy to use, thought to be used with a graphic tablet)
[ ] 2D skeletons (take inspiration with unity's one wich is realy nice)
[ ] Implements a complete physic engine
[ ] 2D lights

## Scripting

[ ] Find a better way to store SerializedComponent, to have something more effective on read
[ ] Allow to handle size variations of SerializedComponent
[ ] There should be some memory leak when a js object is released
[X] Javascript module is highly unsafe cause i force to put a one thread runtime into a multi thread host
[ ] Implement web workers in js
[X] Allow javascript to play with signals (depends on next)
[X] Fix javascript services
[ ] Make javascript able to import modules via package.json
[ ] Test and make microjob library compatible with javascript
[ ] Abstract everything (espescialy javascript and render)
[ ] Typescript

## Others

[ ] Implement a basic sound features
[X] Dynamic libraries rust
[ ] Hot reload js
[ ] Hot reload rust

# Code clean

[ ] Put an auto-analyser and clean the code
[ ] Tests everywhere
[ ] Rust doc everywhere
[ ] Remove as many unwrap as possible (ecs and javascript)
[ ] A lot of unsafe code were created to avoid lifetime issue in ecs, remove as many as possible
[ ] Use a tool that detect unused dependencies