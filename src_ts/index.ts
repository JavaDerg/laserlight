export async function init() {
    window.onbeforeunload = lockEvent;

    const WASM = await import('wasm-laserlight');

    console.log(typeof WASM, WASM);

    await WASM.new_engine_builder();
}

export function fancyUpDocument(splashMsg: string = "LaserLight is preparing..."): void {
    document.body.setAttribute('style', 'background: black;');

    const div = document.createElement('div');
    div.innerHTML = `<h1>${splashMsg}</h1>`;
    div.setAttribute('style', `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: -1;
        
        display: flex;
        justify-content: center;
        align-items: center;
        
        color: white;
    `);

    document.body.appendChild(div);
}

export function createGameCanvas(): HTMLCanvasElement {
    const canvas = document.createElement('canvas');
    canvas.setAttribute('style', `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
    `);
    document.body.appendChild(canvas);

    return canvas;
}

let lockEvent = function(): string|undefined {
    return undefined;
};

export function lockNavigation(lock: boolean = true) {
    console.log("Page locked:", lock);
    lockEvent = lock ? function() {
        return "";
    } : function() {
        return undefined;
    };

}