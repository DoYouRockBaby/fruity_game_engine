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

systemService.addSystem("test 1", () => {
    entityService
        .iterComponents(["Translate2d", "Velocity"])
        .forEach(([translate, velocity]) => {
            translate.vec = translate.vec.add(translate.vel.mul(frameService.delta));
        });
});

systemService.addSystem("test 2", () => {
    entityService
        .iterComponents(["Translate2d", "Move"])
        .forEach(([translate, move]) => {
            let vel = new Vector2d({ x: 0, y: 0 });

            if (inputService.isPressed("Run Left")) {
                vel.x -= move.velocity;
            }

            if (inputService.isPressed("Run Right")) {
                vel.x += move.velocity * 10;
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
        .iterComponents(["Rotate2d", "Move"])
        .forEach(([rotate, move]) => {
            if (inputService.isPressed("Rotate")) {
                rotate.angle += move.velocity * frameService.delta;
            }
        });
});