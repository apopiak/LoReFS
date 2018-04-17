// extern declarations
extern void lorefs_subrinit(void);
extern void lorefs_subrfini(void);

// declarations for symbols defined in Rust
// vnode ops functions
int lo_close(vnode_t *vp, int flag, int count, offset_t offset, cred *cr, caller_context_t *ct);

// atomic mount counter functions
void lorefs_inc_mount_count();
void lorefs_dec_mount_count();
uint32_t lorefs_mount_count();
void lorefs_reset_mount_count();

struct modlinkage;
int32_t lorefs_mod_remove(struct modlinkage *);

// test functions for calling into Rust; TODO: remove
void lorefs_print_notice();
int32_t lorefs_add(int32_t, int32_t);
