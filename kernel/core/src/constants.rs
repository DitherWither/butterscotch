pub const KERNEL_VERSION: &str = "v0.2.0 Alpha";

// Start the heap at a this address to make it easier to recognize
pub const HEAP_START: usize = 0x_C444_4444_0000;
pub const HEAP_DEFAULT_SIZE: usize = 8 * 1024 * 1024; // 8MiB
