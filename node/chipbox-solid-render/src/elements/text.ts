import type { Color } from "@bindings/scene";
import type { JSX } from "src/jsx-runtime";

export type JsxTextProps = {
    id?: string;
    children?: JSX.ElementChildren;
    font: string;
    weight: number;
    size: number;
    color?: Color; // Optional since it might not be in the current wrapper
};

export type JsxText = JsxTextProps & {
    type: "text";
};
