// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.
// Copyright 2021 RiteLabs. Licensed under Apache-2.0 OR MIT.

use protobuf_build::Builder;

fn main() {
    let base = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    Builder::new()
        .search_dir_for_protos(&format!("{}/proto", base))
        .includes(&[format!("{}/include", base), format!("{}/proto", base)])
        .generate()
}
