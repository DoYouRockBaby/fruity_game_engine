class Service2 {
    constructor() {
        this.hello = this.hello.bind(this);
    }

    hello(str) {
        console.log("Hello", str);
    }
}

class Move {
    constructor(args) {
        Object.assign(this, args);
    }
}

class Velocity {
    constructor(args) {
        Object.assign(this, args);
    }
}

console.log("ICI1");
services.register("service2", new Service2());
console.log("ICI2");
const systemManager = services.get("system_manager");
const entityManager = services.get("entity_manager");
const componentFactory = services.get("components_factory");
const windowsManager = services.get("windows_manager");
const resourcesManager = services.get("resources_manager");
const service2 = services.get("service2");
const inputManager = services.get("input_manager");
const frameManager = services.get("frame_manager");

console.log("ICI8");

// console.log("JS System", windowsManager.getSize());
service2.hello("World");

inputManager.onPressed.addObserver((key) => {
    console.log("Pressed", key);
});

inputManager.onReleased.addObserver((key) => {
    console.log("Released", key);
});

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
    entityManager.create("Image 1", [
        new Position({ pos: new Vector2d({ x: 0.25, y: 0.25 }) }),
        new Size({ size: new Vector2d({ x: 0.5, y: 0.5 }) }),
        new Sprite({
            texture: resourcesManager.getResource("assets/logo.png"),
            material: resourcesManager.getResource("assets/material.material"),
        }),
    ]);

    entityManager.create("Player", [
        new Position({ pos: new Vector2d({ x: -0.25, y: 0.25 }) }),
        new Size({ size: new Vector2d({ x: 0.3, y: 0.3 }) }),
        new Sprite({
            texture: resourcesManager.getResource("assets/logo.png"),
            material: resourcesManager.getResource("assets/material.material"),
        }),
        new Move({ velocity: 0.2 }),
    ]);

    entityManager.create("Camera", [
        new Position({ pos: new Vector2d({ x: -1.5, y: -1 }) }),
        new Size({ size: new Vector2d({ x: 3, y: 2 }) }),
        new Camera({}),
        // new Velocity({ vel: new Vector2d({ x: 0.05, y: 0.05 }) }),
    ]);

    console.log("ENTITIES CREATED");
});

systemManager.addSystem(() => {
    entityManager
        .iterComponents(["Position", "Velocity"])
        .forEach(components => {
            components.get(0).pos = components.get(0).pos.add(components.get(1).vel.mul(frameManager.delta));
        });
});

systemManager.addSystem(() => {
    entityManager
        .iterComponents(["Position", "Move"])
        .forEach(components => {
            let vel = new Vector2d({ x: 0, y: 0 });

            if (inputManager.isPressed("Run Left")) {
                vel.x = components.get(1).velocity;
            }

            if (inputManager.isPressed("Run Right")) {
                vel.x = components.get(1).velocity;
            }

            if (inputManager.isPressed("Jump")) {
                vel.y = components.get(1).velocity;
            }

            if (inputManager.isPressed("Down")) {
                vel.y -= components.get(1).velocity;
            }

            components.get(0).pos = components.get(0).pos.add(vel.mul(frameManager.delta));
        });
});