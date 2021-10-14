const service1 = services.get("service1");
const systemManager = services.get("system_manager");
const entityManager = services.get("entity_manager");

//service1.incrementBy(3);
systemManager.addSystem(() => {
    /*console.log("JS System start");
    console.log(service1.value());
    service1.increment();
    console.log(service1.value());
    service1.incrementBy(3);
    console.log(service1.value());
    console.log("JS System end");*/

    entityManager
        .iterComponents(["Component1", "Component2"])
        .forEach(components => {
            console.log(components.get(0).int1, components.get(1).float1);
            components.get(0).int1 += 1;
        });
});