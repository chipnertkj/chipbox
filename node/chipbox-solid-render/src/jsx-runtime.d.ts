import { children } from "solid-js/types/server/reactive.js";
import { AnyNode } from "./bindings/chipbox-scene";

declare namespace JSX {
    interface IntrinsicElements {
        box: any;
    }

    type Element = AnyNode;

    // Define which property name corresponds to children.
    interface ElementChildrenAttribute {
        children: {}
    }
}

export function jsx(type: any, props: any): JSX.Element {
    const { children = [], ...rest } = props ?? {};
    // Intrinsic element.
    if (typeof type === "function") {
        return type(rest);
    }
    if (typeof type === "string") {
        // We assume this may be called by `jsxs` too. Room for optimization.
        const childrenArray = Array.isArray(children) ? children : [children];
        return {
            type,
            ...rest,
            children: childrenArray,
        } as AnyNode;
    }
    throw new Error("TODO");
}

export function jsxs(type: any, props: any): JSX.Element {
    // TODO: optimization: could take advantage of the fact that children is an array.
    return jsx(type, props);
}

export function Fragment(props: { children: any }): JSX.Element {
    return props.children;
}
