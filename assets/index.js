class CustomService {
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

class TestVec {
    constructor(args) {
        Object.assign(this, args);
    }
}

resourceContainer.add("custom_service", new CustomService());
const systemService = resourceContainer.get("system_service");
const entityService = resourceContainer.get("entity_service");
const windowsService = resourceContainer.get("windows_service");
const customService = resourceContainer.get("custom_service");
const inputService = resourceContainer.get("input_service");
const frameService = resourceContainer.get("frame_service");

customService.hello("World");

inputService.onPressed.addObserver((key) => {
    console.log("Pressed", key);
});

inputService.onReleased.addObserver((key) => {
    console.log("Released", key);
});

entityService.onEntityCreated.addObserver((entity) => {
    if (entity.contains(["Position", "Camera"])) {
        let position = entity.getComponent("Position");
        let camera = entity.getComponent("Camera");
        console.log("New camera");
        console.log("Position", position.pos.x, position.pos.y);
        console.log("Camera", camera.near, camera.far);
        console.log("Unknown", entity.getComponent("Unknown"));

        /*entity.onUpdated.addObserver(() => {
            let camera = entity.getComponent("Camera");
            console.log("New camera was updated", camera.near);
        });*/
    }
});

let player_entity_id = entityService.create("Player", [
    new Position({ pos: new Vector2d({ x: -0.25, y: 0.25 }) }),
    new Size({ size: new Vector2d({ x: 0.3, y: 0.3 }) }),
    new Sprite({
        material: resourceContainer.get("assets/material.material"),
        z_index: 1,
    }),
    new Move({ velocity: 0.2 }),
]);

entityService.create("Image 1", [
    new Parent({ parent_id: player_entity_id }),
    new LocalPosition({ pos: new Vector2d({ x: 0.1, y: 0.1 }) }),
    new Position({ pos: new Vector2d({ x: 0.25, y: 0.25 }) }),
    new Size({ size: new Vector2d({ x: 0.5, y: 0.5 }) }),
    new Sprite({
        material: resourceContainer.get("assets/material.material"),
        z_index: 0,
    }),
    new TestVec({ size: new Vector2d({ x: 0.5, y: 0.5 }) }),
]);

entityService.create("Camera", [
    new Position({ pos: new Vector2d({ x: -1.5, y: -1.3 }) }),
    new Size({ size: new Vector2d({ x: 3, y: 2 }) }),
    new Camera({}),
    // new Velocity({ vel: new Vector2d({ x: 0.05, y: 0.05 }) }),
]);

console.log("ENTITIES CREATED");

systemService.addBeginSystem(() => {
});

systemService.addSystem(() => {
    entityService
        .iterComponents(["Position", "Velocity"])
        .forEach(components => {
            components.get(0).pos = components.get(0).pos.add(components.get(1).vel.mul(frameService.delta));
        });
});

systemService.addSystem(() => {
    entityService
        .iterComponents(["Position", "Move"])
        .forEach(components => {
            let vel = new Vector2d({ x: 0, y: 0 });

            if (inputService.isPressed("Run Left")) {
                vel.x -= components.get(1).velocity;
            }

            if (inputService.isPressed("Run Right")) {
                vel.x += components.get(1).velocity;
            }

            if (inputService.isPressed("Jump")) {
                vel.y += components.get(1).velocity;
            }

            if (inputService.isPressed("Down")) {
                vel.y -= components.get(1).velocity;
                entityService.remove(player_entity_id);
            }

            components.get(0).pos = components.get(0).pos.add(vel.mul(frameService.delta));
        });
});