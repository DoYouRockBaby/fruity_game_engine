interface Component1 {
  int1: number;
  str1: string;
}

interface Service1 {
  increment(): void;
  value(): number;
}

export function system1(): void {
  print("System1 speak from typescript");
}

/*export function system1(component1: Component1, service1: Service1): void {
  component1.int1 += 1;

  service1.increment();
  console.log("System1 speak from typescript: " + component1.int1.toString() + " " + component1.str1 + " " + service1.value().toString());
}*/