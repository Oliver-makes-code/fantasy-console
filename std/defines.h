#pragma once
#include <stdint.h>
#include <stddef.h>

#define WASM(mod, name) __attribute__((import_module(mod), import_name(name)))
