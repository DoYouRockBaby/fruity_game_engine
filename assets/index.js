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

    velocity = 1.0;
}

class Velocity {
    constructor(args) {
        Object.assign(this, args);
    }

    velocity = 1.0;
}

class TestVec {
    constructor(args) {
        Object.assign(this, args);
    }

    scale = new Vector2d({ x: 0, y: 0 });
}

resourceContainer.add("custom_service", new CustomService());
const systemService = resourceContainer.get("system_service");
const entityService = resourceContainer.get("entity_service");
const windowsService = resourceContainer.get("windows_service");
const customService = resourceContainer.get("custom_service");
const inputService = resourceContainer.get("input_service");
const frameService = resourceContainer.get("frame_service");

customService.hello("World");

systemService.addStartupSystem("test startup 0", () => {
    console.log("Je commence tout");

    return () => {
        console.log("Je finis tout");
    }
}, new StartupSystemParams({
    ignorePause: true,
}));

systemService.addStartupSystem("test startup 1", () => {
    console.log("Je commence");

    return () => {
        console.log("Je finis");
    };
});

systemService.addStartupSystem("test startup 2", () => {
    let handle = entityService
        .query()
        .withName()
        .with("Translate2d")
        .with("Velocity")
        .onCreated((name) => {
            console.log(`Entity created ${name}`);

            return () => {
                console.log(`Entity deleted ${name}`);
            }
        });

    return () => {
        handle.dispose();
    }
});

systemService.addStartupSystem("test startup 3", () => {
    let createdEntityId = null;
    const materialResource = resourceContainer.get("./assets/material.material");

    inputService.onPressed.addObserver(input => {
        if (input === "Action 1") {
            createdEntityId = entityService.create("New Entity", true, [
                new Transform2d({}),
                new Sprite({ zIndex: 30, material: materialResource }),
                new Translate2d({ vec: new Vector2d({ x: 1, y: 1 }) }),
                new Velocity({ velocity: 1.0 }),
            ]);
        }
    });

    inputService.onReleased.addObserver(input => {
        if (input === "Action 1") {
            entityService.remove(createdEntityId);
        }
    });
});

systemService.addSystem("test 1", () => {
    entityService
        .query()
        .with("Translate2d")
        .with("Velocity")
        .forEach((translate, velocity) => {
            const beforeTranslate = translate.vec;
            translate.vec = beforeTranslate.add(beforeTranslate.mul(velocity.velocity * frameService.delta));
        });
});

systemService.addSystem("test 2", () => {
    entityService
        .query()
        .withEntity()
        .with("Translate2d")
        .with("Move")
        .forEach((entity, translate, move) => {
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

systemService.addSystem("test 3", () => {
    entityService
        .query()
        .with("Rotate2d")
        .with("Move")
        .forEach((rotate, move) => {
            if (inputService.isPressed("Rotate")) {
                rotate.angle += move.velocity * frameService.delta;
            }
        });
});