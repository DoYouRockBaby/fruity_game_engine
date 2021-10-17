[ ] Tests for fruity_collections
[ ] Tests for javascript
[ ] Tests for fruity_ecs
[ ] Rust doc for fruity_javascript_scripting
[ ] Remove as many unwrap as possible (ecs and javascript)
[ ] A lot of unsafe code were created to avoid lifetime issue in ecs, remove as many as possible
[ ] There should be some memory leak in component storage (in Entity) and entity storage (in EncodableVec)
[X] Javascript module is highly unsafe cause i force to put a one thread runtime into a multi thread host
[ ] Implement web workers in js
[ ] Fix javascript services
[ ] Make javascript able to import modules via package.json
[ ] Test and make microjob library compatible with javascript
[ ] Fin a better way to store SerializedComponent, to have something more effective on read (bincode ?)
[ ] Maybe separate base services/component and Introspect implementation
[ ] Abstract everything (espescialy javascript and render)