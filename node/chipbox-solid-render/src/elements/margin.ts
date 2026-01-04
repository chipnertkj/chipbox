import type { LayoutParam } from "../scene/layout";
import type { JSX } from "src/jsx-runtime";

export type JsxMargin = JsxMarginProps & {
    type: "margin";
};

export type JsxMarginProps = {
    id?: string;
    children?: JSX.ElementChildren;
    base?: LayoutParam;
    horizontal?: LayoutParam;
    left?: LayoutParam;
    right?: LayoutParam;
    vertical?: LayoutParam;
    top?: LayoutParam;
    bottom?: LayoutParam;
};
