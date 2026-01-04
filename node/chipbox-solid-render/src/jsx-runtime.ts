import type { BoxElement, ElementNode } from "@bindings/scene";
import type {
    JsxBox, JsxBoxProps,
    JsxMargin, JsxMarginProps,
    JsxContentBox, JsxContentBoxProps,
    JsxArray, JsxArrayProps,
    JsxGrid, JsxGridProps,
    JsxFlex, JsxFlexProps,
    JsxFlexItem, JsxFlexItemProps,
    JsxTexture, JsxTextureProps,
    JsxText, JsxTextProps,
} from "./elements";

export namespace JSX {
    export interface IntrinsicElements {
        box: JsxBoxProps;
        margin: JsxMarginProps;
        "content-box": JsxContentBoxProps;
        array: JsxArrayProps;
        grid: JsxGridProps;
        flex: JsxFlexProps;
        "flex-item": JsxFlexItemProps;
        texture: JsxTextureProps;
        text: JsxTextProps;
    }

    export type Element = JsxBox | JsxMargin | JsxContentBox
        | JsxArray | JsxGrid | JsxFlex | JsxFlexItem | JsxTexture | JsxText;
    interface ChildArray<T> extends Array<T> { }
    export type Children<T> = T | ChildArray<Children<T>> | (() => Children<T>);
    export type ElementChildren = Children<Element>;
    export interface ElementChildrenAttribute {
        children: ElementChildren;
    }
}

export function jsx(type: "box", props: JsxBoxProps): JsxBox;
export function jsx(type: "margin", props: JsxMarginProps): JsxMargin;
export function jsx(type: "content-box", props: JsxContentBoxProps): JsxContentBox;
export function jsx(type: "array", props: JsxArrayProps): JsxArray;
export function jsx(type: "grid", props: JsxGridProps): JsxGrid;
export function jsx(type: "flex", props: JsxFlexProps): JsxFlex;
export function jsx(type: "flex-item", props: JsxFlexItemProps): JsxFlexItem;
export function jsx(type: "texture", props: JsxTextureProps): JsxTexture;
export function jsx(type: "text", props: JsxTextProps): JsxText;
export function jsx(type: any, props: any): JSX.Element {
    const [children, ...rest] = props;
    const childrenArray = Array.isArray(children) ? children : [children];
    const mappedProps = { children: childrenArray, ...rest };
    switch (typeof type) {
        case "function":
            return type(mappedProps);
        case "string":
            switch (type) {
                case "box":
                    return { type: "box", ...mappedProps };
                case "margin":
                    return { type: "margin", ...mappedProps };
                case "content-box":
                    return { type: "content-box", ...mappedProps };
                case "array":
                    return { type: "array", ...mappedProps };
                case "grid":
                    return { type: "grid", ...mappedProps };
                case "flex":
                    return { type: "flex", ...mappedProps };
            }
            throw new Error(`Unknown element type: ${type}`);
    }
    throw new Error(`Unknown node type: ${typeof type} ${type}`);
}

export function jsxs(type: any, props: any): JSX.Element {
    // PERF: Could take advantage of the fact that children is an array.
    return jsx(type, props);
}

export function Fragment(props: { children: any }): JSX.Element {
    return props.children;
}
