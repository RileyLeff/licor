// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_licorclient_extendr(void *dll);

void R_init_licorclient(void *dll) {
    R_init_licorclient_extendr(dll);
}
