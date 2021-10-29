class Service2 {
    constructor() {
        this.hello = this.hello.bind(this);
    }

    hello(str) {
        console.log("Hello", str);
    }
}

class Velocity {
    constructor(args) {
        Object.assign(this, args);
    }
}

console.log("ICI");
services.register("service2", new Service2());
const systemManager = services.get("system_manager");
const entityManager = services.get("entity_manager");
const componentFactory = services.get("components_factory");
const windowsManager = services.get("windows_manager");
const resourcesManager = services.get("resources_manager");
const service2 = services.get("service2");

console.log("ICI");

console.log("JS System", windowsManager.getSize());
service2.hello("World");

entityManager.onEntityCreated.addObserver((entity) => {
    if (entity.contains(["Position", "Camera"])) {
        let position = entity.getComponent("Position");
        let camera = entity.getComponent("Camera");
        console.log("New camera");
        console.log("Position", position.x, position.y);
        console.log("Camera", camera.near, camera.far);
        console.log("Unknown", entity.getComponent("Unknown"));

        /*entity.onUpdated.addObserver(() => {
            let camera = entity.getComponent("Camera");
            console.log("New camera was updated", camera.near);
        });*/
    }
});


systemManager.addBeginSystem(() => {
    resourcesManager.readResourceSettings("assets/resources.yaml");

    entityManager.create("Image 1", [
        new Position({ x: 0.25, y: 0.25 }),
        new Size({ width: 0.5, height: 0.5 }),
        new Sprite({
            texture: resourcesManager.getResource("assets/logo.png"),
            material: resourcesManager.getResource("assets/material.material"),
        }),
        new Velocity({ x: 0.001, y: 0.001 }),
    ]);

    entityManager.create("Image 2", [
        new Position({ x: -0.25, y: 0.25 }),
        new Size({ width: 0.3, height: 0.3 }),
        new Sprite({
            texture: resourcesManager.getResource("assets/logo.png"),
            material: resourcesManager.getResource("assets/material.material"),
        }),
    ]);

    entityManager.create("Camera", [
        new Position({ x: -1, y: -1 }),
        new Size({ width: 2, height: 2 }),
        new Camera({}),
        new Velocity({ x: 0.001, y: 0.000 }),
    ]);

    console.log("ENTITIES CREATED");
});

systemManager.addSystem(() => {
    entityManager
        .iterComponents(["Position", "Velocity"])
        .forEach(components => {
            components.get(0).x += components.get(1).x;
            components.get(0).y += components.get(1).y;
        });
});