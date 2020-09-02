
export async function init() {
    await import('./../pkg');
    const WASM = await import('wasm-laserlight');
    WASM.init();
}

export function createGameCanvas(): HTMLCanvasElement {
    const canvas = document.createElement('canvas');
    canvas.setAttribute('style', `
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: black;
    `);
    document.querySelector('body').appendChild(canvas);

    return canvas;
}
