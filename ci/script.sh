# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    rustup target add $TARGET || true
    # cargo build --target $TARGET
    cargo build --target $TARGET --release --verbose
    ulimit -a

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    # cargo test --target $TARGET
    RUST_TEST_THREADS=1 RUST_LOG=hclrs::tests=debug cargo test --target $TARGET --release --verbose -- error_missing_semicolon_reg
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
