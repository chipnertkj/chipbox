import type { JSX } from "src/jsx-runtime";

export type JsxFlexItemProps = {
    id?: string;
    children?: JSX.ElementChildren;
    proportion?: number; // Defaults to 1.0
};

export type JsxFlexItem = JsxFlexItemProps & {
    type: "flex-item";
};
