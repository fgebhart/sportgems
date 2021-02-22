#!/bin/bash
source $VIRTUAL_ENV_PATH/bin/activate
# workaround for linker error when testing, see https://pyo3.rs/v0.13.2/faq.html#i-cant-run-cargo-test-im-having-linker-issues-like-symbol-not-found-or-undefined-reference-to-_pyexc_systemerror
alias "cargotest"="cargo test --no-default-features"
zsh
