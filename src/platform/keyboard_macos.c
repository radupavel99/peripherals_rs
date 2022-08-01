#include <stdbool.h>

#if defined(__APPLE__) && defined(__MACH__)
#include <ApplicationServices/ApplicationServices.h>
#endif // defined(__APPLE__) && defined(__MACH__)

bool key_state(uint32_t key) {
#if defined(__APPLE__) && defined(__MACH__)
    return CGEventSourceKeyState(0, key);
#endif // defined(__APPLE__) && defined(__MACH__)
}
