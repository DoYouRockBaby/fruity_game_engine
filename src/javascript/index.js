let service1 = services.get("service1");
let system_manager = services.get("system_manager");

system_manager.add_system(() => {
    console.log("JS System start");
    console.log(service1.value());
    service1.increment();
    console.log(service1.value());
    service1.increment_by(3);
    console.log(service1.value());
    console.log("JS System end");
});
/*console.log(component1_mut.int1);
component1_mut.int1 += 2;
console.log(component1.int1);*/
