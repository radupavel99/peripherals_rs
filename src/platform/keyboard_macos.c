

#if defined(__APPLE__) && defined(__MACH__)
#include <ApplicationServices/ApplicationServices.h>
#endif // defined(__APPLE__) && defined(__MACH__)

#if defined(__APPLE__) && defined(__MACH__)
bool key_state(uint16_t key) {
    return CGEventSourceKeyState(0, key);
}
#endif // defined(__APPLE__) && defined(__MACH__)
