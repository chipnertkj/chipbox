export type JsxTextureProps = {
    id?: string;
    opacity?: number; // Defaults to 1.0
    shader: unknown; // () in Rust, unknown in TS
};

export type JsxTexture = JsxTextureProps & {
    type: "texture";
};
