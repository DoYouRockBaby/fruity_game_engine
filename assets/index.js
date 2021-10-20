/*class Service2 {
    constructor() {
        this.hello = this.hello.bind(this);
    }

    hello(str) {
        console.log("Hello", str);
    }

    hello2 = (str) => {
        console.log("Hello 2", str);
    }
}*/

class Velocity {
    constructor(args) {
        Object.assign(this, args);
    }
}

console.log("ICI");
//services.register("service2", new Service2());
const systemManager = services.get("system_manager");
const entityManager = services.get("entity_manager");
const componentFactory = services.get("components_factory");
const windowsManager = services.get("windows_manager");
const resourcesManager = services.get("resources_manager");

console.log("ICI");


systemManager.addBeginSystem(() => {
    resourcesManager.readResourceSettings("assets/resources.yaml");

    entityManager.create([
        new Position({ x: 0.25, y: 0.25 }),
        new Size({ width: 0.5, height: 0.5 }),
        new Sprite({
            texture: resourcesManager.getResource("assets/logo.png"),
            material: resourcesManager.getResource("assets/material.material"),
        }),
        new Velocity({ x: 0.001, y: 0.001 })]);

    entityManager.create([
        new Position({ x: 0.75, y: 0.75 }),
        new Size({ width: 0.8, height: 0.8 }),
        new Sprite({ texture: resources_manager.get_resource("assets/logo.png") })]);
});

systemManager.addSystem(() => {
    entityManager
        .iterComponents(["Position", "Velocity"])
        .forEach(components => {
            components.get(0).x += components.get(1).x;
            components.get(0).y += components.get(1).y;
        });

    /*console.log("JS System", windowsManager.getSize());
    console.log("1");
    service2.hello("World");
    service2.hello2("World");
    console.log("1");*/

    /*const service1 = services.get("service1");
    const service2 = services.get("service2");
    const entityManager = services.get("entity_manager");

    console.log("1");
    service2.hello("World");
    service2.hello2("World");
    console.log("1");

    service1.incrementBy(3);
    console.log("2");*/

    /*console.log("JS System start");
    console.log(service1.value());
    service1.increment();
    console.log(service1.value());
    service1.incrementBy(3);
    console.log(service1.value());
    console.log("JS System end");*/

    /*entityManager
        .iterComponents(["Component1", "Component2"])
        .forEach(components => {
            console.log(components.get(0).int1, components.get(1).float1);
            components.get(0).int1 += 1;
        });*/
});