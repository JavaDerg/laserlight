export async function init(game_name: string) {
    window.onbeforeunload = lockEvent;

    const WASM = await import('wasm-laserlight');

    let eb = await WASM.new_engine_builder(game_name);
    let ng = WASM.build_engine(eb);
    WASM.run_engine(ng);
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