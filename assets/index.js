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

let player_entity_id = entityService.create("Player", [
    new Translate2d({ vec: new Vector2d({ x: -0.25, y: 0.25 }) }),
    new Rotate2d({ angle: 0.0 }),
    new Scale2d({ vec: new Vector2d({ x: 0.3, y: 0.3 }) }),
    new Sprite({
        material: resourceContainer.get("./assets/material.material"),
        z_index: 10,
    }),
    new Move({ velocity: 0.2 }),
    new Transform2d({}),
]);

let image_1 = entityService.create("Image 1", [
    new Parent({ parent_id: player_entity_id }),
    new Translate2d({ vec: new Vector2d({ x: 0.63, y: 0.32 }) }),
    new Scale2d({ vec: new Vector2d({ x: 0.5, y: 0.5 }) }),
    new Sprite({
        material: resourceContainer.get("./assets/material.material"),
        z_index: 0,
    }),
    new TestVec({ scale: new Vector2d({ x: 0.5, y: 0.5 }) }),
    new Transform2d({}),
]);

entityService.create("Image 2", [
    new Parent({ parent_id: image_1 }),
    new Translate2d({ vec: new Vector2d({ x: -0.15, y: 0.05 }) }),
    new Scale2d({ vec: new Vector2d({ x: 0.5, y: 0.5 }) }),
    new Sprite({
        material: resourceContainer.get("./assets/material.material"),
        z_index: 1,
    }),
    new Transform2d({}),
]);

entityService.create("Camera", [
    new Translate2d({ vec: new Vector2d({ x: 0, y: 0 }) }),
    new Scale2d({ vec: new Vector2d({ x: 3, y: 2 }) }),
    new Camera({}),
    // new Velocity({ vel: new Vector2d({ x: 0.05, y: 0.05 }) }),
    new Transform2d({}),
]);

console.log("ENTITIES CREATED");

systemService.addSystem(() => {
    entityService
        .iterComponents(["Translate2d", "Velocity"])
        .forEach(([translate, velocity]) => {
            translate.vec = translate.vec.add(translate.vel.mul(frameService.delta));
        });
});

systemService.addSystem(() => {
    entityService
        .iterComponents(["Translate2d", "Move"])
        .forEach(([translate, move]) => {
            let vel = new Vector2d({ x: 0, y: 0 });

            if (inputService.isPressed("Run Left")) {
                vel.x -= move.velocity;
            }

            if (inputService.isPressed("Run Right")) {
                vel.x += move.velocity;
            }

            if (inputService.isPressed("Jump")) {
                vel.y += move.velocity;
            }

            if (inputService.isPressed("Down")) {
                vel.y -= move.velocity;
            }

            translate.vec = translate.vec.add(vel.mul(frameService.delta));
        });
});

systemService.addSystem(() => {
    entityService
        .iterComponents(["Rotate2d", "Move"])
        .forEach(([rotate, move]) => {
            if (inputService.isPressed("Rotate")) {
                rotate.angle += move.velocity * frameService.delta;
            }
        });
});