/// Sets the base revision to 1, this is recommended as this is the latest base revision described
/// by the Limine boot protocol specification. See specification for further info.
pub static _BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(1);

pub static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(1);
pub static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new(1);

pub static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(1);
