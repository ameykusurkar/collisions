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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
/**
*/
export const CollisionAlgorithm = Object.freeze({ Pairwise:0,"0":"Pairwise",SweepAndPrune:1,"1":"SweepAndPrune", });

const ColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_color_free(ptr >>> 0));
/**
*/
export class Color {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ColorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_color_free(ptr);
    }
    /**
    * @returns {number}
    */
    get 0() {
        const ret = wasm.__wbg_get_color_0(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set 0(arg0) {
        wasm.__wbg_set_color_0(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get 1() {
        const ret = wasm.__wbg_get_color_1(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set 1(arg0) {
        wasm.__wbg_set_color_1(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get 2() {
        const ret = wasm.__wbg_get_color_2(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set 2(arg0) {
        wasm.__wbg_set_color_2(this.__wbg_ptr, arg0);
    }
}

const ParticleFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_particle_free(ptr >>> 0));
/**
*/
export class Particle {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Particle.prototype);
        obj.__wbg_ptr = ptr;
        ParticleFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ParticleFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_particle_free(ptr);
    }
    /**
    * @returns {Vec2}
    */
    get pos() {
        const ret = wasm.__wbg_get_particle_pos(this.__wbg_ptr);
        return Vec2.__wrap(ret);
    }
    /**
    * @param {Vec2} arg0
    */
    set pos(arg0) {
        _assertClass(arg0, Vec2);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_pos(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {Vec2}
    */
    get vel() {
        const ret = wasm.__wbg_get_particle_vel(this.__wbg_ptr);
        return Vec2.__wrap(ret);
    }
    /**
    * @param {Vec2} arg0
    */
    set vel(arg0) {
        _assertClass(arg0, Vec2);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_particle_vel(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {number}
    */
    get radius() {
        const ret = wasm.__wbg_get_particle_radius(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set radius(arg0) {
        wasm.__wbg_set_particle_radius(this.__wbg_ptr, arg0);
    }
    /**
    * @param {Vec2} pos
    * @param {Vec2} vel
    * @param {number} radius
    * @returns {Particle}
    */
    static new(pos, vel, radius) {
        _assertClass(pos, Vec2);
        var ptr0 = pos.__destroy_into_raw();
        _assertClass(vel, Vec2);
        var ptr1 = vel.__destroy_into_raw();
        const ret = wasm.particle_new(ptr0, ptr1, radius);
        return Particle.__wrap(ret);
    }
    /**
    * @param {Vec2} p
    * @returns {boolean}
    */
    contains(p) {
        _assertClass(p, Vec2);
        var ptr0 = p.__destroy_into_raw();
        const ret = wasm.particle_contains(this.__wbg_ptr, ptr0);
        return ret !== 0;
    }
}

const Vec2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vec2_free(ptr >>> 0));
/**
*/
export class Vec2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Vec2.prototype);
        obj.__wbg_ptr = ptr;
        Vec2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Vec2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vec2_free(ptr);
    }
    /**
    * @returns {number}
    */
    get 0() {
        const ret = wasm.__wbg_get_vec2_0(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set 0(arg0) {
        wasm.__wbg_set_vec2_0(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get 1() {
        const ret = wasm.__wbg_get_vec2_1(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set 1(arg0) {
        wasm.__wbg_set_vec2_1(this.__wbg_ptr, arg0);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @returns {Vec2}
    */
    static new(x, y) {
        const ret = wasm.vec2_new(x, y);
        return Vec2.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    x() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.vec2_x(ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    y() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.vec2_y(ptr);
        return ret;
    }
}

const WorldFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_world_free(ptr >>> 0));
/**
*/
export class World {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(World.prototype);
        obj.__wbg_ptr = ptr;
        WorldFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WorldFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_world_free(ptr);
    }
    /**
    * @param {number} width
    * @param {number} height
    * @returns {World}
    */
    static new(width, height) {
        const ret = wasm.world_new(width, height);
        return World.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    momentum() {
        const ret = wasm.world_momentum(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    num_particles() {
        const ret = wasm.world_num_particles(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    particles() {
        const ret = wasm.world_particles(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    colors() {
        const ret = wasm.world_colors(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {Vec2} start
    * @param {Vec2} end
    */
    push_segment(start, end) {
        _assertClass(start, Vec2);
        var ptr0 = start.__destroy_into_raw();
        _assertClass(end, Vec2);
        var ptr1 = end.__destroy_into_raw();
        wasm.world_push_segment(this.__wbg_ptr, ptr0, ptr1);
    }
    /**
    * Adds the particle to the world if the space is unoccupied.
    * @param {Particle} particle
    * @returns {boolean}
    */
    try_push(particle) {
        _assertClass(particle, Particle);
        var ptr0 = particle.__destroy_into_raw();
        const ret = wasm.world_try_push(this.__wbg_ptr, ptr0);
        return ret !== 0;
    }
    /**
    * @param {number} dt
    * @param {number} drag
    * @param {number} steps
    * @param {CollisionAlgorithm} alg
    * @returns {number}
    */
    step_frame(dt, drag, steps, alg) {
        const ret = wasm.world_step_frame(this.__wbg_ptr, dt, drag, steps, alg);
        return ret >>> 0;
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
        input = new URL('collisions_bg.wasm', import.meta.url);
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
