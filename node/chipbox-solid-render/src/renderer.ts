import type { JSX } from './jsx-runtime';

// TODO: Implement.

/** Create an element with the given tag. */
export function createElement(tag: string): JSX.Element {
    return {
        type: "box",
        id: undefined,
        children: undefined,
        width: "0su",
        height: "0su",
    };
}

/** Create a text node with the given contents. */
export function createTextNode(value: string): JSX.Element {
    return {
        type: "text",
        id: undefined,
        children: undefined,
        font: "",
        weight: 400,
        size: 16,
        color: undefined,
    };
}

/** Replace the contents of the given text node. */
export function replaceText(textNode: JSX.Element, value: string): void {

}

/** Returns true if the given node is a text node. */
export function isTextNode(node: JSX.Element): boolean {
    return false;
}

/** Update a property of the given node. */
export function setProperty<T>(node: JSX.Element, name: string, value: T, prev?: T): void {

}

/** Inserts a node before the anchor (another node).
 *  If it is already a child of the parent, it will be moved to the specified location.
 */
export function insertNode(parent: JSX.Element, node: JSX.Element, anchor?: JSX.Element): void {

}

/** Remove a node from the scene. */
export function removeNode(parent: JSX.Element, node: JSX.Element): void {

}

/** Get the parent node of the given node. */
export function getParentNode(node: JSX.Element): JSX.Element | undefined {
    return undefined;
}

/** Get the first child of the given node. */
export function getFirstChild(node: JSX.Element): JSX.Element | undefined {
    return undefined;
}

/** Get the next sibling of the given node. */
export function getNextSibling(node: JSX.Element): JSX.Element | undefined {
    return undefined;
}
