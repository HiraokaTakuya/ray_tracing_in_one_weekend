(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
eval("\n\nconsole.log(\"start loading wasm\");\nconst ray_tracing = __webpack_require__.e(/*! import() */ 0).then(__webpack_require__.bind(null, /*! ../pkg */ \"../pkg/ray_tracing_in_one_weekend.js\"))\n    .catch(console.error);\nPromise.all([ray_tracing]).then(async function ([{ draw_ray_tracing_set }]) {\n    console.log(\"finished loading wasm\");\n\n    const renderBtn = document.getElementById('render');\n    renderBtn.addEventListener('click', () => {\n        let jsResult = null;\n        let wasmResult = null;\n        {\n            console.log(\"wasm only\");\n            draw_ray_tracing_set();\n        }\n    });\n});\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);