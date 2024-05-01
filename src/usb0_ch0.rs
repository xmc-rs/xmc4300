#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcchar: HCCHAR,
    _reserved1: [u8; 0x04],
    hcint: HCINT,
    hcintmsk: HCINTMSK,
    _reserved_3_hctsiz: [u8; 0x04],
    _reserved_4_hcdma: [u8; 0x04],
    _reserved5: [u8; 0x04],
    hcdmab: HCDMAB,
}
impl RegisterBlock {
    #[doc = "0x00 - Host Channel Characteristics Register"]
    #[inline(always)]
    pub const fn hcchar(&self) -> &HCCHAR {
        &self.hcchar
    }
    #[doc = "0x08 - Host Channel Interrupt Register"]
    #[inline(always)]
    pub const fn hcint(&self) -> &HCINT {
        &self.hcint
    }
    #[doc = "0x0c - Host Channel Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hcintmsk(&self) -> &HCINTMSK {
        &self.hcintmsk
    }
    #[doc = "0x10 - Host Channel Transfer Size Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub const fn hctsiz_scatgather(&self) -> &HCTSIZ_SCATGATHER {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub const fn hctsiz_buffermode(&self) -> &HCTSIZ_BUFFERMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub const fn hcdma_scatgather(&self) -> &HCDMA_SCATGATHER {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub const fn hcdma_buffermode(&self) -> &HCDMA_BUFFERMODE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x1c - Host Channel DMA Buffer Address Register"]
    #[inline(always)]
    pub const fn hcdmab(&self) -> &HCDMAB {
        &self.hcdmab
    }
}
#[doc = "HCCHAR (rw) register accessor: Host Channel Characteristics Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar`]
module"]
pub type HCCHAR = crate::Reg<hcchar::HCCHAR_SPEC>;
#[doc = "Host Channel Characteristics Register"]
pub mod hcchar;
#[doc = "HCINT (rw) register accessor: Host Channel Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint`]
module"]
pub type HCINT = crate::Reg<hcint::HCINT_SPEC>;
#[doc = "Host Channel Interrupt Register"]
pub mod hcint;
#[doc = "HCINTMSK (rw) register accessor: Host Channel Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk`]
module"]
pub type HCINTMSK = crate::Reg<hcintmsk::HCINTMSK_SPEC>;
#[doc = "Host Channel Interrupt Mask Register"]
pub mod hcintmsk;
#[doc = "HCTSIZ_BUFFERMODE (rw) register accessor: Host Channel Transfer Size Register \\[BUFFERMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz_buffermode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz_buffermode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz_buffermode`]
module"]
pub type HCTSIZ_BUFFERMODE = crate::Reg<hctsiz_buffermode::HCTSIZ_BUFFERMODE_SPEC>;
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
pub mod hctsiz_buffermode;
#[doc = "HCTSIZ_SCATGATHER (rw) register accessor: Host Channel Transfer Size Register \\[SCATGATHER\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz_scatgather::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz_scatgather::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz_scatgather`]
module"]
pub type HCTSIZ_SCATGATHER = crate::Reg<hctsiz_scatgather::HCTSIZ_SCATGATHER_SPEC>;
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]"]
pub mod hctsiz_scatgather;
#[doc = "HCDMA_BUFFERMODE (rw) register accessor: Host Channel DMA Address Register \\[BUFFERMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma_buffermode::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma_buffermode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma_buffermode`]
module"]
pub type HCDMA_BUFFERMODE = crate::Reg<hcdma_buffermode::HCDMA_BUFFERMODE_SPEC>;
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]"]
pub mod hcdma_buffermode;
#[doc = "HCDMA_SCATGATHER (rw) register accessor: Host Channel DMA Address Register \\[SCATGATHER\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma_scatgather::R`]. WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma_scatgather::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma_scatgather`]
module"]
pub type HCDMA_SCATGATHER = crate::Reg<hcdma_scatgather::HCDMA_SCATGATHER_SPEC>;
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]"]
pub mod hcdma_scatgather;
#[doc = "HCDMAB (r) register accessor: Host Channel DMA Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdmab::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab`]
module"]
pub type HCDMAB = crate::Reg<hcdmab::HCDMAB_SPEC>;
#[doc = "Host Channel DMA Buffer Address Register"]
pub mod hcdmab;
