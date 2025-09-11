import { createRenderer } from 'solid-js/universal';
import { SceneNode } from './bindings/api';
import * as impl from './renderer';

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
} = createRenderer<SceneNode>({
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

// Forward SolidJS control flow.
export {
  For,
  Show,
  Suspense,
  SuspenseList,
  Switch,
  Match,
  Index,
  ErrorBoundary
} from "solid-js";
