// roughly based on emscripten's generated javascript and gl.js

var Module = {
  onRuntimeInitialized: () => {
    shouldRunNow = false;
    register_plugins(plugins);
    wasm_memory = wasmMemory;
    wasm_exports = Object.fromEntries(
      Object.entries(Module)
        .filter(([k]) => k[0] == "_" && k[1] != "_")
        .map(([k, v]) => [k.slice(1), v]),
    );
    var crate_version = wasm_exports.crate_version();
    if (version != crate_version) {
      console.error(
        "Version mismatch: gl.js version is: " +
        version +
        ", rust sapp-wasm crate version is: " +
        crate_version,
      );
    }
    init_plugins(plugins);
    wasm_exports.main();
  },
};
