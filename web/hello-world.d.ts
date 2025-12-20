/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__hc5aed486ce1c50a0: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__h16b0a13b51fc6b94: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h5f4115ddd91d94ac: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__haab60af082748731: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__ha2ccc0223ad5a3af: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h4d4cfbcdb6ec607b: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__he4baf4503d0c19ae: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hfdc9129a738b677a: (a: number, b: number, c: any, d: any) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hf7676df9f6c09885: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h92c7d3a8d300a8e5: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
