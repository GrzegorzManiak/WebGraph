
import init from './wasm/mandelbrot';
init().then(() => main());

function main() {
    console.info('WASM Bindings loaded');
}
