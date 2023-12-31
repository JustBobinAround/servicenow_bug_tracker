/* tslint:disable */
/* eslint-disable */
/**
* @param {string} pass
* @returns {Promise<any>}
*/
export function build_bugsub(pass: string): Promise<any>;
/**
* @param {string} pass
* @returns {Promise<any>}
*/
export function autofill_form(pass: string): Promise<any>;
/**
* @param {string} pass
* @param {string} id
* @returns {Promise<any>}
*/
export function get_bug_report(pass: string, id: string): Promise<any>;
/**
* @param {string} pass
* @returns {Promise<any>}
*/
export function fill_table(pass: string): Promise<any>;
/**
* @param {string} pass
* @returns {boolean}
*/
export function is_passcode_correct(pass: string): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly build_bugsub: (a: number, b: number) => number;
  readonly autofill_form: (a: number, b: number) => number;
  readonly get_bug_report: (a: number, b: number, c: number, d: number) => number;
  readonly fill_table: (a: number, b: number) => number;
  readonly is_passcode_correct: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdf8936c73d7478b1: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h601cc7bef29c97b2: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
