import type { JSX } from "src/jsx-runtime";
import type { Axis, LinearDirection } from "../scene/layout";
import type { JsxBox } from "./box";

export type JsxArrayProps = {
    id?: string;
    children?: JSX.Children<JsxBox>;
    axis?: Axis; // Defaults to "horizontal"
    direction?: LinearDirection; // Defaults to "forward"
};

export type JsxArray = JsxArrayProps & {
    type: "array";
};
