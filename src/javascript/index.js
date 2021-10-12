console.log("1 He, i'm in typescript 2", { test: 2, test1: "testestst" }, 12);
console.debug("2 He, i'm in typescript 2", { test: 2, test1: "testestst" }, 12);
console.info("3 He, i'm in typescript 2", { test: 2, test1: "testestst" }, 12);
console.warn("4 He, i'm in typescript 2", { test: 2, test1: "testestst" }, 12);
console.error("5 He, i'm in typescript 2", { test: 2, test1: "testestst" }, 12);

console.log(service1.value());
service1.increment();
console.log(service1.value());
service1.increment_by(3);
console.log(service1.value());