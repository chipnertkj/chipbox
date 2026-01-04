import type { Axis, LinearDirection } from "../scene/layout";
import type { JsxBox } from "./box";
import type { JSX } from "src/jsx-runtime";

export type JsxGridProps = {
    id?: string;
    children?: JSX.Children<JsxBox>;
    axis?: Axis; // Defaults to "horizontal"
    direction?: LinearDirection; // Defaults to "forward"
    array_direction?: LinearDirection; // Defaults to "forward"
    array_limit: number; // NonZeroUsize in Rust, number in TS
};

export type JsxGrid = JsxGridProps & {
    type: "grid";
};
