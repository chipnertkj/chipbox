/**
 * ES Module to CommonJS-like transformation for QuickJS.
 *
 * Transforms ES modules into a format suitable for manual module management,
 * enabling fast refresh by allowing modules to be reloaded independently.
 */

import { parse } from '@babel/parser';
import traverse from '@babel/traverse';
import {
    identifier,
    memberExpression,
    assignmentExpression,
    expressionStatement,
    functionExpression,
    variableDeclarator,
    variableDeclaration,
    classExpression,
    stringLiteral,
    callExpression,
    isFunctionDeclaration,
    isVariableDeclaration,
    isClassDeclaration,
    isIdentifier,
    isImportDefaultSpecifier,
    isImportNamespaceSpecifier,
    isImportSpecifier,
    isExportSpecifier,
} from '@babel/types';
import generate from '@babel/generator';

interface CollectedImport {
    specifiers: any[];
    source: string;
}

interface TransformResult {
    imports: CollectedImport[];
    hasExports: boolean;
}

/**
 * Known Vite internal paths that we handle.
 * Any `/@...` import not in this list will trigger a warning.
 */
const KNOWN_VITE_INTERNALS = new Set([
    '/@vite/client',      // HMR client - we stub this
    '/@solid-refresh',    // Solid HMR - needed for fast refresh
    '/@id/',              // Bare specifier resolution prefix
]);

/** Check if an import source is a known Vite internal. */
function isKnownViteInternal(source: string): boolean {
    // Check exact matches
    if (KNOWN_VITE_INTERNALS.has(source)) return true;
    // Check prefix matches (like /@id/...)
    for (const known of KNOWN_VITE_INTERNALS) {
        if (known.endsWith('/') && source.startsWith(known)) return true;
    }
    return false;
}

/** Warn about unknown Vite internal imports. */
function warnUnknownViteInternal(source: string, modulePath: string): void {
    console.warn(
        `[transform] Unknown Vite internal import "${source}" in ${modulePath}. ` +
        `This may not work in QuickJS. Add to KNOWN_VITE_INTERNALS if intentional.`
    );
}

/** Transform ES module source code for QuickJS module management. */
export function transformModule(code: string, modulePath: string): string {
    // Extract sourcemap comment before transformation
    const { code: codeWithoutSourcemap, sourcemap } = extractSourcemap(code);

    const ast = parseModule(codeWithoutSourcemap);
    const { imports, hasExports } = collectImportsAndTransformExports(ast, modulePath);
    const { code: bodyCode } = generate(ast, { retainLines: false, compact: false });
    const importCode = imports.map(generateImportCode);

    // Wrap and append sourcemap at the end
    const transformed = wrapModule(bodyCode, importCode, modulePath, hasExports, sourcemap);
    return transformed;
}

