import { defineConfig, Plugin, UserConfig, } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';
import { transformModule } from './vite/module-transform';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const renderer = 'chipbox-solid-render';

function info(tag: string, message: string) {
    console.log(`[${tag}] ${message}`);
}

function warn(tag: string, message: string) {
    console.warn(`[${tag}] ${message}`);
}

function error(tag: string, message: string) {
    console.error(`[${tag}] ${message}`);
}

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
        // Transform imports for manual module management.
        // Must run BEFORE viteClientStubPlugin so res.end is wrapped first.
        transformImportsPlugin(),
        // Override /@vite/client for QuickJS.
        viteClientPlugin(),
        // Dump served modules to a JSON file.
        dumpServedPlugin(),
        // Send HMR updates to QuickJS on file change.
        quickjsHmrPlugin(),
    ] : [];
}

/**
 * Override /@vite/client for QuickJS runtime.
 *
 * Intercepts the HTTP request for /@vite/client and serves our stub
 * instead. Uses transformRequest to process the TypeScript file through
 * Vite's pipeline (compilation, HMR tracking). The transform middleware
 * then converts it to __qjs_require format.
 *
 * solid-refresh expects the SAME hot context to persist
 * across module reimports. We store contexts globally and return the
 * existing one if it exists.
 */
function viteClientPlugin(): Plugin {
    // Path relative to project root for Vite's module graph
    const clientStubUrl = '/vite/client.ts';
    return {
        name: 'vite-client-stub',
        configureServer(server) {
            server.middlewares.use(async (req, res, next) => {
                if (req.url === '/@vite/client' || req.url?.startsWith('/@vite/client?')) {
                    try {
                        // Process our stub through Vite's transform pipeline
                        const result = await server.transformRequest(clientStubUrl);
                        if (result) {
                            // Append source map if available
                            let code = result.code;
                            if (result.map) {
                                const mapBase64 = Buffer.from(JSON.stringify(result.map)).toString('base64');
                                code += `\n//# sourceMappingURL=data:application/json;base64,${mapBase64}`;
                            }
                            res.setHeader('Content-Type', 'application/javascript');
                            res.end(code);
                            return;
                        }
                    } catch (e) {
                        error('vite-client-stub', `${e} @ ${req.url}`);
                    }
                }
                next();
            });
        },
    };
}

/**
 * Mark `chipbox:*` imports as external so they pass through to rquickjs.
 * These modules are provided by the Rust runtime, not Vite.
 */
function chipboxExternalPlugin(): Plugin {
    return {
        name: 'chipbox-external',
        enforce: 'pre',
        resolveId(id) {
            if (id.startsWith('chipbox:')) {
                // Return the id itself so Vite knows we're handling it
                return id;
            }
        },
        load(id) {
            if (id.startsWith('chipbox:')) {
                // Provide empty exports - actual module comes from rquickjs at runtime
                return 'export default {}';
            }
        },
    };
}

/**
 * Transform served JS modules for QuickJS.
 *
 * Intercepts responses by wrapping res.end() BEFORE Vite's middleware runs.
 * Transforms ALL JavaScript served by Vite (source, deps, internals) to use
 * __qjs_require instead of ES imports.
 */
function transformImportsPlugin(): Plugin {
    return {
        name: 'transform-imports',
        enforce: 'pre',
        configureServer(server) {
            // Add middleware BEFORE Vite's (no return) to wrap res.end
            server.middlewares.use((req, res, next) => {
                const url = req.url;
                if (!url) return next();

                const modulePath = url.split('?')[0];
                const originalEnd = res.end.bind(res);

                // @ts-ignore - overriding end with compatible signature
                res.end = function (chunk?: any, encoding?: any, callback?: any) {
                    const isJS = res.getHeader('Content-Type')?.toString().includes('javascript');
                    if (chunk && isJS) {
                        try {
                            const code = typeof chunk === 'string' ? chunk : chunk.toString('utf-8');
                            const transformed = transformModule(code, modulePath);
                            return originalEnd(transformed, 'utf-8', callback);
                        } catch (e) {
                            error('transform-imports', `${e} @ ${modulePath}`);
                        }
                    }
                    return originalEnd(chunk, encoding, callback);
                };

                next();
            });
        },
    };
}

/**
 * Dump served modules to a JSON file.
 */
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

/**
 * Log HMR connection events.
 */
function quickjsHmrPlugin(): Plugin {
    return {
        name: 'quickjs-hmr',
        configureServer(server) {
            server.ws.on('connection', (socket) => {
                info('hmr', 'WS client connected');
                socket.on('close', () => {
                    info('hmr', 'WS client disconnected');
                });
            });
        },
    };
}
