Based on the `APRILTAG_SRC` environment variable, the apriltag implementation is
chosen.

If `APRILTAG_SRC` is not set, a precompiled apriltag library will be used with
details coming from `pkg-config`. Further environment variables can be used to
tune this, as specified [in the pkg-config crate
docs](https://docs.rs/pkg-config/0.3/pkg_config/).

Otherwise, `APRILTAG_SRC` is assumed to specify a directory with the source
code of the apriltag library. This source code is then compiled and statically
linked into the rust crate.

Example:

    export APRILTAG_SRC=$HOME/Documents/other-peoples-src/apriltag
    cargo test

Regenerate `src/lib.rs` with:

    ./run-bindgen.sh

Currently, the bindings are built against commit
756423674639e1bd3c38503c34a1e39cec57cd0a from
https://github.com/AprilRobotics/apriltag .
