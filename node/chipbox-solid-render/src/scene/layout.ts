import type { LayoutLength } from '@bindings/scene';

export type SceneUnit = `${number}su`;
export type SceneWidth = `${number}sw`;
export type SceneHeight = `${number}sh`;
export type ParentWidth = `${number}pw`;
export type ParentHeight = `${number}ph`;
export type Pixel = `${number}px`;
export type Millimeter = `${number}mm`;
export type Centimeter = `${number}cm`;
export type Inch = `${number}in`;
export type Point = `${number}pt`;

export type LayoutLiteral =
    | SceneUnit
    | SceneWidth
    | SceneHeight
    | ParentWidth
    | ParentHeight
    | Pixel
    | Millimeter
    | Centimeter
    | Inch
    | Point;

export type LayoutParam = LayoutLiteral | LayoutLength | number;

export type LinearDirection = "forward" | "backward";

export type Axis = "horizontal" | "vertical" | "x" | "y";
