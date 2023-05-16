/*! For license information please see mirror-tell-section.js.LICENSE.txt */
(()=>{"use strict";var t,e={246:(t,e,n)=>{function r(){r=function(){return t};var t={},e=Object.prototype,n=e.hasOwnProperty,o=Object.defineProperty||function(t,e,n){t[e]=n.value},i="function"==typeof Symbol?Symbol:{},a=i.iterator||"@@iterator",c=i.asyncIterator||"@@asyncIterator",u=i.toStringTag||"@@toStringTag";function f(t,e,n){return Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}),t[e]}try{f({},"")}catch(t){f=function(t,e,n){return t[e]=n}}function s(t,e,n,r){var i=e&&e.prototype instanceof g?e:g,a=Object.create(i.prototype),c=new S(r||[]);return o(a,"_invoke",{value:E(t,n,c)}),a}function l(t,e,n){try{return{type:"normal",arg:t.call(e,n)}}catch(t){return{type:"throw",arg:t}}}t.wrap=s;var w={};function g(){}function b(){}function d(){}var p={};f(p,a,(function(){return this}));var h=Object.getPrototypeOf,y=h&&h(h(O([])));y&&y!==e&&n.call(y,a)&&(p=y);var v=d.prototype=g.prototype=Object.create(p);function m(t){["next","throw","return"].forEach((function(e){f(t,e,(function(t){return this._invoke(e,t)}))}))}function x(t,e){function r(o,i,a,c){var u=l(t[o],t,i);if("throw"!==u.type){var f=u.arg,s=f.value;return s&&"object"==_(s)&&n.call(s,"__await")?e.resolve(s.__await).then((function(t){r("next",t,a,c)}),(function(t){r("throw",t,a,c)})):e.resolve(s).then((function(t){f.value=t,a(f)}),(function(t){return r("throw",t,a,c)}))}c(u.arg)}var i;o(this,"_invoke",{value:function(t,n){function o(){return new e((function(e,o){r(t,n,e,o)}))}return i=i?i.then(o,o):o()}})}function E(t,e,n){var r="suspendedStart";return function(o,i){if("executing"===r)throw new Error("Generator is already running");if("completed"===r){if("throw"===o)throw i;return A()}for(n.method=o,n.arg=i;;){var a=n.delegate;if(a){var c=k(a,n);if(c){if(c===w)continue;return c}}if("next"===n.method)n.sent=n._sent=n.arg;else if("throw"===n.method){if("suspendedStart"===r)throw r="completed",n.arg;n.dispatchException(n.arg)}else"return"===n.method&&n.abrupt("return",n.arg);r="executing";var u=l(t,e,n);if("normal"===u.type){if(r=n.done?"completed":"suspendedYield",u.arg===w)continue;return{value:u.arg,done:n.done}}"throw"===u.type&&(r="completed",n.method="throw",n.arg=u.arg)}}}function k(t,e){var n=t.iterator[e.method];if(void 0===n){if(e.delegate=null,"throw"===e.method){if(t.iterator.return&&(e.method="return",e.arg=void 0,k(t,e),"throw"===e.method))return w;e.method="throw",e.arg=new TypeError("The iterator does not provide a 'throw' method")}return w}var r=l(n,t.iterator,e.arg);if("throw"===r.type)return e.method="throw",e.arg=r.arg,e.delegate=null,w;var o=r.arg;return o?o.done?(e[t.resultName]=o.value,e.next=t.nextLoc,"return"!==e.method&&(e.method="next",e.arg=void 0),e.delegate=null,w):o:(e.method="throw",e.arg=new TypeError("iterator result is not an object"),e.delegate=null,w)}function L(t){var e={tryLoc:t[0]};1 in t&&(e.catchLoc=t[1]),2 in t&&(e.finallyLoc=t[2],e.afterLoc=t[3]),this.tryEntries.push(e)}function j(t){var e=t.completion||{};e.type="normal",delete e.arg,t.completion=e}function S(t){this.tryEntries=[{tryLoc:"root"}],t.forEach(L,this),this.reset(!0)}function O(t){if(t){var e=t[a];if(e)return e.call(t);if("function"==typeof t.next)return t;if(!isNaN(t.length)){var r=-1,o=function e(){for(;++r<t.length;)if(n.call(t,r))return e.value=t[r],e.done=!1,e;return e.value=void 0,e.done=!0,e};return o.next=o}}return{next:A}}function A(){return{value:void 0,done:!0}}return b.prototype=d,o(v,"constructor",{value:d,configurable:!0}),o(d,"constructor",{value:b,configurable:!0}),b.displayName=f(d,u,"GeneratorFunction"),t.isGeneratorFunction=function(t){var e="function"==typeof t&&t.constructor;return!!e&&(e===b||"GeneratorFunction"===(e.displayName||e.name))},t.mark=function(t){return Object.setPrototypeOf?Object.setPrototypeOf(t,d):(t.__proto__=d,f(t,u,"GeneratorFunction")),t.prototype=Object.create(v),t},t.awrap=function(t){return{__await:t}},m(x.prototype),f(x.prototype,c,(function(){return this})),t.AsyncIterator=x,t.async=function(e,n,r,o,i){void 0===i&&(i=Promise);var a=new x(s(e,n,r,o),i);return t.isGeneratorFunction(n)?a:a.next().then((function(t){return t.done?t.value:a.next()}))},m(v),f(v,u,"Generator"),f(v,a,(function(){return this})),f(v,"toString",(function(){return"[object Generator]"})),t.keys=function(t){var e=Object(t),n=[];for(var r in e)n.push(r);return n.reverse(),function t(){for(;n.length;){var r=n.pop();if(r in e)return t.value=r,t.done=!1,t}return t.done=!0,t}},t.values=O,S.prototype={constructor:S,reset:function(t){if(this.prev=0,this.next=0,this.sent=this._sent=void 0,this.done=!1,this.delegate=null,this.method="next",this.arg=void 0,this.tryEntries.forEach(j),!t)for(var e in this)"t"===e.charAt(0)&&n.call(this,e)&&!isNaN(+e.slice(1))&&(this[e]=void 0)},stop:function(){this.done=!0;var t=this.tryEntries[0].completion;if("throw"===t.type)throw t.arg;return this.rval},dispatchException:function(t){if(this.done)throw t;var e=this;function r(n,r){return a.type="throw",a.arg=t,e.next=n,r&&(e.method="next",e.arg=void 0),!!r}for(var o=this.tryEntries.length-1;o>=0;--o){var i=this.tryEntries[o],a=i.completion;if("root"===i.tryLoc)return r("end");if(i.tryLoc<=this.prev){var c=n.call(i,"catchLoc"),u=n.call(i,"finallyLoc");if(c&&u){if(this.prev<i.catchLoc)return r(i.catchLoc,!0);if(this.prev<i.finallyLoc)return r(i.finallyLoc)}else if(c){if(this.prev<i.catchLoc)return r(i.catchLoc,!0)}else{if(!u)throw new Error("try statement without catch or finally");if(this.prev<i.finallyLoc)return r(i.finallyLoc)}}}},abrupt:function(t,e){for(var r=this.tryEntries.length-1;r>=0;--r){var o=this.tryEntries[r];if(o.tryLoc<=this.prev&&n.call(o,"finallyLoc")&&this.prev<o.finallyLoc){var i=o;break}}i&&("break"===t||"continue"===t)&&i.tryLoc<=e&&e<=i.finallyLoc&&(i=null);var a=i?i.completion:{};return a.type=t,a.arg=e,i?(this.method="next",this.next=i.finallyLoc,w):this.complete(a)},complete:function(t,e){if("throw"===t.type)throw t.arg;return"break"===t.type||"continue"===t.type?this.next=t.arg:"return"===t.type?(this.rval=this.arg=t.arg,this.method="return",this.next="end"):"normal"===t.type&&e&&(this.next=e),w},finish:function(t){for(var e=this.tryEntries.length-1;e>=0;--e){var n=this.tryEntries[e];if(n.finallyLoc===t)return this.complete(n.completion,n.afterLoc),j(n),w}},catch:function(t){for(var e=this.tryEntries.length-1;e>=0;--e){var n=this.tryEntries[e];if(n.tryLoc===t){var r=n.completion;if("throw"===r.type){var o=r.arg;j(n)}return o}}throw new Error("illegal catch attempt")},delegateYield:function(t,e,n){return this.delegate={iterator:O(t),resultName:e,nextLoc:n},"next"===this.method&&(this.arg=void 0),w}},t}function o(t,e,n,r,o,i,a){try{var c=t[i](a),u=c.value}catch(t){return void n(t)}c.done?e(u):Promise.resolve(u).then(r,o)}function i(t){return function(){var e=this,n=arguments;return new Promise((function(r,i){var a=t.apply(e,n);function c(t){o(a,r,i,c,u,"next",t)}function u(t){o(a,r,i,c,u,"throw",t)}c(void 0)}))}}function a(t,e){if(!(t instanceof e))throw new TypeError("Cannot call a class as a function")}function c(t,e){for(var n=0;n<e.length;n++){var r=e[n];r.enumerable=r.enumerable||!1,r.configurable=!0,"value"in r&&(r.writable=!0),Object.defineProperty(t,r.key,r)}}function u(t,e,n){return e&&c(t.prototype,e),n&&c(t,n),Object.defineProperty(t,"prototype",{writable:!1}),t}function _(t){return _="function"==typeof Symbol&&"symbol"==typeof Symbol.iterator?function(t){return typeof t}:function(t){return t&&"function"==typeof Symbol&&t.constructor===Symbol&&t!==Symbol.prototype?"symbol":typeof t},_(t)}var f;n.d(e,{BA:()=>P,E9:()=>F,ZP:()=>B,hz:()=>T,xr:()=>C}),t=n.hmd(t);var s=new Array(128).fill(void 0);function l(t){return s[t]}s.push(void 0,null,!0,!1);var w=s.length;function g(t){var e=l(t);return function(t){t<132||(s[t]=w,w=t)}(t),e}function b(t){w===s.length&&s.push(s.length+1);var e=w;return w=s[e],s[e]=t,e}var d="undefined"!=typeof TextDecoder?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:function(){throw Error("TextDecoder not available")}};"undefined"!=typeof TextDecoder&&d.decode();var p=null;function h(){return null!==p&&0!==p.byteLength||(p=new Uint8Array(f.memory.buffer)),p}function y(t,e){return t>>>=0,d.decode(h().subarray(t,t+e))}function v(t){var e=_(t);if("number"==e||"boolean"==e||null==t)return"".concat(t);if("string"==e)return'"'.concat(t,'"');if("symbol"==e){var n=t.description;return null==n?"Symbol":"Symbol(".concat(n,")")}if("function"==e){var r=t.name;return"string"==typeof r&&r.length>0?"Function(".concat(r,")"):"Function"}if(Array.isArray(t)){var o=t.length,i="[";o>0&&(i+=v(t[0]));for(var a=1;a<o;a++)i+=", "+v(t[a]);return i+="]"}var c,u=/\[object ([^\]]+)\]/.exec(toString.call(t));if(!(u.length>1))return toString.call(t);if("Object"==(c=u[1]))try{return"Object("+JSON.stringify(t)+")"}catch(t){return"Object"}return t instanceof Error?"".concat(t.name,": ").concat(t.message,"\n").concat(t.stack):c}var m=0,x="undefined"!=typeof TextEncoder?new TextEncoder("utf-8"):{encode:function(){throw Error("TextEncoder not available")}},E="function"==typeof x.encodeInto?function(t,e){return x.encodeInto(t,e)}:function(t,e){var n=x.encode(t);return e.set(n),{read:t.length,written:n.length}};function k(t,e,n){if(void 0===n){var r=x.encode(t),o=e(r.length)>>>0;return h().subarray(o,o+r.length).set(r),m=r.length,o}for(var i=t.length,a=e(i)>>>0,c=h(),u=0;u<i;u++){var _=t.charCodeAt(u);if(_>127)break;c[a+u]=_}if(u!==i){0!==u&&(t=t.slice(u)),a=n(a,i,i=u+3*t.length)>>>0;var f=h().subarray(a+u,a+i);u+=E(t,f).written}return m=u,a}var L=null;function j(){return null!==L&&0!==L.byteLength||(L=new Int32Array(f.memory.buffer)),L}function S(t,e){if(!(t instanceof e))throw new Error("expected instance of ".concat(e.name));return t.ptr}function O(t){return null==t}function A(t,e){try{return t.apply(this,e)}catch(t){f.__wbindgen_exn_store(b(t))}}var T=function(){function t(e,n){a(this,t),S(e,F);var r=e.__destroy_into_raw(),o=f.datapoint_new(r,n);return t.__wrap(o)}return u(t,[{key:"__destroy_into_raw",value:function(){var t=this.__wbg_ptr;return this.__wbg_ptr=0,t}},{key:"free",value:function(){var t=this.__destroy_into_raw();f.__wbg_datapoint_free(t)}},{key:"point",get:function(){var t=f.__wbg_get_datapoint_point(this.__wbg_ptr);return F.__wrap(t)},set:function(t){S(t,F);var e=t.__destroy_into_raw();f.__wbg_set_datapoint_point(this.__wbg_ptr,e)}},{key:"value",get:function(){return f.__wbg_get_arrowstyle_arrow_offset(this.__wbg_ptr)},set:function(t){f.__wbg_set_arrowstyle_arrow_offset(this.__wbg_ptr,t)}},{key:"get_id",value:function(){var t,e;try{var n=f.__wbindgen_add_to_stack_pointer(-16);f.datapoint_get_id(n,this.__wbg_ptr);var r=j()[n/4+0],o=j()[n/4+1];return t=r,e=o,y(r,o)}finally{f.__wbindgen_add_to_stack_pointer(16),f.__wbindgen_free(t,e)}}}],[{key:"__wrap",value:function(e){e>>>=0;var n=Object.create(t.prototype);return n.__wbg_ptr=e,n}}]),t}(),P=function(){function t(e,n,r){a(this,t),S(e,F);var o=e.__destroy_into_raw();S(n,F);var i=n.__destroy_into_raw();S(r,F);var c=r.__destroy_into_raw(),u=f.graphinitiator_new(o,i,c);return t.__wrap(u)}return u(t,[{key:"__destroy_into_raw",value:function(){var t=this.__wbg_ptr;return this.__wbg_ptr=0,t}},{key:"free",value:function(){var t=this.__destroy_into_raw();f.__wbg_graphinitiator_free(t)}},{key:"origin",get:function(){var t=f.__wbg_get_datapoint_point(this.__wbg_ptr);return F.__wrap(t)},set:function(t){S(t,F);var e=t.__destroy_into_raw();f.__wbg_set_datapoint_point(this.__wbg_ptr,e)}},{key:"size",get:function(){var t=f.__wbg_get_graphinitiator_size(this.__wbg_ptr);return F.__wrap(t)},set:function(t){S(t,F);var e=t.__destroy_into_raw();f.__wbg_set_graphinitiator_size(this.__wbg_ptr,e)}},{key:"scale",get:function(){var t=f.__wbg_get_graphinitiator_scale(this.__wbg_ptr);return F.__wrap(t)},set:function(t){S(t,F);var e=t.__destroy_into_raw();f.__wbg_set_graphinitiator_scale(this.__wbg_ptr,e)}}],[{key:"__wrap",value:function(e){e>>>=0;var n=Object.create(t.prototype);return n.__wbg_ptr=e,n}}]),t}(),C=function(){function t(e,n){a(this,t),S(n,P);var r=n.__destroy_into_raw(),o=f.linegraph_new(b(e),r);return t.__wrap(o)}return u(t,[{key:"__destroy_into_raw",value:function(){var t=this.__wbg_ptr;return this.__wbg_ptr=0,t}},{key:"free",value:function(){var t=this.__destroy_into_raw();f.__wbg_linegraph_free(t)}},{key:"add_point",value:function(t){S(t,T);var e=t.__destroy_into_raw();f.linegraph_add_point(this.__wbg_ptr,e)}}],[{key:"__wrap",value:function(e){e>>>=0;var n=Object.create(t.prototype);return n.__wbg_ptr=e,n}}]),t}(),F=function(){function t(e,n){a(this,t);var r=f.point_new(e,n);return t.__wrap(r)}return u(t,[{key:"__destroy_into_raw",value:function(){var t=this.__wbg_ptr;return this.__wbg_ptr=0,t}},{key:"free",value:function(){var t=this.__destroy_into_raw();f.__wbg_point_free(t)}},{key:"x",get:function(){return f.__wbg_get_arrowstyle_arrow_length(this.__wbg_ptr)},set:function(t){f.__wbg_set_arrowstyle_arrow_length(this.__wbg_ptr,t)}},{key:"y",get:function(){return f.__wbg_get_arrowstyle_arrow_width(this.__wbg_ptr)},set:function(t){f.__wbg_set_arrowstyle_arrow_width(this.__wbg_ptr,t)}}],[{key:"__wrap",value:function(e){e>>>=0;var n=Object.create(t.prototype);return n.__wbg_ptr=e,n}}]),t}();function N(t,e){return R.apply(this,arguments)}function R(){return R=i(r().mark((function t(e,n){var o,i;return r().wrap((function(t){for(;;)switch(t.prev=t.next){case 0:if(!("function"==typeof Response&&e instanceof Response)){t.next=23;break}if("function"!=typeof WebAssembly.instantiateStreaming){t.next=15;break}return t.prev=2,t.next=5,WebAssembly.instantiateStreaming(e,n);case 5:case 20:return t.abrupt("return",t.sent);case 8:if(t.prev=8,t.t0=t.catch(2),"application/wasm"==e.headers.get("Content-Type")){t.next=14;break}console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",t.t0),t.next=15;break;case 14:throw t.t0;case 15:return t.next=17,e.arrayBuffer();case 17:return o=t.sent,t.next=20,WebAssembly.instantiate(o,n);case 23:return t.next=25,WebAssembly.instantiate(e,n);case 25:if(!((i=t.sent)instanceof WebAssembly.Instance)){t.next=30;break}return t.abrupt("return",{instance:i,module:e});case 30:return t.abrupt("return",i);case 31:case"end":return t.stop()}}),t,null,[[2,8]])}))),R.apply(this,arguments)}function I(){var e={wbg:{}};return e.wbg.__wbindgen_object_drop_ref=function(t){g(t)},e.wbg.__wbindgen_object_clone_ref=function(t){return b(l(t))},e.wbg.__wbg_log_c801d84ad6d2e8a7=function(t,e){console.log(y(t,e))},e.wbg.__wbg_crypto_70a96de3b6b73dac=function(t){return b(l(t).crypto)},e.wbg.__wbindgen_is_object=function(t){var e=l(t);return"object"===_(e)&&null!==e},e.wbg.__wbg_process_dd1577445152112e=function(t){return b(l(t).process)},e.wbg.__wbg_versions_58036bec3add9e6f=function(t){return b(l(t).versions)},e.wbg.__wbg_node_6a9d28205ed5b0d8=function(t){return b(l(t).node)},e.wbg.__wbindgen_is_string=function(t){return"string"==typeof l(t)},e.wbg.__wbg_msCrypto_adbc770ec9eca9c7=function(t){return b(l(t).msCrypto)},e.wbg.__wbg_require_f05d779769764e82=function(){return A((function(){return b(t.require)}),arguments)},e.wbg.__wbindgen_is_function=function(t){return"function"==typeof l(t)},e.wbg.__wbindgen_string_new=function(t,e){return b(y(t,e))},e.wbg.__wbg_getRandomValues_3774744e221a22ad=function(){return A((function(t,e){l(t).getRandomValues(l(e))}),arguments)},e.wbg.__wbg_randomFillSync_e950366c42764a07=function(){return A((function(t,e){l(t).randomFillSync(g(e))}),arguments)},e.wbg.__wbg_instanceof_Window_c5579e140698a9dc=function(t){var e;try{e=l(t)instanceof Window}catch(t){e=!1}return e},e.wbg.__wbg_document_508774c021174a52=function(t){var e=l(t).document;return O(e)?0:b(e)},e.wbg.__wbg_body_db30cc67afcfce41=function(t){var e=l(t).body;return O(e)?0:b(e)},e.wbg.__wbg_createElement_d975e66d06bc88da=function(){return A((function(t,e,n){return b(l(t).createElement(y(e,n)))}),arguments)},e.wbg.__wbg_createElementNS_0863d6a8a49df376=function(){return A((function(t,e,n,r,o){return b(l(t).createElementNS(0===e?void 0:y(e,n),y(r,o)))}),arguments)},e.wbg.__wbg_instanceof_SvgPathElement_de37f2682ff0d623=function(t){var e;try{e=l(t)instanceof SVGPathElement}catch(t){e=!1}return e},e.wbg.__wbg_setAttribute_1b177bcd399b9b56=function(){return A((function(t,e,n,r,o){l(t).setAttribute(y(e,n),y(r,o))}),arguments)},e.wbg.__wbg_instanceof_CanvasRenderingContext2d_ad94e23ca309f82e=function(t){var e;try{e=l(t)instanceof CanvasRenderingContext2D}catch(t){e=!1}return e},e.wbg.__wbg_appendChild_1139b53a65d69bed=function(){return A((function(t,e){return b(l(t).appendChild(l(e)))}),arguments)},e.wbg.__wbg_instanceof_HtmlCanvasElement_b2dfeaf97587c9fa=function(t){var e;try{e=l(t)instanceof HTMLCanvasElement}catch(t){e=!1}return e},e.wbg.__wbg_getContext_24464d6344024525=function(){return A((function(t,e,n){var r=l(t).getContext(y(e,n));return O(r)?0:b(r)}),arguments)},e.wbg.__wbg_newnoargs_c9e6043b8ad84109=function(t,e){return b(new Function(y(t,e)))},e.wbg.__wbg_call_557a2f2deacc4912=function(){return A((function(t,e){return b(l(t).call(l(e)))}),arguments)},e.wbg.__wbg_self_742dd6eab3e9211e=function(){return A((function(){return b(self.self)}),arguments)},e.wbg.__wbg_window_c409e731db53a0e2=function(){return A((function(){return b(window.window)}),arguments)},e.wbg.__wbg_globalThis_b70c095388441f2d=function(){return A((function(){return b(globalThis.globalThis)}),arguments)},e.wbg.__wbg_global_1c72617491ed7194=function(){return A((function(){return b(n.g.global)}),arguments)},e.wbg.__wbindgen_is_undefined=function(t){return void 0===l(t)},e.wbg.__wbg_call_587b30eea3e09332=function(){return A((function(t,e,n){return b(l(t).call(l(e),l(n)))}),arguments)},e.wbg.__wbg_buffer_55ba7a6b1b92e2ac=function(t){return b(l(t).buffer)},e.wbg.__wbg_newwithbyteoffsetandlength_88d1d8be5df94b9b=function(t,e,n){return b(new Uint8Array(l(t),e>>>0,n>>>0))},e.wbg.__wbg_new_09938a7d020f049b=function(t){return b(new Uint8Array(l(t)))},e.wbg.__wbg_set_3698e3ca519b3c3c=function(t,e,n){l(t).set(l(e),n>>>0)},e.wbg.__wbg_newwithlength_89eeca401d8918c2=function(t){return b(new Uint8Array(t>>>0))},e.wbg.__wbg_subarray_d82be056deb4ad27=function(t,e,n){return b(l(t).subarray(e>>>0,n>>>0))},e.wbg.__wbindgen_debug_string=function(t,e){var n=k(v(l(e)),f.__wbindgen_malloc,f.__wbindgen_realloc),r=m;j()[t/4+1]=r,j()[t/4+0]=n},e.wbg.__wbindgen_throw=function(t,e){throw new Error(y(t,e))},e.wbg.__wbindgen_memory=function(){return b(f.memory)},e}function W(t,e){return f=t.exports,G.__wbindgen_wasm_module=e,L=null,p=null,f.__wbindgen_start(),f}function G(t){return U.apply(this,arguments)}function U(){return U=i(r().mark((function t(e){var o,i,a,c;return r().wrap((function(t){for(;;)switch(t.prev=t.next){case 0:if(void 0===f){t.next=2;break}return t.abrupt("return",f);case 2:return void 0===e&&(e=new URL(n(784),n.b)),o=I(),("string"==typeof e||"function"==typeof Request&&e instanceof Request||"function"==typeof URL&&e instanceof URL)&&(e=fetch(e)),t.t0=N,t.next=9,e;case 9:return t.t1=t.sent,t.t2=o,t.next=13,(0,t.t0)(t.t1,t.t2);case 13:return i=t.sent,a=i.instance,c=i.module,t.abrupt("return",W(a,c));case 17:case"end":return t.stop()}}),t)}))),U.apply(this,arguments)}const B=G},784:(t,e,n)=>{t.exports=n.p+"efdbf4fefb98f1a85557.wasm"}},n={};function r(t){var o=n[t];if(void 0!==o)return o.exports;var i=n[t]={id:t,loaded:!1,exports:{}};return e[t](i,i.exports,r),i.loaded=!0,i.exports}r.m=e,r.d=(t,e)=>{for(var n in e)r.o(e,n)&&!r.o(t,n)&&Object.defineProperty(t,n,{enumerable:!0,get:e[n]})},r.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(t){if("object"==typeof window)return window}}(),r.hmd=t=>((t=Object.create(t)).children||(t.children=[]),Object.defineProperty(t,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+t.id)}}),t),r.o=(t,e)=>Object.prototype.hasOwnProperty.call(t,e),(()=>{var t;r.g.importScripts&&(t=r.g.location+"");var e=r.g.document;if(!t&&e&&(e.currentScript&&(t=e.currentScript.src),!t)){var n=e.getElementsByTagName("script");n.length&&(t=n[n.length-1].src)}if(!t)throw new Error("Automatic publicPath is not supported in this browser");t=t.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),r.p=t})(),r.b=document.baseURI||self.location.href,(0,(t=r(246)).ZP)().then((function(){return function(){console.info("WASM Bindings loaded");var e=document.getElementById("main");console.info("Canvas element found",e);var n=new t.xr(e,new t.BA(new t.E9(0,0),new t.E9(0,0),new t.E9(0,0))),r=new t.hz(new t.E9(0,0),10);n.add_point(r)}()}))})();