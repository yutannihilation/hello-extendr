

// Take the address of the wrap__hello stub function to avoid
// the linker removing the static library.
//
// This will be removed in future versions with the module macro.
void R_init_libhello();
void R_init_libhello2();

void *__dummy = (void*)&R_init_libhello;
void *__dummy2 = (void*)&R_init_libhello2;
