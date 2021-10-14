const service1 = services.get("service1");
const systemManager = services.get("system_manager");
const entityManager = services.get("entity_manager");

service1.incrementBy(3);
systemManager.addSystem(() => {
    console.log("JS System start");
    console.log(service1.value());
    service1.increment();
    console.log(service1.value());
    service1.incrementBy(3);
    console.log(service1.value());
    console.log("JS System end");

    entityManager
        .iterEntities(["Component1", "Component2"])
        .forEach(entity => {
            console.log(entity.lenght());
        });
});