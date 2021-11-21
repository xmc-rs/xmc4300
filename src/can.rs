#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Clock Control Register"]
    pub clc: crate::Reg<clc::CLC_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Module Identification Register"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x0c - CAN Fractional Divider Register"]
    pub fdr: crate::Reg<fdr::FDR_SPEC>,
    _reserved3: [u8; 0xf0],
    #[doc = "0x100..0x140 - List Register"]
    pub list: [crate::Reg<list::LIST_SPEC>; 16],
    #[doc = "0x140..0x160 - Message Pending Register"]
    pub mspnd: [crate::Reg<mspnd::MSPND_SPEC>; 8],
    _reserved5: [u8; 0x20],
    #[doc = "0x180..0x1a0 - Message Index Register"]
    pub msid: [crate::Reg<msid::MSID_SPEC>; 8],
    _reserved6: [u8; 0x20],
    #[doc = "0x1c0 - Message Index Mask Register"]
    pub msimask: crate::Reg<msimask::MSIMASK_SPEC>,
    #[doc = "0x1c4 - Panel Control Register"]
    pub panctr: crate::Reg<panctr::PANCTR_SPEC>,
    #[doc = "0x1c8 - Module Control Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x1cc - Module Interrupt Trigger Register"]
    pub mitr: crate::Reg<mitr::MITR_SPEC>,
}
#[doc = "CLC register accessor: an alias for `Reg<CLC_SPEC>`"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "CAN Clock Control Register"]
pub mod clc;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "FDR register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "CAN Fractional Divider Register"]
pub mod fdr;
#[doc = "LIST register accessor: an alias for `Reg<LIST_SPEC>`"]
pub type LIST = crate::Reg<list::LIST_SPEC>;
#[doc = "List Register"]
pub mod list;
#[doc = "MSPND register accessor: an alias for `Reg<MSPND_SPEC>`"]
pub type MSPND = crate::Reg<mspnd::MSPND_SPEC>;
#[doc = "Message Pending Register"]
pub mod mspnd;
#[doc = "MSID register accessor: an alias for `Reg<MSID_SPEC>`"]
pub type MSID = crate::Reg<msid::MSID_SPEC>;
#[doc = "Message Index Register"]
pub mod msid;
#[doc = "MSIMASK register accessor: an alias for `Reg<MSIMASK_SPEC>`"]
pub type MSIMASK = crate::Reg<msimask::MSIMASK_SPEC>;
#[doc = "Message Index Mask Register"]
pub mod msimask;
#[doc = "PANCTR register accessor: an alias for `Reg<PANCTR_SPEC>`"]
pub type PANCTR = crate::Reg<panctr::PANCTR_SPEC>;
#[doc = "Panel Control Register"]
pub mod panctr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "MITR register accessor: an alias for `Reg<MITR_SPEC>`"]
pub type MITR = crate::Reg<mitr::MITR_SPEC>;
#[doc = "Module Interrupt Trigger Register"]
pub mod mitr;
