// Type declarations for chipbox:tracing native module.
declare module "chipbox:tracing" {
    /** Log a trace-level message through chipbox. */
    export function trace(message: string): void;
    /** Log a debug-level message through chipbox. */
    export function debug(message: string): void;
    /** Log an info-level message through chipbox. */
    export function info(message: string): void;
    /** Log a warn-level message through chipbox. */
    export function warn(message: string): void;
    /** Log an error-level message through chipbox. */
    export function error(message: string): void;
}
