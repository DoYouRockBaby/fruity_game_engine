import { print2 } from "./log2.js";

export const test = "test";

export function print(value) {
    print2(value.toString() + "\n");
}