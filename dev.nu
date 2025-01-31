# Runs auto rebuild for `chipbox-dev`.
# Requires `cargo-watch` to be installed.
def main () {
    exec "cargo" "watch" "-w" "chipbox-dev" "-x" "run --bin chipbox-dev --release -- -v"
}
