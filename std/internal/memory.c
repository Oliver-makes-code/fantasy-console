#include <memory.h>
#include <stdbool.h>
#include <stdint.h>
#include <defines.h>
#include <debug.h>

struct memoryblock {
    struct memoryblock *next;
    size_t size;
    bool free;
};

typedef struct memoryblock memoryblock_t;

memoryblock_t *heap_pointer asm("std_internal_memory_heap_pointer") = 0;

// Gets the number of pages for the memory
inline uint32_t memory_size() asm("std_internal_memory_size");
inline uint32_t memory_size() {
    uint32_t value;
    asm volatile(
        "memory.size 0\n"
        "local.set %0"
        : [value] "=r" (value)
    );
    return value;
}

// Grows the memory by amount
inline uint32_t memory_grow(uint32_t amount) asm("std_internal_memory_grow");
inline uint32_t memory_grow(uint32_t amount) {
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

memoryblock_t *reserve_page() asm("std_internal_memory_reserve_page");
memoryblock_t *reserve_page() {
    memoryblock_t *ptr = (memoryblock_t *)(memory_size() * PAGE_SIZE);
    memory_grow(1);

    *ptr = (memoryblock_t) {
        .next = 0,
        .size = PAGE_SIZE,
        .free = true
    };

    return ptr;
}

memoryblock_t *next_free(size_t size) asm("std_internal_memory_next_free");
memoryblock_t *next_free(size_t size) {
    // Not initialized
    if (!heap_pointer)
        heap_pointer = reserve_page();

    // Calculate the actual needed size
    size_t acutal_size = size + sizeof(memoryblock_t);

    memoryblock_t *curr = heap_pointer;

    // Go through the list
    loop {
        // If it fits and is free, return
        if (curr->free && curr->size >= acutal_size)
            return curr;

        // If it doesn't have a next, create a new one
        if (!curr->next) {
            memoryblock_t *next = reserve_page();
            // Merge the blocks if current is free
            if (curr->free) {
                curr->size += next->size;
                next = curr;
            } else {
                curr->next = next;
            }
            curr = next;
            // Continue in case the user is requesting > PAGE_SIZE
            continue;
        }

        // Get the next element
        curr = curr->next;
    }
}

void merge_blocks() asm("std_internal_memory_merge_blocks");
void merge_blocks() {
    // If the heap hasn't been initialized, return
    if (!heap_pointer)
        return;

    memoryblock_t *curr = heap_pointer;

    // While there's another pointer to check
    while (curr->next) {
        // If both current and next are free, merge them
        if (curr->free && curr->next->free) {
            curr->size += curr->next->size;
            curr->next = curr->next->next;
        } else {
            curr = curr->next;
        }
    }
}

void *malloc(size_t size) {
    // Get the next free block
    memoryblock_t *block = next_free(size);
    void *v_block = block;

    // Get the left over bytes
    size_t leftover = block->size - size;
    if (leftover >= sizeof(memoryblock_t)) {
        memoryblock_t *next = (v_block + size);

        // Create the metadata
        next->next = block->next;
        next->size = leftover;
        next->free = true;

        // Update the current block
        block->next = next;
        block->size = size;
    }

    // Mark as not free
    block->free = false;

    void *out = v_block + sizeof(memoryblock_t);

    return out;
}

void free(void *p) {
    // If either are null, return
    if (!heap_pointer || !p)
        return;
    memoryblock_t *base = (p - sizeof(memoryblock_t));

    bool actual = false;

    if (heap_pointer == base) {
        // This is the heap pointer.
        actual = true;
    } else {
        memoryblock_t *curr = heap_pointer;
        loop {
            // This is a heap pointer
            if (curr->next == base) {
                actual = true;
                break;
            }
            // This isn't a heap pointer
            if (!curr->next)
                break;
        }
    }

    // This isn't a heap pointer
    if (!actual)
        return;

    base->free = true;
    merge_blocks();
}
