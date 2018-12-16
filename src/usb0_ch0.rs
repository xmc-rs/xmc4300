#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Host Channel Characteristics Register"]
    pub hcchar: HCCHAR,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Host Channel Interrupt Register"]
    pub hcint: HCINT,
    #[doc = "0x0c - Host Channel Interrupt Mask Register"]
    pub hcintmsk: HCINTMSK,
    #[doc = "0x10 - Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
    pub hctsiz_buffermode: HCTSIZ_BUFFERMODE,
    #[doc = "0x14 - Host Channel DMA Address Register \\[BUFFERMODE\\]"]
    pub hcdma_buffermode: HCDMA_BUFFERMODE,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Host Channel DMA Buffer Address Register"]
    pub hcdmab: HCDMAB,
}
#[doc = "Host Channel Characteristics Register"]
pub struct HCCHAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel Characteristics Register"]
pub mod hcchar;
#[doc = "Host Channel Interrupt Register"]
pub struct HCINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel Interrupt Register"]
pub mod hcint;
#[doc = "Host Channel Interrupt Mask Register"]
pub struct HCINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel Interrupt Mask Register"]
pub mod hcintmsk;
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
pub struct HCTSIZ_BUFFERMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
pub mod hctsiz_buffermode;
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]"]
pub struct HCTSIZ_SCATGATHER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]"]
pub mod hctsiz_scatgather;
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]"]
pub struct HCDMA_BUFFERMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]"]
pub mod hcdma_buffermode;
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]"]
pub struct HCDMA_SCATGATHER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]"]
pub mod hcdma_scatgather;
#[doc = "Host Channel DMA Buffer Address Register"]
pub struct HCDMAB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Channel DMA Buffer Address Register"]
pub mod hcdmab;
