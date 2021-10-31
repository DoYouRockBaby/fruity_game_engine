cargo build \
  && RUST_BACKTRACE=1 cargo watch \
  -i "*/fruity_editor/**" \
  -i "*/fruity_graphic/**" \
  -i "*/fruity_graphic_2d/**" \
  -i "*/fruity_javascript_scripting/**" \
  -i "*/fruity_windows/**" \
  -x "run"