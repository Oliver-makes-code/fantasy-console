#include <defines.h>

#ifdef __cplusplus
extern "C" {
#endif

WASM("dbg", "write_character")
extern void dbg_WriteCharacter(char c);

WASM("dbg", "end_line")
extern void dbg_EndLine();

WASM("dbg", "write_str")
extern void dbg_WriteString(char *c);

WASM("dbg", "write_int")
extern void dbg_WriteInt(int64_t i);

WASM("dbg", "write_uint")
extern void dbg_WriteUint(uint64_t i);

WASM("dbg", "write_ptr")
extern void dbg_WritePtr(void *p);

#ifdef __cplusplus
}
#endif
