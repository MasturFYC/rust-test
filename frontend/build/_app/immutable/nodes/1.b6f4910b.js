import{s as b,f as u,l as h,a as S,g as d,h as v,m as g,d as m,c as q,i as _,x,n as E,O as $,K as y}from"../chunks/scheduler.24659058.js";import{S as C,i as H}from"../chunks/index.fb75f0fb.js";import{p as K}from"../chunks/stores.9461886f.js";function O(i){var f;let a,s=i[0].status+"",r,o,n,p=((f=i[0].error)==null?void 0:f.message)+"",c;return{c(){a=u("h1"),r=h(s),o=S(),n=u("p"),c=h(p)},l(e){a=d(e,"H1",{});var t=v(a);r=g(t,s),t.forEach(m),o=q(e),n=d(e,"P",{});var l=v(n);c=g(l,p),l.forEach(m)},m(e,t){_(e,a,t),x(a,r),_(e,o,t),_(e,n,t),x(n,c)},p(e,[t]){var l;t&1&&s!==(s=e[0].status+"")&&E(r,s),t&1&&p!==(p=((l=e[0].error)==null?void 0:l.message)+"")&&E(c,p)},i:$,o:$,d(e){e&&(m(a),m(o),m(n))}}}function P(i,a,s){let r;return y(i,K,o=>s(0,r=o)),[r]}class z extends C{constructor(a){super(),H(this,a,P,O,b,{})}}export{z as component};
