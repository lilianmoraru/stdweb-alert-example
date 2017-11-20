extern crate glob;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("wasm32-") || target.contains("asmjs-") {
        let deps_dir = format!("{}/../../../deps",
                               std::env::var("OUT_DIR").unwrap());

        // Not very reliable, you need to clean first
        let wasm_pattern = format!("{}/*.wasm", deps_dir);
        let js_pattern = format!("{}/*.js", deps_dir);

        let js_dest = format!("{}/static/js",
                              std::env::var("CARGO_MANIFEST_DIR").unwrap());
        std::fs::create_dir_all(&js_dest).unwrap();

        for path in glob::glob(&wasm_pattern).unwrap() {
            let dest = format!("{}/alert.wasm", js_dest);
            std::fs::copy(path.unwrap(), dest).unwrap();
        }

        for path in glob::glob(&js_pattern).unwrap() {
            let dest = format!("{}/alert.js", js_dest);
            std::fs::copy(path.unwrap(), dest).unwrap();
        }
    }
}
