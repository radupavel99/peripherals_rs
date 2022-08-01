#ifndef A
#define A

#include <stdbool.h>
#include <stdint.h>

#if defined(__APPLE__) && defined(__MACH__)
bool key_state(uint16_t key);
#endif // defined(__APPLE__) && defined(__MACH__)

#endif A
