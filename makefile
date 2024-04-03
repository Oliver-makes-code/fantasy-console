rwildcard=$(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

COMPILE_FLAGS = -Istd -mmultivalue -Xclang -target-abi -Xclang experimental-mv --target=wasm32-unknown-unknown --no-standard-libraries -O3

examples/%: std
	clang $@.c std.a $(COMPILE_FLAGS) -o $(patsubst examples/%,%,$@).wasm -Wl,--export-all -Wl,--no-entry

std: $(patsubst %.c,%,$(call rwildcard,std,*.c))
	-rm std.a
	llvm-ar rcs std.a $(call rwildcard,std,*.o) 

std/%:
	clang -c $@.c $(COMPILE_FLAGS) -o $@.o

.PHONY: std
