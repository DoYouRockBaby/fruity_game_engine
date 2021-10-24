# ROADMAP

[X] Make the archetype able to store datas without referencing for performances
[X] Make serialized able to be extended in modules (put it into introspect, make intropect better ...)
[ ] Add a "field changed" signal on every entities
[ ] Allow javascript to play with signals
[ ] Implement entity hierarchy

# LESS PRIORITY

[ ] Put an auto-analyser and clean the code
[ ] Tests everywhere
[ ] Rust doc everywhere
[ ] Remove as many unwrap as possible (ecs and javascript)
[X] A lot of unsafe code were created to avoid lifetime issue in ecs, remove as many as possible
[ ] Find a better way to store SerializedComponent, to have something more effective on read
[ ] There should be some memory leak in archetype
[X] Javascript module is highly unsafe cause i force to put a one thread runtime into a multi thread host
[ ] Implement web workers in js
[ ] Fix javascript services
[ ] Make javascript able to import modules via package.json
[ ] Test and make microjob library compatible with javascript
[ ] Abstract everything (espescialy javascript and render)