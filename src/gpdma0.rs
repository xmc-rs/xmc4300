#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Raw IntTfr Status"]
    pub rawtfr: crate::Reg<rawtfr::RAWTFR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Raw IntBlock Status"]
    pub rawblock: crate::Reg<rawblock::RAWBLOCK_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Raw IntSrcTran Status"]
    pub rawsrctran: crate::Reg<rawsrctran::RAWSRCTRAN_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Raw IntBlock Status"]
    pub rawdsttran: crate::Reg<rawdsttran::RAWDSTTRAN_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - Raw IntErr Status"]
    pub rawerr: crate::Reg<rawerr::RAWERR_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - IntTfr Status"]
    pub statustfr: crate::Reg<statustfr::STATUSTFR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - IntBlock Status"]
    pub statusblock: crate::Reg<statusblock::STATUSBLOCK_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x38 - IntSrcTran Status"]
    pub statussrctran: crate::Reg<statussrctran::STATUSSRCTRAN_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - IntBlock Status"]
    pub statusdsttran: crate::Reg<statusdsttran::STATUSDSTTRAN_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x48 - IntErr Status"]
    pub statuserr: crate::Reg<statuserr::STATUSERR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x50 - Mask for Raw IntTfr Status"]
    pub masktfr: crate::Reg<masktfr::MASKTFR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x58 - Mask for Raw IntBlock Status"]
    pub maskblock: crate::Reg<maskblock::MASKBLOCK_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x60 - Mask for Raw IntSrcTran Status"]
    pub masksrctran: crate::Reg<masksrctran::MASKSRCTRAN_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x68 - Mask for Raw IntBlock Status"]
    pub maskdsttran: crate::Reg<maskdsttran::MASKDSTTRAN_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x70 - Mask for Raw IntErr Status"]
    pub maskerr: crate::Reg<maskerr::MASKERR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x78 - IntTfr Status"]
    pub cleartfr: crate::Reg<cleartfr::CLEARTFR_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x80 - IntBlock Status"]
    pub clearblock: crate::Reg<clearblock::CLEARBLOCK_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x88 - IntSrcTran Status"]
    pub clearsrctran: crate::Reg<clearsrctran::CLEARSRCTRAN_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x90 - IntBlock Status"]
    pub cleardsttran: crate::Reg<cleardsttran::CLEARDSTTRAN_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x98 - IntErr Status"]
    pub clearerr: crate::Reg<clearerr::CLEARERR_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0xa0 - Combined Interrupt Status Register"]
    pub statusint: crate::Reg<statusint::STATUSINT_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0xa8 - Source Software Transaction Request Register"]
    pub reqsrcreg: crate::Reg<reqsrcreg::REQSRCREG_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0xb0 - Destination Software Transaction Request Register"]
    pub reqdstreg: crate::Reg<reqdstreg::REQDSTREG_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0xb8 - Single Source Transaction Request Register"]
    pub sglreqsrcreg: crate::Reg<sglreqsrcreg::SGLREQSRCREG_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0xc0 - Single Destination Transaction Request Register"]
    pub sglreqdstreg: crate::Reg<sglreqdstreg::SGLREQDSTREG_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0xc8 - Last Source Transaction Request Register"]
    pub lstsrcreg: crate::Reg<lstsrcreg::LSTSRCREG_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0xd0 - Last Destination Transaction Request Register"]
    pub lstdstreg: crate::Reg<lstdstreg::LSTDSTREG_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0xd8 - GPDMA Configuration Register"]
    pub dmacfgreg: crate::Reg<dmacfgreg::DMACFGREG_SPEC>,
    _reserved28: [u8; 0x04],
    #[doc = "0xe0 - GPDMA Channel Enable Register"]
    pub chenreg: crate::Reg<chenreg::CHENREG_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0xe8 - GPDMA0 ID Register"]
    pub id: crate::Reg<id::ID_SPEC>,
    _reserved30: [u8; 0x4c],
    #[doc = "0x138 - GPDMA Component Type"]
    pub type_: crate::Reg<type_::TYPE_SPEC>,
    #[doc = "0x13c - DMA Component Version"]
    pub version: crate::Reg<version::VERSION_SPEC>,
}
#[doc = "RAWTFR register accessor: an alias for `Reg<RAWTFR_SPEC>`"]
pub type RAWTFR = crate::Reg<rawtfr::RAWTFR_SPEC>;
#[doc = "Raw IntTfr Status"]
pub mod rawtfr;
#[doc = "RAWBLOCK register accessor: an alias for `Reg<RAWBLOCK_SPEC>`"]
pub type RAWBLOCK = crate::Reg<rawblock::RAWBLOCK_SPEC>;
#[doc = "Raw IntBlock Status"]
pub mod rawblock;
#[doc = "RAWSRCTRAN register accessor: an alias for `Reg<RAWSRCTRAN_SPEC>`"]
pub type RAWSRCTRAN = crate::Reg<rawsrctran::RAWSRCTRAN_SPEC>;
#[doc = "Raw IntSrcTran Status"]
pub mod rawsrctran;
#[doc = "RAWDSTTRAN register accessor: an alias for `Reg<RAWDSTTRAN_SPEC>`"]
pub type RAWDSTTRAN = crate::Reg<rawdsttran::RAWDSTTRAN_SPEC>;
#[doc = "Raw IntBlock Status"]
pub mod rawdsttran;
#[doc = "RAWERR register accessor: an alias for `Reg<RAWERR_SPEC>`"]
pub type RAWERR = crate::Reg<rawerr::RAWERR_SPEC>;
#[doc = "Raw IntErr Status"]
pub mod rawerr;
#[doc = "STATUSTFR register accessor: an alias for `Reg<STATUSTFR_SPEC>`"]
pub type STATUSTFR = crate::Reg<statustfr::STATUSTFR_SPEC>;
#[doc = "IntTfr Status"]
pub mod statustfr;
#[doc = "STATUSBLOCK register accessor: an alias for `Reg<STATUSBLOCK_SPEC>`"]
pub type STATUSBLOCK = crate::Reg<statusblock::STATUSBLOCK_SPEC>;
#[doc = "IntBlock Status"]
pub mod statusblock;
#[doc = "STATUSSRCTRAN register accessor: an alias for `Reg<STATUSSRCTRAN_SPEC>`"]
pub type STATUSSRCTRAN = crate::Reg<statussrctran::STATUSSRCTRAN_SPEC>;
#[doc = "IntSrcTran Status"]
pub mod statussrctran;
#[doc = "STATUSDSTTRAN register accessor: an alias for `Reg<STATUSDSTTRAN_SPEC>`"]
pub type STATUSDSTTRAN = crate::Reg<statusdsttran::STATUSDSTTRAN_SPEC>;
#[doc = "IntBlock Status"]
pub mod statusdsttran;
#[doc = "STATUSERR register accessor: an alias for `Reg<STATUSERR_SPEC>`"]
pub type STATUSERR = crate::Reg<statuserr::STATUSERR_SPEC>;
#[doc = "IntErr Status"]
pub mod statuserr;
#[doc = "MASKTFR register accessor: an alias for `Reg<MASKTFR_SPEC>`"]
pub type MASKTFR = crate::Reg<masktfr::MASKTFR_SPEC>;
#[doc = "Mask for Raw IntTfr Status"]
pub mod masktfr;
#[doc = "MASKBLOCK register accessor: an alias for `Reg<MASKBLOCK_SPEC>`"]
pub type MASKBLOCK = crate::Reg<maskblock::MASKBLOCK_SPEC>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskblock;
#[doc = "MASKSRCTRAN register accessor: an alias for `Reg<MASKSRCTRAN_SPEC>`"]
pub type MASKSRCTRAN = crate::Reg<masksrctran::MASKSRCTRAN_SPEC>;
#[doc = "Mask for Raw IntSrcTran Status"]
pub mod masksrctran;
#[doc = "MASKDSTTRAN register accessor: an alias for `Reg<MASKDSTTRAN_SPEC>`"]
pub type MASKDSTTRAN = crate::Reg<maskdsttran::MASKDSTTRAN_SPEC>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskdsttran;
#[doc = "MASKERR register accessor: an alias for `Reg<MASKERR_SPEC>`"]
pub type MASKERR = crate::Reg<maskerr::MASKERR_SPEC>;
#[doc = "Mask for Raw IntErr Status"]
pub mod maskerr;
#[doc = "CLEARTFR register accessor: an alias for `Reg<CLEARTFR_SPEC>`"]
pub type CLEARTFR = crate::Reg<cleartfr::CLEARTFR_SPEC>;
#[doc = "IntTfr Status"]
pub mod cleartfr;
#[doc = "CLEARBLOCK register accessor: an alias for `Reg<CLEARBLOCK_SPEC>`"]
pub type CLEARBLOCK = crate::Reg<clearblock::CLEARBLOCK_SPEC>;
#[doc = "IntBlock Status"]
pub mod clearblock;
#[doc = "CLEARSRCTRAN register accessor: an alias for `Reg<CLEARSRCTRAN_SPEC>`"]
pub type CLEARSRCTRAN = crate::Reg<clearsrctran::CLEARSRCTRAN_SPEC>;
#[doc = "IntSrcTran Status"]
pub mod clearsrctran;
#[doc = "CLEARDSTTRAN register accessor: an alias for `Reg<CLEARDSTTRAN_SPEC>`"]
pub type CLEARDSTTRAN = crate::Reg<cleardsttran::CLEARDSTTRAN_SPEC>;
#[doc = "IntBlock Status"]
pub mod cleardsttran;
#[doc = "CLEARERR register accessor: an alias for `Reg<CLEARERR_SPEC>`"]
pub type CLEARERR = crate::Reg<clearerr::CLEARERR_SPEC>;
#[doc = "IntErr Status"]
pub mod clearerr;
#[doc = "STATUSINT register accessor: an alias for `Reg<STATUSINT_SPEC>`"]
pub type STATUSINT = crate::Reg<statusint::STATUSINT_SPEC>;
#[doc = "Combined Interrupt Status Register"]
pub mod statusint;
#[doc = "REQSRCREG register accessor: an alias for `Reg<REQSRCREG_SPEC>`"]
pub type REQSRCREG = crate::Reg<reqsrcreg::REQSRCREG_SPEC>;
#[doc = "Source Software Transaction Request Register"]
pub mod reqsrcreg;
#[doc = "REQDSTREG register accessor: an alias for `Reg<REQDSTREG_SPEC>`"]
pub type REQDSTREG = crate::Reg<reqdstreg::REQDSTREG_SPEC>;
#[doc = "Destination Software Transaction Request Register"]
pub mod reqdstreg;
#[doc = "SGLREQSRCREG register accessor: an alias for `Reg<SGLREQSRCREG_SPEC>`"]
pub type SGLREQSRCREG = crate::Reg<sglreqsrcreg::SGLREQSRCREG_SPEC>;
#[doc = "Single Source Transaction Request Register"]
pub mod sglreqsrcreg;
#[doc = "SGLREQDSTREG register accessor: an alias for `Reg<SGLREQDSTREG_SPEC>`"]
pub type SGLREQDSTREG = crate::Reg<sglreqdstreg::SGLREQDSTREG_SPEC>;
#[doc = "Single Destination Transaction Request Register"]
pub mod sglreqdstreg;
#[doc = "LSTSRCREG register accessor: an alias for `Reg<LSTSRCREG_SPEC>`"]
pub type LSTSRCREG = crate::Reg<lstsrcreg::LSTSRCREG_SPEC>;
#[doc = "Last Source Transaction Request Register"]
pub mod lstsrcreg;
#[doc = "LSTDSTREG register accessor: an alias for `Reg<LSTDSTREG_SPEC>`"]
pub type LSTDSTREG = crate::Reg<lstdstreg::LSTDSTREG_SPEC>;
#[doc = "Last Destination Transaction Request Register"]
pub mod lstdstreg;
#[doc = "DMACFGREG register accessor: an alias for `Reg<DMACFGREG_SPEC>`"]
pub type DMACFGREG = crate::Reg<dmacfgreg::DMACFGREG_SPEC>;
#[doc = "GPDMA Configuration Register"]
pub mod dmacfgreg;
#[doc = "CHENREG register accessor: an alias for `Reg<CHENREG_SPEC>`"]
pub type CHENREG = crate::Reg<chenreg::CHENREG_SPEC>;
#[doc = "GPDMA Channel Enable Register"]
pub mod chenreg;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "GPDMA0 ID Register"]
pub mod id;
#[doc = "TYPE register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "GPDMA Component Type"]
pub mod type_;
#[doc = "VERSION register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "DMA Component Version"]
pub mod version;
