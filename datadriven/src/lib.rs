/*!
# datadriven

**datadriven** is a tool for testing. Ported from [cockroachdb/datadriven](https://github.com/cockroachdb/datadriven)

To execute data-driven tests, pass the path of the test file as well as a
function which can interpret and execute whatever commands are present in
the test file. The framework invokes the function, passing it information
about the test case in a TestData struct.

The function must return the actual results of the case, which
run_test() compares with the expected results. If the two are not
equal, the test is marked to fail.

[run_test()](fn.run_test.html) will run the test on a file or given folder

Recommend usage:

for test function `test_func_001`, place the testdata in `src/testdata/test_func_001`, `run_test(src/testdata/test_func_001, func_001)`

for test function `test_func_002`, place the testdata in `src/testdata/test_func_002`, `run_test(src/testdata/test_func_002, func_002)`

or just run a file `run_test(src/testdata/data.txt, func_002)`

and so on.

The path tree looks like the following:
```text
.
├── Cargo.toml
└── src
    ├── datadriven.rs
    ├── lib.rs
    └── testdata
        ├── data.txt
        ├── test_func_001
        │   ├── data_001.txt
        │   └── data_002.txt
        └── test_func_002
            ├── data_001.txt
            └── data_002.txt
```

The comparison is done by [similar-asserts](https://docs.rs/similar-asserts/1.1.0/similar_asserts/)

The difference between [cockroachdb/datadriven](https://github.com/cockroachdb/datadriven)
1. no rewrite
2. no subtest

*/

#![deny(missing_docs)]

mod datadriven;
mod line_sparser;
mod test_data;
mod test_data_reader;

pub use self::datadriven::run_test;
pub use self::datadriven::walk;
pub use self::test_data::CmdArg;
pub use self::test_data::TestData;
use anyhow::Result;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

#[allow(dead_code)]
fn default_logger() {
    use std::sync::Once;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};
    static START: Once = Once::new();
    START.call_once(|| {
        let fmt_layer = fmt::layer().with_target(false);
        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("debug"))
            .unwrap();

        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(filter_layer)
            .init();
    });
}

fn get_dirs_or_file(path: &str) -> Result<Vec<PathBuf>> {
    match read_dir(path) {
        Ok(dir) => Ok(dir
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?),
        _ => Ok(vec![PathBuf::from(path)]),
    }
}
