const wasm = require('./pkg/routing_wasm')


class RouterWrapper {
    constructor() {
        this.router = wasm.Router.new();
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

        if (out < 0) {
            throw new Error('Insertion error')
        }

        this.handlers.push(handler);
    }
    
}

new RouterWrapper().on("Get");
