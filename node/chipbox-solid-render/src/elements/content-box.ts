import type { JSX } from "src/jsx-runtime";

export type JsxContentBoxProps = {
    id?: string;
    children?: JSX.ElementChildren;
};

export type JsxContentBox = JsxContentBoxProps & {
    type: "content-box";
};
