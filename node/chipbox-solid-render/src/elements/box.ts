
import type { LayoutParam } from "../scene/layout";
import type { JSX } from "src/jsx-runtime";

export type JsxBoxProps = {
    id?: string;
    children?: JSX.ElementChildren;
    width: LayoutParam;
    height: LayoutParam;
}

export type JsxBox = JsxBoxProps & {
    type: "box";
};
