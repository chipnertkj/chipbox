# chipbox

`chipbox` is an open-source desktop [DAW](https://en.wikipedia.org/wiki/Digital_audio_workstation) written in [Rust](https://www.rust-lang.org/).

## Context

This repository is currently set up for my own convenience &mdash; the project is in early development.

The current setup uses a [QuickJS](https://bellard.org/quickjs/)-based runtime to run a [SolidJS](https://www.solidjs.com/) frontend written in [TypeScript](https://www.typescriptlang.org/) inside the application.

By default, the frontend code is embedded into the final executable. During development, a [Vite](https://vitejs.dev/) development server is used to host it instead, which allows for hot reloading through [Hot Module Replacement](https://vite.dev/guide/features.html#hot-module-replacement). Since the execution environment is not a browser, it also implements a custom [HMR client](https://vite.dev/guide/api-hmr.html).

The frontend uses a custom [DSL](https://en.wikipedia.org/wiki/Domain-specific_language)/framework based on the [DOM](https://en.wikipedia.org/wiki/Document_Object_Model) to describe its UI. It maps to graphics primitives, as well as explicit layout and behavioral elements. These are then processed by the Rust application to produce a scene graph, which is finally rendered with [Vello](https://github.com/linebender/vello), a GPU renderer.

## Planned features

The following is subject to change, although things already listed here are of high priority.

- An audio [node graph system](https://en.wikipedia.org/wiki/Node_graph_architecture) that enables users to create flexible synthesizers and effects chains.
- [VST](https://en.wikipedia.org/wiki/Virtual_Studio_Technology), [CLAP](https://cleveraudio.org/), [SF2](https://en.wikipedia.org/wiki/SoundFont), [SFZ](https://sfzformat.com/) support.
- High configurability, with a user-friendly interface.
- Editor extensibility through custom [WebAssembly](https://webassembly.org/) modules, [Lua](https://www.lua.org/about.html) and/or [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript) scripting.
- At least [ASIO](https://en.wikipedia.org/wiki/Audio_Stream_Input/Output), [WASAPI](https://learn.microsoft.com/en-us/windows/win32/coreaudio/wasapi), [JACK](https://jackaudio.org/), [ALSA](https://www.alsa-project.org/wiki/Main_Page) audio backends.
- [MIDI](https://en.wikipedia.org/wiki/MIDI) I/O.
- [Microtonal](https://en.wikipedia.org/wiki/Microtonal_music) support.
