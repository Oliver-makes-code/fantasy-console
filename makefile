examples/%: std
	clang $@.c std.o --target=wasm32-unknown-unknown --no-standard-libraries -O3 -Istd -o $(patsubst examples/%,%,$@).wasm -Wl,--export-all -Wl,--no-entry

std: std.o
	clang -c std/*.c --target=wasm32-unknown-unknown --no-standard-libraries -O3 -o std.o

.PHONY: std