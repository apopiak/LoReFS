// extern declarations
extern void lorefs_subrinit(void);
extern void lorefs_subrfini(void);

// declarations for symbols defined in Rust

// atomic mount counter functions
void lorefs_inc_mount_count();
void lorefs_dec_mount_count();
uint32_t lorefs_mount_count();
void lorefs_reset_mount_count();

struct modlinkage;
int32_t lorefs_mod_remove(struct modlinkage *);

void lorefs_print_notice();
// test function for calling into Rust; TODO: remove
int32_t lorefs_add(int32_t, int32_t);
