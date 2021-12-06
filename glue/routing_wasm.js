let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextDecoder, TextEncoder } = require(`util`);
const QuickLRU = require('quick-lru');

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

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

// We can try using the same memory location to pass the strings.
const cache = new QuickLRU({ maxSize: 100 });
function passStringToWasm0(arg, malloc) {
    let data = cache.get(arg);
    if (!data) {
        let offset = 0;
        let len = arg.length;

        data = new Uint8Array(len);
        for (; offset < len; offset++) {
            const char = arg.charCodeAt(offset);
            if (char > 0x7F) {
                break;
            }
            data[offset] = arg.charCodeAt(offset);
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            newData = new Uint8Array(len = offset + arg.length * 3);
            newData.set(data, 0);

            const view = newData.subarray(offset, len);
            const ret = encodeString(arg, view);
    
            offset += ret.written;
            data = newData.subarray(0, offset)
        }
        cache.set(arg, data);
    }

    const len = data.length;
    const ptr = malloc(len);

    const mem = getUint8Memory0();
    mem.set(data, ptr)

    WASM_VECTOR_LEN = len;
    return ptr;
}
/**
*/
module.exports.greet = function() {
    wasm.greet();
};

/**
*/
module.exports.Method = Object.freeze({ ACL:0,"0":"ACL",BIND:1,"1":"BIND",CHECKOUT:2,"2":"CHECKOUT",CONNECT:3,"3":"CONNECT",COPY:4,"4":"COPY",DELETE:5,"5":"DELETE",GET:6,"6":"GET",HEAD:7,"7":"HEAD",LINK:8,"8":"LINK",LOCK:9,"9":"LOCK",MSEARCH:10,"10":"MSEARCH",MERGE:11,"11":"MERGE",MKACTIVITY:12,"12":"MKACTIVITY",MKCALENDAR:13,"13":"MKCALENDAR",MKCOL:14,"14":"MKCOL",MOVE:15,"15":"MOVE",NOTIFY:16,"16":"NOTIFY",OPTIONS:17,"17":"OPTIONS",PATCH:18,"18":"PATCH",POST:19,"19":"POST",PRI:20,"20":"PRI",PROPFIND:21,"21":"PROPFIND",PROPPATCH:22,"22":"PROPPATCH",PURGE:23,"23":"PURGE",PUT:24,"24":"PUT",REBIND:25,"25":"REBIND",REPORT:26,"26":"REPORT",SEARCH:27,"27":"SEARCH",SOURCE:28,"28":"SOURCE",SUBSCRIBE:29,"29":"SUBSCRIBE",TRACE:30,"30":"TRACE",UNBIND:31,"31":"UNBIND",UNLINK:32,"32":"UNLINK",UNLOCK:33,"33":"UNLOCK",UNSUBSCRIBE:34,"34":"UNSUBSCRIBE", });
/**
*/
class Router {

    static __wrap(ptr) {
        const obj = Object.create(Router.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_router_free(ptr);
    }
    /**
    */
    constructor() {
        var ret = wasm.router_new();
        return Router.__wrap(ret);
    }
    /**
    * @param {string} default_route
    */
    set_default_route(default_route) {
        var ptr0 = passStringToWasm0(default_route, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.router_set_default_route(this.ptr, ptr0, len0);
    }
    /**
    * @param {string} on_bad_url
    */
    set_on_bad_url(on_bad_url) {
        var ptr0 = passStringToWasm0(on_bad_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.router_set_on_bad_url(this.ptr, ptr0, len0);
    }
    /**
    * @param {number} method
    * @param {string} route
    * @returns {number}
    */
    lookup(method, route) {
        var ptr0 = passStringToWasm0(route, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.router_lookup(this.ptr, method, ptr0, len0);
        return ret >>> 0;
    }
    /**
    * @param {number} method
    * @param {string} path
    * @param {number} func
    * @returns {any}
    */
    insert(method, path, func) {
        var ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.router_insert(this.ptr, method, ptr0, len0, func);
        return takeObject(ret);
    }
    /**
    * @param {number} method
    * @param {string} path
    * @param {number} handler
    * @returns {boolean}
    */
    on(method, path, handler) {
        var ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.router_on(this.ptr, method, ptr0, len0, handler);
        return ret !== 0;
    }
    /**
    * @param {number} method
    * @param {string} path
    * @param {number} handler
    * @returns {boolean}
    */
    iter_on(method, path, handler) {
        var ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.router_iter_on(this.ptr, method, ptr0, len0, handler);
        return ret !== 0;
    }
}
module.exports.Router = Router;

module.exports.__wbg_log_a7aa4e2ab9ce4941 = function(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

module.exports.__wbg_alert_72f9e77272cf312f = function(arg0, arg1) {
    alert(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};

const path = require('path').join(`${__dirname}/../pkg/`, 'routing_wasm_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

