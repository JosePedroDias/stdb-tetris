export const K_UP = 'ArrowUp';
export const K_DOWN = 'ArrowDown';
export const K_LEFT = 'ArrowLeft';
export const K_RIGHT = 'ArrowRight';
export const K_Z = 'z';
export const K_X = 'x';
export const K_SPACE_BAR = ' ';

const relevant = new Set([K_UP, K_SPACE_BAR, K_DOWN, K_LEFT, K_RIGHT, K_Z, K_X]);

let onKeyFn: Function = () => {};

export function onKey(fn: Function) {
    onKeyFn = fn;
}

function onKeyFactory(isDown: boolean) {
    return function(ev: KeyboardEvent) {
        //console.log(ev.key, isDown);
        if (ev.altKey || ev.metaKey || ev.ctrlKey || !relevant.has(ev.key)) return;
        ev.preventDefault();
        ev.stopPropagation();
        onKeyFn(ev.key, isDown);
    }
}

export function isRotateLeft(key: string) {
    return key === K_Z;
}
export function isRotateRight(key: string) {
    return key === K_X;
}

export function isMoveLeft(key: string) {
    return key === K_LEFT;
}
export function isMoveRight(key: string) {
    return key === K_RIGHT;
}

export function isMoveDown(key: string) {
    return key === K_DOWN;
}
export function isDrop(key: string) {
    return key === K_SPACE_BAR || key === K_UP;
}

window.addEventListener('keydown', onKeyFactory(true));
window.addEventListener('keyup',   onKeyFactory(false));
