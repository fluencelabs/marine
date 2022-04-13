import { isBrowser, isNode } from 'browser-or-node';
import { defaultNames } from '.';

const bufferToSharedArrayBuffer = (buffer: Buffer): SharedArrayBuffer => {
    const sab = new SharedArrayBuffer(buffer.length);
    const tmp = new Uint8Array(sab);
    tmp.set(buffer, 0);
    return sab;
};

/**
 * Load wasm file from the server. Only works in browsers.
 * The function will try load file into SharedArrayBuffer if the site is cross-origin isolated.
 * Otherwise the return value fallbacks to Buffer which is less performant but is still compatible with FluenceAppService methods.
 * We strongly recommend to set-up cross-origin headers. For more details see: See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements
 * Filename is relative to current origin.
 * @param filePath - path to the wasm file relative to current origin
 * @returns Either SharedArrayBuffer or Buffer with the wasm file
 */
export const loadWasmFromServer = async (filePath: string): Promise<SharedArrayBuffer | Buffer> => {
    if (!isBrowser) {
        throw new Error('Files can be loaded from url only in browser environment');
    }

    const fullUrl = window.location.origin + '/' + filePath;
    const res = await fetch(fullUrl);
    const ab = await res.arrayBuffer();
    new Uint8Array(ab);
    const buffer = Buffer.from(ab);

    // only convert to shared buffers if necessary CORS headers have been set:
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements
    if (eval('crossOriginIsolated')) {
        return bufferToSharedArrayBuffer(buffer);
    }

    return buffer;
};

/**
 * Load wasm file from npm package. Only works in nodejs environment.
 * The function returns SharedArrayBuffer compatible with FluenceAppService methods.
 * @param source - object specifying the source of the file. Consist two fields: package name and file path.
 * @returns SharedArrayBuffer with the wasm file
 */
export const loadWasmFromNpmPackage = async (source: { package: string; file: string }): Promise<SharedArrayBuffer> => {
    if (!isNode) {
        throw new Error('Files can be loaded from npm packages only in nodejs environment');
    }

    // eval('require') is needed so that
    // webpack will complain about missing dependencies for web target
    const r = eval('require');
    const path = r('path');
    const fs = r('fs').promises;
    const packagePath = r.resolve(source.package);
    const filePath = path.join(path.dirname(packagePath), source.file);
    const buffer = await fs.readFile(filePath);
    return bufferToSharedArrayBuffer(buffer);
};

/**
 * Load wasm file from the file system. Only works in nodejs environment.
 * The functions returns SharedArrayBuffer compatible with FluenceAppService methods.
 * @param filePath - path to the wasm file
 * @returns SharedArrayBuffer with the wasm file
 */
export const loadWasmFromFileSystem = async (filePath: string): Promise<SharedArrayBuffer> => {
    if (!isNode) {
        throw new Error('Files can be loaded from file system only in nodejs environment');
    }

    // eval('require') is needed so that
    // webpack will complain about missing dependencies for web target
    const r = eval('require');
    const fs = r('fs').promises;
    const buffer = await fs.readFile(filePath);
    return bufferToSharedArrayBuffer(buffer);
};

/**
 * Load wasm marine control module and avm module required for marine-js to work from the default source.
 * Both modules are loaded in the format compatible with FluenceAppService methods.
 * If called from the nodejs environment files are loaded from corresponding npm packages.
 * If called inside browser files are loaded from the server.
 * @returns Object with two fields: "marine" and "avm" corresponding to control module and avm module
 */
export const loadDefaults = async (): Promise<{
    marine: SharedArrayBuffer | Buffer;
    avm: SharedArrayBuffer | Buffer;
}> => {
    let promises;
    // check if we are running inside the browser and instantiate worker with the corresponding script
    if (isBrowser) {
        promises = [
            // force new line
            loadWasmFromServer(defaultNames.marine.file),
            loadWasmFromServer(defaultNames.avm.file),
        ];
    } else if (isNode) {
        promises = [
            // force new line
            loadWasmFromNpmPackage(defaultNames.marine),
            loadWasmFromNpmPackage(defaultNames.avm),
        ];
    } else {
        throw new Error('Unknown environment');
    }

    const [marine, avm] = await Promise.all(promises);
    return {
        marine,
        avm,
    };
};
