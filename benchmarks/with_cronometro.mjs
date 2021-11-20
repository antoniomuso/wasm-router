import cronometro from 'cronometro';

import Router from '../index.js';

const routerWasm = new Router();
import FindMyWay from 'find-my-way';
const router = FindMyWay();

const routes = [
  { method: 'GET', url: '/user' },
  { method: 'GET', url: '/user/comments' },
  { method: 'GET', url: '/user/avatar' },
  { method: 'GET', url: '/user/lookup/username/:username' },
  { method: 'GET', url: '/user/lookup/email/:address' },
  { method: 'GET', url: '/event/:id' },
  { method: 'GET', url: '/event/:id/comments' },
  { method: 'POST', url: '/event/:id/comment' },
  { method: 'GET', url: '/map/:location/events' },
  { method: 'GET', url: '/status' },
  { method: 'GET', url: '/very/deeply/nested/route/hello/there' },
  { method: 'GET', url: '/static/*' }
]

function noop () {}

routes.forEach(route => {
    routerWasm.insert(route.method, route.url, noop)
})

routes.forEach(route => {
    router.on(route.method, route.url, noop)
})

routerWasm.lookup('GET', '/very/deeply/nested/route/hello/there');
  
cronometro({
    wasmLong: () => {
        routerWasm.lookup('GET', '/very/deeply/nested/route/hello/there');
    },
    findMyWayLong: () => {
        router.find('GET', '/very/deeply/nested/route/hello/there');
    },
    wasmShort: () => {
        routerWasm.lookup('GET', '/user');
    },
    findMyWayShort: () => {
        router.find('GET', '/user');
    }
}, {print: {compare: true}});
