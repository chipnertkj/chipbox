import { SceneNode } from './bindings/api';


// TODO: Implement.

/** Create an element with the given tag. */
export function createElement(tag: string): SceneNode {
    throw new Error("TODO");
}

/** Create a text node with the given contents. */
export function createTextNode(value: string): SceneNode {
    throw new Error("TODO");
}

/** Replace the contents of the given text node. */
export function replaceText(textNode: SceneNode, value: string): void {

}

/** Returns true if the given node is a text node. */
export function isTextNode(node: SceneNode): boolean {
    return false;
}

/** Update a property of the given node. */
export function setProperty<T>(node: SceneNode, name: string, value: T, prev?: T): void {

}

/** Inserts a node before the anchor (another node).
 *  If it is already a child of the parent, it will be moved to the specified location.
 */
export function insertNode(parent: SceneNode, node: SceneNode, anchor?: SceneNode): void {

}

/** Remove a node from the scene. */
export function removeNode(parent: SceneNode, node: SceneNode): void {

}

/** Get the parent node of the given node. */
export function getParentNode(node: SceneNode): SceneNode | undefined {
    return undefined;
}

/** Get the first child of the given node. */
export function getFirstChild(node: SceneNode): SceneNode | undefined {
    return undefined;
}

/** Get the next sibling of the given node. */
export function getNextSibling(node: SceneNode): SceneNode | undefined {
    return undefined;
}
