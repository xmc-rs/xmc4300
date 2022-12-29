#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    pub clc: CLC,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Module Identification Register"]
    pub id: ID,
    _reserved2: [u8; 0x1c],
    #[doc = "0x28 - OCDS Control and Status Register"]
    pub ocs: OCS,
    _reserved3: [u8; 0x54],
    #[doc = "0x80 - Global Configuration Register"]
    pub globcfg: GLOBCFG,
    _reserved4: [u8; 0x1c],
    #[doc = "0xa0..0xa8 - Input Class Register, Global"]
    pub globiclass: [GLOBICLASS; 2],
    _reserved5: [u8; 0x10],
    #[doc = "0xb8 - Global Boundary Select Register"]
    pub globbound: GLOBBOUND,
    _reserved6: [u8; 0x24],
    #[doc = "0xe0 - Global Event Flag Register"]
    pub globeflag: GLOBEFLAG,
    _reserved7: [u8; 0x5c],
    #[doc = "0x140 - Global Event Node Pointer Register"]
    pub globevnp: GLOBEVNP,
    _reserved8: [u8; 0x1c],
    #[doc = "0x160 - Global Test Functions Register"]
    pub globtf: GLOBTF,
    _reserved9: [u8; 0x1c],
    #[doc = "0x180..0x188 - Background Request Source Channel Select Register"]
    pub brssel: [BRSSEL; 2],
    _reserved10: [u8; 0x38],
    #[doc = "0x1c0..0x1c8 - Background Request Source Pending Register"]
    pub brspnd: [BRSPND; 2],
    _reserved11: [u8; 0x38],
    #[doc = "0x200 - Background Request Source Control Register"]
    pub brsctrl: BRSCTRL,
    #[doc = "0x204 - Background Request Source Mode Register"]
    pub brsmr: BRSMR,
    _reserved13: [u8; 0x78],
    #[doc = "0x280 - Global Result Control Register"]
    pub globrcr: GLOBRCR,
    _reserved14: [u8; 0x7c],
    #[doc = "0x300 - Global Result Register"]
    pub globres: GLOBRES,
    _reserved15: [u8; 0x7c],
    #[doc = "0x380 - Global Result Register, Debug"]
    pub globresd: GLOBRESD,
    _reserved16: [u8; 0x6c],
    #[doc = "0x3f0 - External Multiplexer Select Register"]
    pub emuxsel: EMUXSEL,
}
#[doc = "CLC (rw) register accessor: an alias for `Reg<CLC_SPEC>`"]
pub type CLC = crate::Reg<clc::CLC_SPEC>;
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "OCS (rw) register accessor: an alias for `Reg<OCS_SPEC>`"]
pub type OCS = crate::Reg<ocs::OCS_SPEC>;
#[doc = "OCDS Control and Status Register"]
pub mod ocs;
#[doc = "GLOBCFG (rw) register accessor: an alias for `Reg<GLOBCFG_SPEC>`"]
pub type GLOBCFG = crate::Reg<globcfg::GLOBCFG_SPEC>;
#[doc = "Global Configuration Register"]
pub mod globcfg;
#[doc = "GLOBICLASS (rw) register accessor: an alias for `Reg<GLOBICLASS_SPEC>`"]
pub type GLOBICLASS = crate::Reg<globiclass::GLOBICLASS_SPEC>;
#[doc = "Input Class Register, Global"]
pub mod globiclass;
#[doc = "GLOBBOUND (rw) register accessor: an alias for `Reg<GLOBBOUND_SPEC>`"]
pub type GLOBBOUND = crate::Reg<globbound::GLOBBOUND_SPEC>;
#[doc = "Global Boundary Select Register"]
pub mod globbound;
#[doc = "GLOBEFLAG (rw) register accessor: an alias for `Reg<GLOBEFLAG_SPEC>`"]
pub type GLOBEFLAG = crate::Reg<globeflag::GLOBEFLAG_SPEC>;
#[doc = "Global Event Flag Register"]
pub mod globeflag;
#[doc = "GLOBEVNP (rw) register accessor: an alias for `Reg<GLOBEVNP_SPEC>`"]
pub type GLOBEVNP = crate::Reg<globevnp::GLOBEVNP_SPEC>;
#[doc = "Global Event Node Pointer Register"]
pub mod globevnp;
#[doc = "GLOBTF (rw) register accessor: an alias for `Reg<GLOBTF_SPEC>`"]
pub type GLOBTF = crate::Reg<globtf::GLOBTF_SPEC>;
#[doc = "Global Test Functions Register"]
pub mod globtf;
#[doc = "BRSSEL (rw) register accessor: an alias for `Reg<BRSSEL_SPEC>`"]
pub type BRSSEL = crate::Reg<brssel::BRSSEL_SPEC>;
#[doc = "Background Request Source Channel Select Register"]
pub mod brssel;
#[doc = "BRSPND (rw) register accessor: an alias for `Reg<BRSPND_SPEC>`"]
pub type BRSPND = crate::Reg<brspnd::BRSPND_SPEC>;
#[doc = "Background Request Source Pending Register"]
pub mod brspnd;
#[doc = "BRSCTRL (rw) register accessor: an alias for `Reg<BRSCTRL_SPEC>`"]
pub type BRSCTRL = crate::Reg<brsctrl::BRSCTRL_SPEC>;
#[doc = "Background Request Source Control Register"]
pub mod brsctrl;
#[doc = "BRSMR (rw) register accessor: an alias for `Reg<BRSMR_SPEC>`"]
pub type BRSMR = crate::Reg<brsmr::BRSMR_SPEC>;
#[doc = "Background Request Source Mode Register"]
pub mod brsmr;
#[doc = "GLOBRCR (rw) register accessor: an alias for `Reg<GLOBRCR_SPEC>`"]
pub type GLOBRCR = crate::Reg<globrcr::GLOBRCR_SPEC>;
#[doc = "Global Result Control Register"]
pub mod globrcr;
#[doc = "GLOBRES (rw) register accessor: an alias for `Reg<GLOBRES_SPEC>`"]
pub type GLOBRES = crate::Reg<globres::GLOBRES_SPEC>;
#[doc = "Global Result Register"]
pub mod globres;
#[doc = "GLOBRESD (rw) register accessor: an alias for `Reg<GLOBRESD_SPEC>`"]
pub type GLOBRESD = crate::Reg<globresd::GLOBRESD_SPEC>;
#[doc = "Global Result Register, Debug"]
pub mod globresd;
#[doc = "EMUXSEL (rw) register accessor: an alias for `Reg<EMUXSEL_SPEC>`"]
pub type EMUXSEL = crate::Reg<emuxsel::EMUXSEL_SPEC>;
#[doc = "External Multiplexer Select Register"]
pub mod emuxsel;
