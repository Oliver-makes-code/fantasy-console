%:
	clang examples/$@.c --target=wasm32-unknown-unknown --no-standard-libraries -O3 -o $@.wasm -Wl,--export-all -Wl,--no-entry
