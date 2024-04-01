examples/%: std
	clang $@.c std.o -mmultivalue -Xclang -target-abi -Xclang experimental-mv --target=wasm32-unknown-unknown --no-standard-libraries -O3 -Istd -o $(patsubst examples/%,%,$@).wasm -Wl,--export-all -Wl,--no-entry

std: std.o
	clang -c std/*.c -mmultivalue -Xclang -target-abi -Xclang experimental-mv --target=wasm32-unknown-unknown --no-standard-libraries -O3 -o std.o

.PHONY: std
