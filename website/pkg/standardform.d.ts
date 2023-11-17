/* tslint:disable */
/* eslint-disable */
/**
* Represents a number in standard form.
*
* The `Standardform` struct holds the significand (mantissa) of the number 
* and an exponent that determines the power of 10 by which the significand should be multiplied.
*/
export class StandardForm {
  free(): void;
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
  constructor(mantissa: number, exponent: number);
/**
* Converts string into `StandardFrom` as traits cannot be 'bridged' 
* @param {string} string
* @returns {StandardForm}
*/
  static new_from_string(string: string): StandardForm;
/**
* Converts `StandardFrom` into f64 as traits cannot be 'bridged' 
* @returns {number}
*/
  into_f64(): number;
/**
* Returns the string representation of the number in scientific notation.
* @returns {string}
*/
  to_scientific_notation(): string;
/**
* Returns the string representation of the number in engineering notation.
* @returns {string}
*/
  to_engineering_notation(): string;
/**
*r" Computes the sine of a number (in radians).
* @returns {StandardForm}
*/
  sin(): StandardForm;
/**
*r" Computes the cosine of a number (in radians).
* @returns {StandardForm}
*/
  cos(): StandardForm;
/**
*r" Computes the tangent of a number (in radians).
* @returns {StandardForm}
*/
  tan(): StandardForm;
/**
*r" Computes the arcsine of a number.
* @returns {StandardForm}
*/
  asin(): StandardForm;
/**
*r" Computes the arccosine of a number.
* @returns {StandardForm}
*/
  acos(): StandardForm;
/**
*r" Computes the arctangent of a number.
* @returns {StandardForm}
*/
  atan(): StandardForm;
/**
*r" Computes the hyperbolic sine.
* @returns {StandardForm}
*/
  sinh(): StandardForm;
/**
*r" Computes the hyperbolic cosine.
* @returns {StandardForm}
*/
  cosh(): StandardForm;
/**
*r" Computes the hyperbolic tangent.
* @returns {StandardForm}
*/
  tanh(): StandardForm;
/**
*r" Computes the inverse hyperbolic sine.
* @returns {StandardForm}
*/
  asinh(): StandardForm;
/**
*r" Computes the inverse hyperbolic cosine.
* @returns {StandardForm}
*/
  acosh(): StandardForm;
/**
*r" Computes the inverse hyperbolic tangent.
* @returns {StandardForm}
*/
  atanh(): StandardForm;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_standardform_free: (a: number) => void;
  readonly standardform_new: (a: number, b: number) => number;
  readonly standardform_new_from_string: (a: number, b: number, c: number) => void;
  readonly standardform_into_f64: (a: number) => number;
  readonly standardform_to_scientific_notation: (a: number, b: number) => void;
  readonly standardform_to_engineering_notation: (a: number, b: number) => void;
  readonly standardform_sin: (a: number) => number;
  readonly standardform_cos: (a: number) => number;
  readonly standardform_tan: (a: number) => number;
  readonly standardform_asin: (a: number) => number;
  readonly standardform_acos: (a: number) => number;
  readonly standardform_atan: (a: number) => number;
  readonly standardform_sinh: (a: number) => number;
  readonly standardform_cosh: (a: number) => number;
  readonly standardform_tanh: (a: number) => number;
  readonly standardform_asinh: (a: number) => number;
  readonly standardform_acosh: (a: number) => number;
  readonly standardform_atanh: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
