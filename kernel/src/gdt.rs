use x86_64::{
    instructions::tables::load_tss,
    registers::segmentation::{Segment, CS},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

/// # Safety
///
/// Should only be called on one thread, once
/// Is called by main during startup
pub unsafe fn init() {
    TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        #[allow(static_mut_ref)]
        let stack_start = VirtAddr::from_ptr(&STACK);
        stack_start + STACK_SIZE // stack end
    };

    // kernel code/data
    // let kernel_code_selector = GDT.add_entry(Descriptor::kernel_code_segment());
    // GDT.add_entry(Descriptor::kernel_data_segment());

    // #[allow(static_mut_ref)]
    // let tss_selector = GDT.add_entry(Descriptor::tss_segment(&TSS));

    // GDT.load();
    // CS::set_reg(kernel_code_selector);
    // load_tss(tss_selector);
}
