let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* Represents a number in standard form.
*
* The `Standardform` struct holds the significand (mantissa) of the number
* and an exponent that determines the power of 10 by which the significand should be multiplied.
*/
export class StandardForm {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(StandardForm.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_standardform_free(ptr);
    }
    /**
    * Creates a new instance of `StandardForm` with the given mantissa and exponent.
    *
    * This constructor initializes a new `StandardForm` instance with the provided `mantissa` and `exponent`.
    * It's important to note that the provided `mantissa` and `exponent` may not be exactly the same as the
    * values stored in the resulting instance. The values are adjusted automatically to adhere to the rules
    * of standard form representation, ensuring the most appropriate form for the given input.
    *
    *  ## Rules :
    * If the current mantissa and exponent do not satisfy the standard form representation requirements,
    * this method will adjust them while maintaining the value of the number represented. The adjustment
    * ensures that the mantissa is between 1 (inclusive) and 10 (exclusive) and the exponent is such that
    * the product of mantissa and 10 raised to the exponent yields the original number.
    * @param {number} mantissa
    * @param {number} exponent
    */
    constructor(mantissa, exponent) {
        const ret = wasm.standardform_new(mantissa, exponent);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * Converts string into `StandardFrom` as traits cannot be 'bridged'
    * @param {string} string
    * @returns {StandardForm}
    */
    static new_from_string(string) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            wasm.standardform_new_from_string(retptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return StandardForm.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Converts `StandardFrom` into f64 as traits cannot be 'bridged'
    * @returns {number}
    */
    into_f64() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_into_f64(ptr);
        return ret;
    }
    /**
    * Returns the string representation of the number in scientific notation.
    * @returns {string}
    */
    to_scientific_notation() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.standardform_to_scientific_notation(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * Returns the string representation of the number in engineering notation.
    * @returns {string}
    */
    to_engineering_notation() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.standardform_to_engineering_notation(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    *r" Computes the sine of a number (in radians).
    * @returns {StandardForm}
    */
    sin() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_sin(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the cosine of a number (in radians).
    * @returns {StandardForm}
    */
    cos() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_cos(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the tangent of a number (in radians).
    * @returns {StandardForm}
    */
    tan() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_tan(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the arcsine of a number.
    * @returns {StandardForm}
    */
    asin() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_asin(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the arccosine of a number.
    * @returns {StandardForm}
    */
    acos() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_acos(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the arctangent of a number.
    * @returns {StandardForm}
    */
    atan() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_atan(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the hyperbolic sine.
    * @returns {StandardForm}
    */
    sinh() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_sinh(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the hyperbolic cosine.
    * @returns {StandardForm}
    */
    cosh() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_cosh(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the hyperbolic tangent.
    * @returns {StandardForm}
    */
    tanh() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_tanh(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the inverse hyperbolic sine.
    * @returns {StandardForm}
    */
    asinh() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_asinh(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the inverse hyperbolic cosine.
    * @returns {StandardForm}
    */
    acosh() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_acosh(ptr);
        return StandardForm.__wrap(ret);
    }
    /**
    *r" Computes the inverse hyperbolic tangent.
    * @returns {StandardForm}
    */
    atanh() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.standardform_atanh(ptr);
        return StandardForm.__wrap(ret);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('standardform_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
