import { isBrowser, isNode } from 'browser-or-node';
import { defaultNames } from '.';

const bufferToSharedArrayBuffer = (buffer: Buffer): SharedArrayBuffer | Buffer => {
    // only convert to shared buffers if necessary CORS headers have been set:
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements
    if (isBrowser && eval('crossOriginIsolated')) {
        const sab = new SharedArrayBuffer(buffer.length);
        const tmp = new Uint8Array(sab);
        tmp.set(buffer, 0);
        return sab;
    } else {
        return buffer;
    }
};

export const loadWasmFromServer = async (fileName: string): Promise<SharedArrayBuffer | Buffer> => {
    if (!isBrowser) {
        throw new Error('Files can be loaded from url only in browser environment');
    }

    const fullUrl = window.location.origin + '/' + fileName;
    const res = await fetch(fullUrl);
    const ab = await res.arrayBuffer();
    new Uint8Array(ab);
    const buffer = Buffer.from(ab);
    return bufferToSharedArrayBuffer(buffer);
};

export const loadWasmFromNpmPackage = async (source: {
    package: string;
    file: string;
}): Promise<SharedArrayBuffer | Buffer> => {
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

export const loadWasmFromFileSystem = async (filePath: string): Promise<SharedArrayBuffer | Buffer> => {
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
