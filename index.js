const wasm = require('./glue/routing_wasm')


class RouterWrapper {
    constructor() {
        this.router = new wasm.Router();
        this.handlers = [];
    }   
    
    /**
     * 
     * @param {string} method 
     * @param {string} path
     * @param {function} handler
     */
    on (method, path, handler) {
        if (!method ||  !Object.prototype.hasOwnProperty.call(wasm.Method, method.toUpperCase())) {
                throw new Error('Wrong method parameter');
        }
        if (!path) throw new Error('Path is empty');
        if (typeof handler !== 'function') throw new Error('handler is not a function');

        let out = this.router.on(wasm.Method[method.toUpperCase()], path, this.handlers.length);

        if (!out) {
            throw new Error('Insertion error')
        }

        this.handlers.push(handler);
    }

    insert (method, path, handler) {
        if (!method ||  !Object.prototype.hasOwnProperty.call(wasm.Method, method.toUpperCase())) {
                throw new Error('Wrong method parameter');
        }
        if (!path) throw new Error('Path is empty');
        if (typeof handler !== 'function') throw new Error('handler is not a function');

        let out = this.router.insert(wasm.Method[method.toUpperCase()], path, this.handlers.length);

        this.handlers.push(handler);
    }
    
    lookup (method, path) {
        if (!method ||  !Object.prototype.hasOwnProperty.call(wasm.Method, method.toUpperCase())) {
            throw new Error('Wrong method parameter');
        }
        if (!path) throw new Error('Path is empty');
        let out = this.router.lookup(wasm.Method[method.toUpperCase()], path);
        if (out >= 0) {
            this.handlers[out]();
        }
    }
}

module.exports = RouterWrapper;


