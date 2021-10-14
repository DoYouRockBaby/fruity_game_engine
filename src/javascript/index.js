let service1 = services.get("service1");
let system_manager = services.get("system_manager");
let entity_manager = services.get("entity_manager");

service1.incrementBy(3);
system_manager.addSystem(() => {
    console.log("JS System start");
    console.log(service1.value());
    service1.increment();
    console.log(service1.value());
    service1.incrementBy(3);
    console.log(service1.value());
    console.log("JS System end");

    entity_manager.forEachEntity(["Component1", "Component2"], (entity) => {
        console.log("len: " + entity.length());
    });
});
