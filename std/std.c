#include <stdint.h>
#include "std.h"
#include "debug.h"

// The size of a page in WASM
#define PAGE_SIZE 65536

// Gets the number of pages for the memory
uint32_t std_internal_MemorySize() {
    uint32_t value;
    asm volatile(
        "memory.size 0\n"
        "local.set %0"
        : [value] "=r" (value)
    );
    return value;
}

// Grows the memory by amount
uint32_t std_internal_MemoryGrow(uint32_t amount) {
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

// A page we keep reserved for internal data stuctures.
uint8_t *std_internal_DataPage;

void std_Init() {
    std_internal_DataPage = (uint8_t *)(std_internal_MemorySize() * PAGE_SIZE);
    std_internal_MemoryGrow(1);

    dbg_WriteString(":3\n");

    // Set up data structures and stuff
}
