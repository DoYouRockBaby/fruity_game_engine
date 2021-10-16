class Service2 {
    constructor() {
        this.hello = this.hello.bind(this);
    }

    hello(str) {
        console.log("Hello", str);
    }

    hello2 = (str) => {
        console.log("Hello 2", str);
    }
}

class ComponentJs1 {
    constructor(args) {
        Object.assign(this, args);
    }
}

//services.register("service2", new Service2());
const systemManager = services.get("system_manager");
const entityManager = services.get("entity_manager");
const componentFactory = services.get("components_factory");
const windowsManager = services.get("windows_manager");

entityManager.create([new ComponentJs1({ str1: "test1", int1: 3 })]);

// const test_component_1 = componentFactory.instantiate("Component1", { float1: 10.101, int1: 30 });
const test_component_1 = new Component1({ float1: 10.101, int1: 30 });

console.log(test_component_1.int1, test_component_1.float1);

entityManager.create([test_component_1, new ComponentJs1({ str1: "test1", int1: 3 })]);

systemManager.addSystem(() => {
    console.log("JS System", windowsManager.getSize());

    entityManager
        .iterComponents(["ComponentJs1", "Component1"])
        .forEach(components => {
            console.log(components.get(0).int1, components.get(0).str1);
            console.log(components.get(1).int1, components.get(1).float1);
            components.get(0).int1 += 1;
        });

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