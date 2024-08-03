import{s as q,a as B,e as d,c as U,i as b,d as h,b as j,o as W,f as z,g as F,h as G,j as D,k as m,l as H,m as J,n as K,t as M,p as I,q as k}from"../chunks/scheduler.a4f235b2.js";import{S as Q,i as X,t as g,c as L,a as w,g as P,b as v,d as O,m as R,e as y}from"../chunks/index.bd871e0a.js";const Y="modulepreload",Z=function(o,e){return new URL(o,e).href},T={},p=function(e,n,i){if(!n||n.length===0)return e();const r=document.getElementsByTagName("link");return Promise.all(n.map(f=>{if(f=Z(f,i),f in T)return;T[f]=!0;const t=f.endsWith(".css"),s=t?'[rel="stylesheet"]':"";if(!!i)for(let a=r.length-1;a>=0;a--){const u=r[a];if(u.href===f&&(!t||u.rel==="stylesheet"))return}else if(document.querySelector(`link[href="${f}"]${s}`))return;const c=document.createElement("link");if(c.rel=t?"stylesheet":Y,t||(c.as="script",c.crossOrigin=""),c.href=f,document.head.appendChild(c),t)return new Promise((a,u)=>{c.addEventListener("load",a),c.addEventListener("error",()=>u(new Error(`Unable to preload CSS for ${f}`)))})})).then(()=>e()).catch(f=>{const t=new Event("vite:preloadError",{cancelable:!0});if(t.payload=f,window.dispatchEvent(t),!t.defaultPrevented)throw f})},se={};function $(o){let e,n,i;var r=o[1][0];function f(t,s){return{props:{data:t[3],form:t[2]}}}return r&&(e=k(r,f(o)),o[12](e)),{c(){e&&v(e.$$.fragment),n=d()},l(t){e&&O(e.$$.fragment,t),n=d()},m(t,s){e&&R(e,t,s),b(t,n,s),i=!0},p(t,s){if(s&2&&r!==(r=t[1][0])){if(e){P();const l=e;g(l.$$.fragment,1,0,()=>{y(l,1)}),L()}r?(e=k(r,f(t)),t[12](e),v(e.$$.fragment),w(e.$$.fragment,1),R(e,n.parentNode,n)):e=null}else if(r){const l={};s&8&&(l.data=t[3]),s&4&&(l.form=t[2]),e.$set(l)}},i(t){i||(e&&w(e.$$.fragment,t),i=!0)},o(t){e&&g(e.$$.fragment,t),i=!1},d(t){t&&h(n),o[12](null),e&&y(e,t)}}}function x(o){let e,n,i;var r=o[1][0];function f(t,s){return{props:{data:t[3],$$slots:{default:[ee]},$$scope:{ctx:t}}}}return r&&(e=k(r,f(o)),o[11](e)),{c(){e&&v(e.$$.fragment),n=d()},l(t){e&&O(e.$$.fragment,t),n=d()},m(t,s){e&&R(e,t,s),b(t,n,s),i=!0},p(t,s){if(s&2&&r!==(r=t[1][0])){if(e){P();const l=e;g(l.$$.fragment,1,0,()=>{y(l,1)}),L()}r?(e=k(r,f(t)),t[11](e),v(e.$$.fragment),w(e.$$.fragment,1),R(e,n.parentNode,n)):e=null}else if(r){const l={};s&8&&(l.data=t[3]),s&8215&&(l.$$scope={dirty:s,ctx:t}),e.$set(l)}},i(t){i||(e&&w(e.$$.fragment,t),i=!0)},o(t){e&&g(e.$$.fragment,t),i=!1},d(t){t&&h(n),o[11](null),e&&y(e,t)}}}function ee(o){let e,n,i;var r=o[1][1];function f(t,s){return{props:{data:t[4],form:t[2]}}}return r&&(e=k(r,f(o)),o[10](e)),{c(){e&&v(e.$$.fragment),n=d()},l(t){e&&O(e.$$.fragment,t),n=d()},m(t,s){e&&R(e,t,s),b(t,n,s),i=!0},p(t,s){if(s&2&&r!==(r=t[1][1])){if(e){P();const l=e;g(l.$$.fragment,1,0,()=>{y(l,1)}),L()}r?(e=k(r,f(t)),t[10](e),v(e.$$.fragment),w(e.$$.fragment,1),R(e,n.parentNode,n)):e=null}else if(r){const l={};s&16&&(l.data=t[4]),s&4&&(l.form=t[2]),e.$set(l)}},i(t){i||(e&&w(e.$$.fragment,t),i=!0)},o(t){e&&g(e.$$.fragment,t),i=!1},d(t){t&&h(n),o[10](null),e&&y(e,t)}}}function V(o){let e,n=o[6]&&A(o);return{c(){e=z("div"),n&&n.c(),this.h()},l(i){e=F(i,"DIV",{id:!0,"aria-live":!0,"aria-atomic":!0,style:!0});var r=G(e);n&&n.l(r),r.forEach(h),this.h()},h(){D(e,"id","svelte-announcer"),D(e,"aria-live","assertive"),D(e,"aria-atomic","true"),m(e,"position","absolute"),m(e,"left","0"),m(e,"top","0"),m(e,"clip","rect(0 0 0 0)"),m(e,"clip-path","inset(50%)"),m(e,"overflow","hidden"),m(e,"white-space","nowrap"),m(e,"width","1px"),m(e,"height","1px")},m(i,r){b(i,e,r),n&&n.m(e,null)},p(i,r){i[6]?n?n.p(i,r):(n=A(i),n.c(),n.m(e,null)):n&&(n.d(1),n=null)},d(i){i&&h(e),n&&n.d()}}}function A(o){let e;return{c(){e=H(o[7])},l(n){e=J(n,o[7])},m(n,i){b(n,e,i)},p(n,i){i&128&&K(e,n[7])},d(n){n&&h(e)}}}function te(o){let e,n,i,r,f;const t=[x,$],s=[];function l(a,u){return a[1][1]?0:1}e=l(o),n=s[e]=t[e](o);let c=o[5]&&V(o);return{c(){n.c(),i=B(),c&&c.c(),r=d()},l(a){n.l(a),i=U(a),c&&c.l(a),r=d()},m(a,u){s[e].m(a,u),b(a,i,u),c&&c.m(a,u),b(a,r,u),f=!0},p(a,[u]){let E=e;e=l(a),e===E?s[e].p(a,u):(P(),g(s[E],1,1,()=>{s[E]=null}),L(),n=s[e],n?n.p(a,u):(n=s[e]=t[e](a),n.c()),w(n,1),n.m(i.parentNode,i)),a[5]?c?c.p(a,u):(c=V(a),c.c(),c.m(r.parentNode,r)):c&&(c.d(1),c=null)},i(a){f||(w(n),f=!0)},o(a){g(n),f=!1},d(a){a&&(h(i),h(r)),s[e].d(a),c&&c.d(a)}}}function ne(o,e,n){let{stores:i}=e,{page:r}=e,{constructors:f}=e,{components:t=[]}=e,{form:s}=e,{data_0:l=null}=e,{data_1:c=null}=e;j(i.page.notify);let a=!1,u=!1,E=null;W(()=>{const _=i.page.subscribe(()=>{a&&(n(6,u=!0),M().then(()=>{n(7,E=document.title||"untitled page")}))});return n(5,a=!0),_});function N(_){I[_?"unshift":"push"](()=>{t[1]=_,n(0,t)})}function S(_){I[_?"unshift":"push"](()=>{t[0]=_,n(0,t)})}function C(_){I[_?"unshift":"push"](()=>{t[0]=_,n(0,t)})}return o.$$set=_=>{"stores"in _&&n(8,i=_.stores),"page"in _&&n(9,r=_.page),"constructors"in _&&n(1,f=_.constructors),"components"in _&&n(0,t=_.components),"form"in _&&n(2,s=_.form),"data_0"in _&&n(3,l=_.data_0),"data_1"in _&&n(4,c=_.data_1)},o.$$.update=()=>{o.$$.dirty&768&&i.page.set(r)},[t,f,s,l,c,a,u,E,i,r,N,S,C]}class oe extends Q{constructor(e){super(),X(this,e,ne,te,q,{stores:8,page:9,constructors:1,components:0,form:2,data_0:3,data_1:4})}}const ae=[()=>p(()=>import("../nodes/0.f04bab7d.js"),["../nodes/0.f04bab7d.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js","../chunks/Close.925834a2.js","../chunks/WarningAltFilled.c9065f8a.js","../chunks/stores.001abc97.js","../chunks/singletons.f13f43b7.js","../chunks/index.693a3765.js","../chunks/Row.d2ac7eba.js","../chunks/LocalStorage.ecf7c53e.js","../chunks/navigation.593a0fe6.js","../chunks/index.eacf8113.js","../assets/0.1337b139.css"],import.meta.url),()=>p(()=>import("../nodes/1.deae9d70.js"),["../nodes/1.deae9d70.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js","../chunks/stores.001abc97.js","../chunks/singletons.f13f43b7.js","../chunks/index.693a3765.js"],import.meta.url),()=>p(()=>import("../nodes/2.ac02686a.js"),["../nodes/2.ac02686a.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js"],import.meta.url),()=>p(()=>import("../nodes/3.80a52de0.js"),["../nodes/3.80a52de0.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js","../chunks/index.eacf8113.js","../chunks/TextInput.b52bdd72.js","../chunks/WarningAltFilled.c9065f8a.js","../chunks/Delete.6f0a3216.js","../chunks/index.693a3765.js","../chunks/Close.925834a2.js","../chunks/FluidForm.3544e16e.js","../assets/3.0749f547.css"],import.meta.url),()=>p(()=>import("../nodes/4.8b1ff486.js"),["../nodes/4.8b1ff486.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js","../chunks/navigation.593a0fe6.js","../chunks/singletons.f13f43b7.js","../chunks/index.693a3765.js","../chunks/index.eacf8113.js","../chunks/TextInput.b52bdd72.js","../chunks/WarningAltFilled.c9065f8a.js","../assets/4.69ae3c68.css"],import.meta.url),()=>p(()=>import("../nodes/5.665ceca9.js"),["../nodes/5.665ceca9.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js","../chunks/DeleteRelation.7e03cf5b.js","../chunks/WarningAltFilled.c9065f8a.js","../chunks/Close.925834a2.js","../chunks/TextInput.b52bdd72.js","../chunks/Row.d2ac7eba.js","../chunks/index.693a3765.js","../chunks/Delete.6f0a3216.js","../chunks/index.eacf8113.js","../assets/5.7c2761c1.css"],import.meta.url),()=>p(()=>import("../nodes/6.714a222e.js"),["../nodes/6.714a222e.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js","../chunks/index.eacf8113.js","../chunks/LocalStorage.ecf7c53e.js","../assets/6.b9292ddf.css"],import.meta.url),()=>p(()=>import("../nodes/7.3d100299.js"),["../nodes/7.3d100299.js","../chunks/scheduler.a4f235b2.js","../chunks/index.bd871e0a.js","../chunks/Close.925834a2.js","../chunks/WarningAltFilled.c9065f8a.js","../chunks/DeleteRelation.7e03cf5b.js","../chunks/TextInput.b52bdd72.js","../chunks/Row.d2ac7eba.js","../chunks/index.693a3765.js","../chunks/Delete.6f0a3216.js","../chunks/FluidForm.3544e16e.js","../chunks/index.eacf8113.js","../assets/7.586f77cb.css"],import.meta.url)],le=[],fe={"/":[2],"/category":[3],"/login":[4],"/product":[5],"/profile":[6],"/relation":[7]},ce={handleError:({error:o})=>{console.error(o)}};export{fe as dictionary,ce as hooks,se as matchers,ae as nodes,oe as root,le as server_loads};