/** Extract sourcemap comment from code. */
function extractSourcemap(code: string): { code: string; sourcemap: string | null } {
    // Match both inline and URL sourcemap comments, capture the full data URI
    const sourcemapRegex = /\n?\/\/[#@]\s*sourceMappingURL=(.+)$/;
    const match = code.match(sourcemapRegex);
    if (match) {
        return {
            code: code.slice(0, match.index),
            sourcemap: match[1],
        };
    }
    return { code, sourcemap: null };
}

function parseModule(code: string) {
    return parse(code, {
        sourceType: 'module',
        plugins: ['jsx', 'typescript', 'dynamicImport', 'importMeta'],
    });
}

function collectImportsAndTransformExports(ast: ReturnType<typeof parse>, modulePath: string): TransformResult {
    const imports: CollectedImport[] = [];
    let hasExports = false;

    traverse(ast, {
        ImportDeclaration(path) {
            const source = path.node.source.value;

            // chipbox:* imports use dynamic import() - handled by native module loader
            if (source.startsWith('chipbox:')) {
                imports.push({
                    specifiers: path.node.specifiers,
                    source,
                });
                path.remove();
                return;
            }

            // Warn about unknown Vite internals
            if (source.startsWith('/@') && !isKnownViteInternal(source)) {
                warnUnknownViteInternal(source, modulePath);
            }

            imports.push({
                specifiers: path.node.specifiers,
                source,
            });
            path.remove();
        },
        ExportNamedDeclaration(path) {
            hasExports = true;
            transformNamedExport(path);
        },
        ExportDefaultDeclaration(path) {
            hasExports = true;
            transformDefaultExport(path);
        },
        ExportAllDeclaration(path) {
            hasExports = true;
            const source = path.node.source.value;

            // Warn about unknown Vite internals
            if (source.startsWith('/@') && !isKnownViteInternal(source)) {
                warnUnknownViteInternal(source, modulePath);
            }

            transformExportAll(path);
        },
        // Transform import.meta -> __import_meta (not valid in scripts)
        MetaProperty(path) {
            if (path.node.meta.name === 'import' && path.node.property.name === 'meta') {
                path.replaceWith(identifier('__import_meta'));
            }
        },
    });

    return { imports, hasExports };
}

function transformNamedExport(path: any): void {
    const { declaration, specifiers } = path.node;

    if (declaration) {
        if (isFunctionDeclaration(declaration) && declaration.id) {
            const name = declaration.id.name;
            path.replaceWith(
                expressionStatement(
                    assignmentExpression(
                        '=',
                        memberExpression(identifier('exports'), identifier(name)),
                        functionExpression(
                            identifier(name),
                            declaration.params,
                            declaration.body,
                            declaration.generator,
                            declaration.async
                        )
                    )
                )
            );
        } else if (isVariableDeclaration(declaration)) {
            // Handle both simple identifiers and destructuring patterns
            const statements: any[] = [];

            for (const decl of declaration.declarations) {
                if (isIdentifier(decl.id)) {
                    // Simple: export const foo = bar
                    const name = decl.id.name;
                    statements.push(
                        variableDeclaration(declaration.kind, [
                            variableDeclarator(
                                identifier(name),
                                assignmentExpression(
                                    '=',
                                    memberExpression(identifier('exports'), identifier(name)),
                                    decl.init || identifier('undefined')
                                )
                            )
                        ])
                    );
                } else if (decl.id.type === 'ObjectPattern') {
                    // Destructuring: export const { a, b } = fn()
                    // Keep the original destructuring declaration
                    statements.push(variableDeclaration(declaration.kind, [decl]));

                    // Add export assignments for each destructured property
                    for (const prop of decl.id.properties) {
                        if (prop.type === 'ObjectProperty' && isIdentifier(prop.value)) {
                            const name = prop.value.name;
                            statements.push(
                                expressionStatement(
                                    assignmentExpression(
                                        '=',
                                        memberExpression(identifier('exports'), identifier(name)),
                                        identifier(name)
                                    )
                                )
                            );
                        } else if (prop.type === 'RestElement' && isIdentifier(prop.argument)) {
                            const name = prop.argument.name;
                            statements.push(
                                expressionStatement(
                                    assignmentExpression(
                                        '=',
                                        memberExpression(identifier('exports'), identifier(name)),
                                        identifier(name)
                                    )
                                )
                            );
                        }
                    }
                } else {
                    // Other patterns - keep as-is
                    statements.push(variableDeclaration(declaration.kind, [decl]));
                }
            }

            path.replaceWithMultiple(statements);
        } else if (isClassDeclaration(declaration) && declaration.id) {
            const name = declaration.id.name;
            path.replaceWith(
                expressionStatement(
                    assignmentExpression(
                        '=',
                        memberExpression(identifier('exports'), identifier(name)),
                        classExpression(
                            identifier(name),
                            declaration.superClass,
                            declaration.body,
                            declaration.decorators
                        )
                    )
                )
            );
        }
    } else if (specifiers.length > 0) {
        const source = path.node.source;

        if (source) {
            // Re-export: export { X, Y } from 'source'
            // Convert to: const { X, Y } = await __qjs_require('source'); exports.X = X; exports.Y = Y;
            const sourceValue = source.value;
            const localNames = specifiers
                .filter(isExportSpecifier)
                .map((spec: any) => spec.local.name);
            const exportedNames = specifiers
                .filter(isExportSpecifier)
                .map((spec: any) => isIdentifier(spec.exported) ? spec.exported.name : spec.exported.value);

            // Build destructuring: const { a, b } = await __qjs_require('source');
            const destructure = localNames.map((local: string, i: number) => {
                const exported = exportedNames[i];
                return local === exported ? local : `${local}: ${exported}`;
            }).join(', ');

            // Build assignments: exports.X = X; exports.Y = Y;
            const assignments = specifiers
                .filter(isExportSpecifier)
                .map((spec: any) => {
                    const exportedName = isIdentifier(spec.exported)
                        ? spec.exported.name
                        : spec.exported.value;
                    return expressionStatement(
                        assignmentExpression(
                            '=',
                            memberExpression(identifier('exports'), identifier(exportedName)),
                            identifier(spec.local.name)
                        )
                    );
                });

            // Create the require + destructure statement
            // We'll use a raw string approach since building AST for await is complex
            const requireCode = `const { ${localNames.join(', ')} } = await __qjs_require(${JSON.stringify(sourceValue)});`;

            // Parse the require statement to get AST nodes
            const requireAst = parse(requireCode, { sourceType: 'module' });
            const requireStatement = requireAst.program.body[0];

            path.replaceWithMultiple([requireStatement, ...assignments]);
        } else {
            // Regular re-export of local variables: export { X, Y }
            const assignments = specifiers
                .filter(isExportSpecifier)
                .map((spec: any) => {
                    const exportedName = isIdentifier(spec.exported)
                        ? spec.exported.name
                        : spec.exported.value;
                    return expressionStatement(
                        assignmentExpression(
                            '=',
                            memberExpression(identifier('exports'), identifier(exportedName)),
                            identifier(spec.local.name)
                        )
                    );
                });
            path.replaceWithMultiple(assignments);
        }
    }
}

function transformDefaultExport(path: any): void {
    path.replaceWith(
        expressionStatement(
            assignmentExpression(
                '=',
                memberExpression(identifier('exports'), identifier('default')),
                path.node.declaration
            )
        )
    );
}

function transformExportAll(path: any): void {
    // Object.assign(exports, await __qjs_require(...))
    path.replaceWith(
        expressionStatement(
            callExpression(
                memberExpression(identifier('Object'), identifier('assign')),
                [
                    identifier('exports'),
                    {
                        type: 'AwaitExpression',
                        argument: callExpression(identifier('__qjs_require'), [
                            stringLiteral(path.node.source.value),
                        ]),
                    } as any,
                ]
            )
        )
    );
}

function generateImportCode(imp: CollectedImport): string {
    const { specifiers, source } = imp;
    const sourceJson = JSON.stringify(source);

    // chipbox:* imports use dynamic import() - handled by native module loader
    // Check both chipbox: and /@id/chipbox: (Vite may resolve it)
    if (source.startsWith('chipbox:') || source.startsWith('/@id/chipbox:')) {
        // Use /@id/chipbox: path to match resolver
        const chipboxPath = source.startsWith('/@id/')
            ? source
            : `/@id/${source}`;
        const chipboxPathJson = JSON.stringify(chipboxPath);
        if (specifiers.length === 0) {
            return `await import(${chipboxPathJson});`;
        }

        let defaultImport: string | null = null;
        let namespaceImport: string | null = null;
        const namedImports: string[] = [];

        for (const spec of specifiers) {
            if (isImportDefaultSpecifier(spec)) {
                defaultImport = spec.local.name;
            } else if (isImportNamespaceSpecifier(spec)) {
                namespaceImport = spec.local.name;
            } else if (isImportSpecifier(spec)) {
                const imported = isIdentifier(spec.imported)
                    ? spec.imported.name
                    : spec.imported.value;
                const local = spec.local.name;
                namedImports.push(imported === local ? imported : `${imported}: ${local}`);
            }
        }

        if (namespaceImport) {
            return `const ${namespaceImport} = await import(${chipboxPathJson});`;
        }

        const destructure: string[] = [];
        if (defaultImport) {
            destructure.push(`default: ${defaultImport}`);
        }
        destructure.push(...namedImports);
        return `const { ${destructure.join(', ')} } = await import(${chipboxPathJson});`;
    }

    // Vite modules use __qjs_require
    if (specifiers.length === 0) {
        return `await __qjs_require(${sourceJson});`;
    }

    let defaultImport: string | null = null;
    let namespaceImport: string | null = null;
    const namedImports: string[] = [];

    for (const spec of specifiers) {
        if (isImportDefaultSpecifier(spec)) {
            defaultImport = spec.local.name;
        } else if (isImportNamespaceSpecifier(spec)) {
            namespaceImport = spec.local.name;
        } else if (isImportSpecifier(spec)) {
            const imported = isIdentifier(spec.imported)
                ? spec.imported.name
                : spec.imported.value;
            const local = spec.local.name;
            namedImports.push(imported === local ? imported : `${imported}: ${local}`);
        }
    }

    if (namespaceImport) {
        return `const ${namespaceImport} = await __qjs_require(${sourceJson});`;
    }

    const destructure: string[] = [];
    if (defaultImport) {
        destructure.push(`default: ${defaultImport}`);
    }
    destructure.push(...namedImports);
    return `const { ${destructure.join(', ')} } = await __qjs_require(${sourceJson});`;
}

function wrapModule(
    bodyCode: string,
    importCode: string[],
    modulePath: string,
    hasExports: boolean,
    // TODO: figure out the sourcemap problem
    // this is invalidated - we should not even have a sourcemap at this point in the pipeline
    sourcemap: string | null
): string {
    const registerCall = hasExports
        ? `__qjs_register_module(${JSON.stringify(modulePath)}, exports);`
        : '';

    // __import_meta replaces import.meta (not valid in scripts)
    // Provides .url for solid-refresh's createHotContext(import.meta.url)
    // .hot is initially null but gets set by solid-refresh's injected code
    // Async IIFE to support await __qjs_require(...)
    const modulePathJson = JSON.stringify(modulePath);
    return `(async function() {
  const exports = {};
  const module = { exports };
  const __import_meta = { url: ${modulePathJson}, hot: null };

${importCode.join('\n')}

${bodyCode}

  return module;
})();`;
}
