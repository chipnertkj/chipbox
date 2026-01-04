import { createRenderer } from 'solid-js/universal';
import * as impl from './renderer';
import type { JSX } from './jsx-runtime';

// Define SolidJS universal renderer.
export const {
    render,
    effect,
    memo,
    createComponent,
    createElement,
    insertNode,
    insert,
    spread,
    setProp,
    mergeProps,
} = createRenderer<JSX.Element>({
    createElement: impl.createElement,
    createTextNode: impl.createTextNode,
    replaceText: impl.replaceText,
    isTextNode: impl.isTextNode,
    setProperty: impl.setProperty,
    insertNode: impl.insertNode,
    removeNode: impl.removeNode,
    getParentNode: impl.getParentNode,
    getFirstChild: impl.getFirstChild,
    getNextSibling: impl.getNextSibling,
});

// Simple typed wrapper for createComponent
function createTypedComponent<TProps, TReturn extends JSX.Element>(
    Comp: (props: TProps) => TReturn,
    props: TProps
): TReturn {
    return createComponent(
        Comp,
        props
    ) as TReturn;
}

// Export control flow components.
export {
    For,
    Show,
    Suspense,
    SuspenseList,
    Switch,
    Match,
    Index,
    ErrorBoundary,
} from "solid-js";
