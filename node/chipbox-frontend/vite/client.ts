/**
 * Vite HMR client stub for QuickJS runtime.
 *
 * Provides the `createHotContext` function that solid-refresh uses to
 * register HMR callbacks. Global functions are exposed for Rust to call
 * when triggering updates.
 */

import { warn, error, info } from "chipbox:tracing";

interface HotContext {
    acceptCallbacks: Array<(mod?: any) => void>;
    disposeCallbacks: Array<(data: Record<string, unknown>) => void>;
    selfAccepted?: boolean;
    hot: HotModule;
}

interface HotModule {
    accept(cb?: (mod?: any) => void): void;
    dispose(cb: (data: Record<string, unknown>) => void): void;
    prune(cb: () => void): void;
    invalidate(): void;
    on(event: string, cb: () => void): void;
    send(event: string, data?: unknown): void;
    data: Record<string, unknown>;
}

// Global storage for hot contexts. Persists across reimports.
const g = globalThis as Record<string, unknown>;
if (!g.__vite_hot_contexts) {
    g.__vite_hot_contexts = new Map<string, HotContext>();
}
const hotContexts = g.__vite_hot_contexts as Map<string, HotContext>;

/**
 * Create or retrieve a hot context for a module.
 * solid-refresh expects the SAME context to persist across reimports.
 */
export function createHotContext(ownerPath: string): HotModule {
    const existing = hotContexts.get(ownerPath);
    if (existing) {
        // Clear old accept callbacks - new module will register fresh ones
        existing.acceptCallbacks = [];
        return existing.hot;
    }

    // First time this module is loaded - create new context
    const context: HotContext = {
        acceptCallbacks: [],
        disposeCallbacks: [],
        hot: {
            accept(cb) {
                if (cb) {
                    context.acceptCallbacks.push(cb);
                } else {
                    context.selfAccepted = true;
                }
            },
            dispose(cb) {
                if (cb) context.disposeCallbacks.push(cb);
            },
            prune(_cb) { },
            invalidate() {
                warn(`[HMR] invalidate() called for ${ownerPath}`);
            },
            on(_event, _cb) { },
            send(_event, _data) { },
            data: {},
        }
    };

    hotContexts.set(ownerPath, context);
    return context.hot;
}

/**
 * Trigger HMR update for a module. Called from Rust.
 * Returns true if handled, false if full reload needed.
 */
g.__vite_hot_accept = (path: string, newModule?: any): boolean => {
    const context = hotContexts.get(path);
    if (!context) {
        warn`[HMR] No hot context for ${path}`;
        return false;
    }

    // Run dispose callbacks
    for (const cb of context.disposeCallbacks) {
        try { cb(context.hot.data); } catch (e) { error`[HMR] dispose error: ${e}`; }
    }
    context.disposeCallbacks = [];

    // Run accept callbacks with new module
    for (const cb of context.acceptCallbacks) {
        try { cb(newModule); } catch (e) { error`[HMR] accept error: ${e}`; }
    }
    return true;
};

/**
 * Prune a module from the HMR registry. Called from Rust.
 */
g.__vite_hot_prune = (path: string): void => {
    const context = hotContexts.get(path);
    if (!context) return;

    for (const cb of context.disposeCallbacks) {
        try { cb(context.hot.data); } catch (e) { error(`[HMR] dispose error: ${e}`); }
    }

    hotContexts.delete(path);
    info(`[HMR] Pruned module: ${path}`);
};

/** Debug: list all registered hot modules. */
g.__vite_hot_list = (): string[] => [...hotContexts.keys()];

// Re-export for any code expecting these.
export const hot = null;
