/* tslint:disable */
/* eslint-disable */
/**
*/
export class Color {
  free(): void;
/**
*/
  0: number;
/**
*/
  1: number;
/**
*/
  2: number;
}
/**
*/
export class Particle {
  free(): void;
/**
* @param {Vec2} pos
* @param {Vec2} vel
* @param {number} radius
* @returns {Particle}
*/
  static new(pos: Vec2, vel: Vec2, radius: number): Particle;
/**
* @param {Vec2} p
* @returns {boolean}
*/
  contains(p: Vec2): boolean;
/**
*/
  pos: Vec2;
/**
*/
  radius: number;
/**
*/
  vel: Vec2;
}
/**
*/
export class Vec2 {
  free(): void;
/**
* @param {number} x
* @param {number} y
* @returns {Vec2}
*/
  static new(x: number, y: number): Vec2;
/**
* @returns {number}
*/
  x(): number;
/**
* @returns {number}
*/
  y(): number;
/**
*/
  0: number;
/**
*/
  1: number;
}
/**
*/
export class World {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @returns {World}
*/
  static new(width: number, height: number): World;
/**
* @returns {number}
*/
  momentum(): number;
/**
* @returns {number}
*/
  num_particles(): number;
/**
* @returns {number}
*/
  particles(): number;
/**
* @returns {number}
*/
  colors(): number;
/**
* Adds the particle to the world if the space is unoccupied.
* @param {Particle} particle
* @returns {boolean}
*/
  try_push(particle: Particle): boolean;
/**
* @param {number} dt
* @param {number} drag
*/
  step_frame(dt: number, drag: number): void;
/**
* @param {number} dt
* @param {number} drag
*/
  step_dt(dt: number, drag: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_color_free: (a: number) => void;
  readonly __wbg_get_color_0: (a: number) => number;
  readonly __wbg_set_color_0: (a: number, b: number) => void;
  readonly __wbg_get_color_1: (a: number) => number;
  readonly __wbg_set_color_1: (a: number, b: number) => void;
  readonly __wbg_get_color_2: (a: number) => number;
  readonly __wbg_set_color_2: (a: number, b: number) => void;
  readonly __wbg_particle_free: (a: number) => void;
  readonly __wbg_get_particle_pos: (a: number) => number;
  readonly __wbg_set_particle_pos: (a: number, b: number) => void;
  readonly __wbg_get_particle_vel: (a: number) => number;
  readonly __wbg_set_particle_vel: (a: number, b: number) => void;
  readonly __wbg_get_particle_radius: (a: number) => number;
  readonly __wbg_set_particle_radius: (a: number, b: number) => void;
  readonly particle_new: (a: number, b: number, c: number) => number;
  readonly particle_contains: (a: number, b: number) => number;
  readonly __wbg_world_free: (a: number) => void;
  readonly world_new: (a: number, b: number) => number;
  readonly world_momentum: (a: number) => number;
  readonly world_num_particles: (a: number) => number;
  readonly world_particles: (a: number) => number;
  readonly world_colors: (a: number) => number;
  readonly world_try_push: (a: number, b: number) => number;
  readonly world_step_frame: (a: number, b: number, c: number) => void;
  readonly world_step_dt: (a: number, b: number, c: number) => void;
  readonly __wbg_vec2_free: (a: number) => void;
  readonly __wbg_get_vec2_0: (a: number) => number;
  readonly __wbg_set_vec2_0: (a: number, b: number) => void;
  readonly __wbg_get_vec2_1: (a: number) => number;
  readonly __wbg_set_vec2_1: (a: number, b: number) => void;
  readonly vec2_new: (a: number, b: number) => number;
  readonly vec2_x: (a: number) => number;
  readonly vec2_y: (a: number) => number;
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
