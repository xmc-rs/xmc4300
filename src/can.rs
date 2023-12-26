#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    clc: CLC,
    _reserved1: [u8; 0x04],
    id: ID,
    fdr: FDR,
    _reserved3: [u8; 0xf0],
    list: [LIST; 16],
    mspnd: [MSPND; 8],
    _reserved5: [u8; 0x20],
    msid: [MSID; 8],
    _reserved6: [u8; 0x20],
    msimask: MSIMASK,
    panctr: PANCTR,
    mcr: MCR,
    mitr: MITR,
}
impl RegisterBlock {
    #[doc = "0x00 - CAN Clock Control Register"]
    #[inline(always)]
    pub const fn clc(&self) -> &CLC {
        &self.clc
    }
    #[doc = "0x08 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x0c - CAN Fractional Divider Register"]
    #[inline(always)]
    pub const fn fdr(&self) -> &FDR {
        &self.fdr
    }
    #[doc = "0x100..0x140 - List Register"]
    #[inline(always)]
    pub const fn list(&self, n: usize) -> &LIST {
        &self.list[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - List Register"]
    #[inline(always)]
    pub fn list_iter(&self) -> impl Iterator<Item = &LIST> {
        self.list.iter()
    }
    #[doc = "0x140..0x160 - Message Pending Register"]
    #[inline(always)]
    pub const fn mspnd(&self, n: usize) -> &MSPND {
        &self.mspnd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x160 - Message Pending Register"]
    #[inline(always)]
    pub fn mspnd_iter(&self) -> impl Iterator<Item = &MSPND> {
        self.mspnd.iter()
    }
    #[doc = "0x180..0x1a0 - Message Index Register"]
    #[inline(always)]
    pub const fn msid(&self, n: usize) -> &MSID {
        &self.msid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1a0 - Message Index Register"]
    #[inline(always)]
    pub fn msid_iter(&self) -> impl Iterator<Item = &MSID> {
        self.msid.iter()
    }
    #[doc = "0x1c0 - Message Index Mask Register"]
    #[inline(always)]
    pub const fn msimask(&self) -> &MSIMASK {
        &self.msimask
    }
    #[doc = "0x1c4 - Panel Control Register"]
    #[inline(always)]
    pub const fn panctr(&self) -> &PANCTR {
        &self.panctr
    }
    #[doc = "0x1c8 - Module Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x1cc - Module Interrupt Trigger Register"]
    #[inline(always)]
    pub const fn mitr(&self) -> &MITR {
        &self.mitr
    }
}
#[doc = "CLC (rw) register accessor: CAN Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clc`]
module"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "CAN Clock Control Register"]
pub mod clc;
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "FDR (rw) register accessor: CAN Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "CAN Fractional Divider Register"]
pub mod fdr;
#[doc = "LIST (r) register accessor: List Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`list::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@list`]
module"]
pub type LIST = crate::Reg<list::LIST_SPEC>;
#[doc = "List Register"]
pub mod list;
#[doc = "MSPND (rw) register accessor: Message Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspnd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspnd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mspnd`]
module"]
pub type MSPND = crate::Reg<mspnd::MSPND_SPEC>;
#[doc = "Message Pending Register"]
pub mod mspnd;
#[doc = "MSID (r) register accessor: Message Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msid`]
module"]
pub type MSID = crate::Reg<msid::MSID_SPEC>;
#[doc = "Message Index Register"]
pub mod msid;
#[doc = "MSIMASK (rw) register accessor: Message Index Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msimask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msimask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msimask`]
module"]
pub type MSIMASK = crate::Reg<msimask::MSIMASK_SPEC>;
#[doc = "Message Index Mask Register"]
pub mod msimask;
#[doc = "PANCTR (rw) register accessor: Panel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`panctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`panctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@panctr`]
module"]
pub type PANCTR = crate::Reg<panctr::PANCTR_SPEC>;
#[doc = "Panel Control Register"]
pub mod panctr;
#[doc = "MCR (rw) register accessor: Module Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "MITR (w) register accessor: Module Interrupt Trigger Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mitr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mitr`]
module"]
pub type MITR = crate::Reg<mitr::MITR_SPEC>;
#[doc = "Module Interrupt Trigger Register"]
pub mod mitr;
