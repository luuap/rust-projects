import * as fs from 'fs';

/**
 * Replaces CanvasRenderingContext2D with Offscreen variant in in glue code 
 * Workaround for https://github.com/rustwasm/wasm-bindgen/issues/1614
 * Call after wasm-pack build: node bugfix.mjs
 */
function bugfix() {
  const file = fs.readFileSync('pkg/wasm_demos_bg.js', 'utf-8');
  const fixed = file.replace('instanceof CanvasRenderingContext2D', 'instanceof OffscreenCanvasRenderingContext2D');
  fs.writeFileSync('pkg/wasm_demos_bg.js', fixed, 'utf-8');
  console.log('Bugfix complete');
}

bugfix();