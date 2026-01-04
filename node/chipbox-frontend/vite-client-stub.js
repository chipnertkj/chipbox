// Stub for `rquickjs`.
// Global storage for hot contexts. Persists across reimports.
if (!globalThis.__vite_hot_contexts) {
    globalThis.__vite_hot_contexts = new Map();
}
const hotContexts = globalThis.__vite_hot_contexts;

export function createHotContext(ownerPath) {
    // Return existing context if module is being reimported (HMR)
    const existing = hotContexts.get(ownerPath);
    if (existing) {
        // Clear old accept callbacks - new module will register fresh ones
        existing.acceptCallbacks = [];
        return existing.hot;
    }

    // First time this module is loaded - create new context
    let context = {
        acceptCallbacks: [],
        disposeCallbacks: [],
        hot: {
            // Accept callback - solid-refresh registers its update handler here
            accept(cb) {
                if (cb) context.acceptCallbacks.push(cb);
            },
            // Dispose callback - cleanup before module is replaced
            dispose(cb) {
                if (cb) context.disposeCallbacks.push(cb);
            },
            // Prune callback - module removed entirely
            prune(cb) { },
            // Force full reload
            invalidate() {
                console.warn('[HMR] invalidate() called for', ownerPath);
            },
            // Event listeners (unused in QuickJS)
            on(event, cb) { },
            send(event, data) { },
            // Persistent data across reimports - solid-refresh stores registry here
            data: {},
        }
    };

    hotContexts.set(ownerPath, context);
    return context.hot;
}

// Rust calls this to trigger fine-grained HMR update.
// Returns true if handled, false if full reload needed.
globalThis.__vite_hot_accept = (path) => {
    const context = hotContexts.get(path);
    if (!context) {
        console.warn('[HMR] No hot context for', path);
        return false;
    }

    // Run dispose callbacks first (cleanup old module)
    for (const cb of context.disposeCallbacks) {
        try { cb(context.hot.data); } catch (e) { console.error('[HMR] dispose error:', e); }
    }
    context.disposeCallbacks = [];

    // Run accept callbacks (solid-refresh does component diffing here)
    for (const cb of context.acceptCallbacks) {
        try { cb(); } catch (e) { console.error('[HMR] accept error:', e); }
    }

    return true;
};

// Rust calls this to prune a module (removed from module graph).
// Runs dispose callbacks and removes from registry.
globalThis.__vite_hot_prune = (path) => {
    const context = hotContexts.get(path);
    if (!context) {
        return; // Already pruned or never existed
    }

    // Run dispose callbacks (cleanup)
    for (const cb of context.disposeCallbacks) {
        try { cb(context.hot.data); } catch (e) { console.error('[HMR] dispose error:', e); }
    }

    // Remove from registry entirely
    hotContexts.delete(path);
    console.log('[HMR] Pruned module:', path);
};

// Debug: list all registered hot modules
globalThis.__vite_hot_list = () => [...hotContexts.keys()];

// Re-export for any code expecting these.
export const hot = null;
