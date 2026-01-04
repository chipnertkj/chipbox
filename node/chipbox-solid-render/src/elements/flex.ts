import type { Axis, LinearDirection } from "../scene/layout";
import type { JsxFlexItem } from "./flex-item";
import type { JSX } from "src/jsx-runtime";

export type JsxFlexProps = {
    id?: string;
    children?: JSX.Children<JsxFlexItem>;
    axis?: Axis; // Defaults to "horizontal"
    direction?: LinearDirection; // Defaults to "forward"
};

export type JsxFlex = JsxFlexProps & {
    type: "flex";
};
