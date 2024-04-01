#include "defines.h"

#ifdef __cplusplus
extern "C" {
#endif

WASM("dbg", "write_character")
extern void dbg_WriteCharacter(char c);

WASM("dbg", "end_line")
extern void dbg_EndLine();

WASM("dbg", "write_str")
extern void dbg_WriteString(char *c);

#ifdef __cplusplus
}
#endif
