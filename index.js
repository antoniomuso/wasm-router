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

let route = new RouterWrapper();
route.insert("Get", '/ciao/bla', () => {
    console.log('bla');
});

route.insert("Get", '/ciao', () => {
    console.log('ciao');
});

route.insert("Get", '/ciao/bl', () => {
    console.log("bl");
});

route.insert("Get", '/c/fratm', () => {
    console.log("fratm");
});

route.lookup('Get', '/c/fratm');
route.lookup('Get', '/ciao/bl');
route.lookup('Get', '/ciao');
route.lookup('Get', '/ciao/bla');



