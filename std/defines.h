#pragma once

#include <stdint.h>
#include <stddef.h>

#define WASM(mod, name) __attribute__((import_module(mod), import_name(name)))

#define NO_INLINE __attribute__((noinline))

// The size of a page in WASM
#define PAGE_SIZE 65536

#define loop while (true)
