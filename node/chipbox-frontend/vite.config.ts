import { defineConfig, Plugin } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const renderer = "chipbox-solid-render";

export default defineConfig((configEnv) => {
    // Whether we are using the development server, or using Vite as the bundler.
    const dev = configEnv.command === 'serve';
    const mode = dev && 'development' || 'production';

    const pluginsToAppend: Plugin[] =
        // Add this set only on debug.
        dev ? [
            // Send HMR updates to QuickJS on file change.
            {
                name: 'quickjs-hmr',
                handleHotUpdate({ file, server }) {
                    server.ws.send({
                        type: 'update',
                        updates: [{
                            type: 'js-update',
                            path: file,
                            acceptedPath: file,
                            timestamp: Date.now()
                        }]
                    });
                    server.watcher.emit("change", file);
                },
            },
        ]
            // Otherwise, add nothing.
            : [];

    return {
        // Embedded base path.
        base: './',
        // Force mode based on execution mode.
        mode: mode,
        define: {
            // Define `NODE_ENV` based on execution mode.
            'process.env.NODE_ENV': JSON.stringify(mode),
        },
        plugins: [
            // SolidJS integration.
            solidPlugin({
                solid: {
                    moduleName: renderer,
                    generate: 'universal',
                },
            }),
            // Append any plugins added dynamically.
            ...pluginsToAppend,
        ],
        // Copy over files from `./public/`.
        publicDir: resolve(__dirname, 'public'),
        // Alias resolution.
        resolve: {
            // `chipbox-solid-render` may use a symlink due to `pnpm`.
            preserveSymlinks: true,
            dedupe: ['solid-js'],
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
                hmr: true,
                // File watcher config.
                watch: {},
                // We implement a custom HMR client.
                injectClientScripts: false,
                // Enable sourcemaps to keep errors readable.
                sourcemap: true,
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
                // Bundle everything (nothing is external).
                external: [],
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
});
