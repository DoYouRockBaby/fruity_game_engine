# FEATURES V1

## ECS

[X] Make serialized able to be extended in modules (put it into introspect, make intropect better ...)
[X] Make the archetype able to store datas in plain array for performances
[P] Test and stabilize ECS
[P] Implement entity hierarchy
[X] Add a "field changed" signal on every entities
[ ] There should be some memory leak in archetype
[ ] Make macros for native systems

## Editor

[ ] Base editor
[ ] Entity hierarchy visualisation
[ ] Components visualization
[ ] Resources browser
[ ] Resources visualisation, take the material as an example and try to make it easy to edit in sprite component
[ ] Gizmos

## Game tools

[ ] Inputs
[ ] Time service
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
[ ] Dynamic libraries rust
[ ] Hot reload js
[ ] Hot reload rust

# Code clean

[ ] Put an auto-analyser and clean the code
[ ] Tests everywhere
[ ] Rust doc everywhere
[ ] Remove as many unwrap as possible (ecs and javascript)
[X] A lot of unsafe code were created to avoid lifetime issue in ecs, remove as many as possible