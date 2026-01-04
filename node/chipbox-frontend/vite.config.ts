import { defineConfig, Plugin, UserConfig, } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import { fileURLToPath } from 'url';
import { readFileSync } from 'fs';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const renderer = "chipbox-solid-render";

/// Entry point.
export default defineConfig((configEnv) => {
    // If true, we are using the development server.
    const dev = configEnv.command === 'serve';
    return config(dev);
});

/// Vite configuration.
function config(dev: boolean): UserConfig {
    const mode = dev ? 'development' : 'production';

    return {
        // Embedded base path.
        base: './',
        // Force mode based on execution mode.
        mode: mode,
        define: {
            // Define `NODE_ENV` based on execution mode.
            'process.env.NODE_ENV': JSON.stringify(mode),
        },
        plugins: plugins(dev),
        // Copy over files from `./public/`.
        publicDir: resolve(__dirname, 'public'),
        // Alias resolution.
        resolve: {
            // Renderer may use a symlink due to `pnpm`.
            preserveSymlinks: true,
            dedupe: ['solid-js'],
            alias: {
                '@bindings': resolve(__dirname, `../${renderer}/generated`),
            },
        },
        // We use `vite-plugin-solid` to handle JSX.
        esbuild: { jsx: 'preserve' },
        // Dev server config.
        server: dev
            ? {
                // Allow local development only.
                allowedHosts: ["localhost"],
                // Don't open the browser.
                open: false,
                // We use HMR for module updates.
                hmr: {
                    protocol: 'ws',
                    host: 'localhost',
                    port: 24678,
                },
                // File watcher config.
                watch: {},
                // Allow Vite to serve/watch files from local node modules.
                fs: {
                    allow: ['..']
                }
            }
            // Disable server in build-only mode.
            : undefined,
        // Release build config.
        build: {
            // QuickJS targets and almost fully supports ES2020.
            target: 'es2020',
            // N/A in this context.
            modulePreload: false,
            // Use `publicDir` instead.
            assetsDir: undefined,
            // We don't use CSS.
            cssCodeSplit: false,
            cssTarget: undefined,
            cssMinify: false,
            // Enable sourcemaps to keep errors readable.
            sourcemap: true,
            // Bundler config.
            rollupOptions: {
                input: { main: resolve(__dirname, 'src/main.tsx') },
                output: {
                    // Use ES module files.
                    format: 'es',
                    entryFileNames: '[name].js',
                    // Force single JS bundle.
                    manualChunks: undefined,
                    // Force single chunk even with dynamic imports.
                    inlineDynamicImports: true,
                },
                // Native chipbox modules are handled by rquickjs runtime.
                external: [/^chipbox:/],
            },
            // Don't minify. Keep readable in production.
            minify: false,
            // We don't care.
            reportCompressedSize: false,
            // Don't watch without dev server.
            watch: null,
        },
        optimizeDeps: {
            // Pre-bundle SolidJS.
            include: ['solid-js'],
            exclude: [
                // Exclude web-only dependencies.
                'solid-js/web', '@solid-js/router',
                // Renderer may be actively developed at runtime.
                renderer
            ],
        },
    };
}

function plugins(dev: boolean): Plugin[] {
    return [
        // SolidJS integration.
        solidPlugin({
            solid: {
                moduleName: renderer,
                generate: 'universal',
            },
        }),
        // Append any plugins added dynamically.
        ...devPlugins(dev),
    ];
}

function devPlugins(dev: boolean): Plugin[] {
    return dev ? [
        // Externalize chipbox:* native modules for rquickjs runtime.
        chipboxExternalPlugin(),
        // Stub /@vite/client for QuickJS (no browser WebSocket).
        viteClientStubPlugin(),
        // Dump served modules to a JSON file.
        dumpServedPlugin(),
        // Send HMR updates to QuickJS on file change.
        quickjsHmrPlugin(),
    ] : [];
}

function read_local_utf8(path: string): string {
    return readFileSync(resolve(__dirname, path), 'utf8');
}

/**
 * Stub /@vite/client for QuickJS runtime.
 *
 * Vite serves /@vite/client via internal middleware, so we intercept
 * the HTTP request directly and return our stub.
 *
 * solid-refresh expects the SAME hot context to persist
 * across module reimports. We store contexts globally and return the
 * existing one if it exists.
 */
function viteClientStubPlugin(): Plugin {
    const vite_client_stub = read_local_utf8('vite-client-stub.js');
    return {
        name: 'vite-client-stub',
        enforce: 'pre',
        configureServer(server) {
            // Intercept before Vite's internal middleware
            server.middlewares.use((req, res, next) => {
                if (req.url === '/@vite/client' || req.url?.startsWith('/@vite/client?')) {
                    res.setHeader('Content-Type', 'application/javascript');
                    res.end(vite_client_stub);
                    return;
                }
                next();
            });
        },
    };
}

/** Mark `chipbox:*` imports as external so they pass through to rquickjs. */
function chipboxExternalPlugin(): Plugin {
    const chipbox_external_stub = read_local_utf8('chipbox-external-stub.ts');
    return {
        name: 'chipbox-external',
        enforce: 'pre',
        resolveId(id) {
            if (id.startsWith('chipbox:')) {
                return { id, external: true };
            }
        },
        load(id) {
            if (id.startsWith('chipbox:')) {
                return chipbox_external_stub;
            }
        },
    };
}


function dumpServedPlugin(): Plugin {
    return {
        name: 'dump-served',
        configureServer(server) {
            server.middlewares.use('/__served', (_req, res) => {
                const mods = [...server.moduleGraph.idToModuleMap.values()]
                    .map(m => ({
                        id: m.id,
                        file: m.file,
                        url: m.url,
                    }));
                res.setHeader('content-type', 'application/json');
                res.end(JSON.stringify(mods, null, 2));
            });
        },
    };
}

function log(tag: string, message: string) {
    console.log(`[${tag}] ${message}`);
}

function hmrLog(message: string) {
    log('HMR', message);
}

function quickjsHmrPlugin(): Plugin {
    return {
        name: 'quickjs-hmr',
        configureServer(server) {
            server.ws.on('connection', (socket) => {
                hmrLog('WS client connected');
                socket.on('close', () => {
                    hmrLog('WS client disconnected');
                });
            });
        },
    };
}
