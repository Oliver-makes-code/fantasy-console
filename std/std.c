#include <stdint.h>
#include "std.h"

// The size of a page in WASM
#define PAGE_SIZE 65536

// Gets the number of pages for the memory
uint32_t memory_size() {
    uint32_t value;
    asm volatile(
        "memory.size 0\n"
        "local.set %0"
        : [value] "=r" (value)
    );
    return value;
}

// Grows the memory by amount
uint32_t memory_grow(uint32_t amount) {
    uint32_t value;
    asm volatile(
        "local.get %1\n"
        "memory.grow 0\n"
        "local.set %0\n"
        : [value] "=r" (value)
        : [amount] "r" (amount)
    );
    return value;
}

uint8_t *internal_page;

void std_Init() {
    internal_page = (uint8_t *)(memory_size() * PAGE_SIZE);
}
