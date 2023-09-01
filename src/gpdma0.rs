#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Raw IntTfr Status"]
    pub rawtfr: RAWTFR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Raw IntBlock Status"]
    pub rawblock: RAWBLOCK,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Raw IntSrcTran Status"]
    pub rawsrctran: RAWSRCTRAN,
    _reserved3: [u8; 0x04],
    #[doc = "0x18 - Raw IntBlock Status"]
    pub rawdsttran: RAWDSTTRAN,
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - Raw IntErr Status"]
    pub rawerr: RAWERR,
    _reserved5: [u8; 0x04],
    #[doc = "0x28 - IntTfr Status"]
    pub statustfr: STATUSTFR,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - IntBlock Status"]
    pub statusblock: STATUSBLOCK,
    _reserved7: [u8; 0x04],
    #[doc = "0x38 - IntSrcTran Status"]
    pub statussrctran: STATUSSRCTRAN,
    _reserved8: [u8; 0x04],
    #[doc = "0x40 - IntBlock Status"]
    pub statusdsttran: STATUSDSTTRAN,
    _reserved9: [u8; 0x04],
    #[doc = "0x48 - IntErr Status"]
    pub statuserr: STATUSERR,
    _reserved10: [u8; 0x04],
    #[doc = "0x50 - Mask for Raw IntTfr Status"]
    pub masktfr: MASKTFR,
    _reserved11: [u8; 0x04],
    #[doc = "0x58 - Mask for Raw IntBlock Status"]
    pub maskblock: MASKBLOCK,
    _reserved12: [u8; 0x04],
    #[doc = "0x60 - Mask for Raw IntSrcTran Status"]
    pub masksrctran: MASKSRCTRAN,
    _reserved13: [u8; 0x04],
    #[doc = "0x68 - Mask for Raw IntBlock Status"]
    pub maskdsttran: MASKDSTTRAN,
    _reserved14: [u8; 0x04],
    #[doc = "0x70 - Mask for Raw IntErr Status"]
    pub maskerr: MASKERR,
    _reserved15: [u8; 0x04],
    #[doc = "0x78 - IntTfr Status"]
    pub cleartfr: CLEARTFR,
    _reserved16: [u8; 0x04],
    #[doc = "0x80 - IntBlock Status"]
    pub clearblock: CLEARBLOCK,
    _reserved17: [u8; 0x04],
    #[doc = "0x88 - IntSrcTran Status"]
    pub clearsrctran: CLEARSRCTRAN,
    _reserved18: [u8; 0x04],
    #[doc = "0x90 - IntBlock Status"]
    pub cleardsttran: CLEARDSTTRAN,
    _reserved19: [u8; 0x04],
    #[doc = "0x98 - IntErr Status"]
    pub clearerr: CLEARERR,
    _reserved20: [u8; 0x04],
    #[doc = "0xa0 - Combined Interrupt Status Register"]
    pub statusint: STATUSINT,
    _reserved21: [u8; 0x04],
    #[doc = "0xa8 - Source Software Transaction Request Register"]
    pub reqsrcreg: REQSRCREG,
    _reserved22: [u8; 0x04],
    #[doc = "0xb0 - Destination Software Transaction Request Register"]
    pub reqdstreg: REQDSTREG,
    _reserved23: [u8; 0x04],
    #[doc = "0xb8 - Single Source Transaction Request Register"]
    pub sglreqsrcreg: SGLREQSRCREG,
    _reserved24: [u8; 0x04],
    #[doc = "0xc0 - Single Destination Transaction Request Register"]
    pub sglreqdstreg: SGLREQDSTREG,
    _reserved25: [u8; 0x04],
    #[doc = "0xc8 - Last Source Transaction Request Register"]
    pub lstsrcreg: LSTSRCREG,
    _reserved26: [u8; 0x04],
    #[doc = "0xd0 - Last Destination Transaction Request Register"]
    pub lstdstreg: LSTDSTREG,
    _reserved27: [u8; 0x04],
    #[doc = "0xd8 - GPDMA Configuration Register"]
    pub dmacfgreg: DMACFGREG,
    _reserved28: [u8; 0x04],
    #[doc = "0xe0 - GPDMA Channel Enable Register"]
    pub chenreg: CHENREG,
    _reserved29: [u8; 0x04],
    #[doc = "0xe8 - GPDMA0 ID Register"]
    pub id: ID,
    _reserved30: [u8; 0x4c],
    #[doc = "0x138 - GPDMA Component Type"]
    pub type_: TYPE,
    #[doc = "0x13c - DMA Component Version"]
    pub version: VERSION,
}
#[doc = "RAWTFR (rw) register accessor: Raw IntTfr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawtfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawtfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rawtfr`]
module"]
pub type RAWTFR = crate::Reg<rawtfr::RAWTFR_SPEC>;
#[doc = "Raw IntTfr Status"]
pub mod rawtfr;
#[doc = "RAWBLOCK (rw) register accessor: Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawblock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawblock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rawblock`]
module"]
pub type RAWBLOCK = crate::Reg<rawblock::RAWBLOCK_SPEC>;
#[doc = "Raw IntBlock Status"]
pub mod rawblock;
#[doc = "RAWSRCTRAN (rw) register accessor: Raw IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawsrctran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawsrctran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rawsrctran`]
module"]
pub type RAWSRCTRAN = crate::Reg<rawsrctran::RAWSRCTRAN_SPEC>;
#[doc = "Raw IntSrcTran Status"]
pub mod rawsrctran;
#[doc = "RAWDSTTRAN (rw) register accessor: Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawdsttran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawdsttran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rawdsttran`]
module"]
pub type RAWDSTTRAN = crate::Reg<rawdsttran::RAWDSTTRAN_SPEC>;
#[doc = "Raw IntBlock Status"]
pub mod rawdsttran;
#[doc = "RAWERR (rw) register accessor: Raw IntErr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rawerr`]
module"]
pub type RAWERR = crate::Reg<rawerr::RAWERR_SPEC>;
#[doc = "Raw IntErr Status"]
pub mod rawerr;
#[doc = "STATUSTFR (r) register accessor: IntTfr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statustfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statustfr`]
module"]
pub type STATUSTFR = crate::Reg<statustfr::STATUSTFR_SPEC>;
#[doc = "IntTfr Status"]
pub mod statustfr;
#[doc = "STATUSBLOCK (r) register accessor: IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusblock::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statusblock`]
module"]
pub type STATUSBLOCK = crate::Reg<statusblock::STATUSBLOCK_SPEC>;
#[doc = "IntBlock Status"]
pub mod statusblock;
#[doc = "STATUSSRCTRAN (r) register accessor: IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statussrctran::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statussrctran`]
module"]
pub type STATUSSRCTRAN = crate::Reg<statussrctran::STATUSSRCTRAN_SPEC>;
#[doc = "IntSrcTran Status"]
pub mod statussrctran;
#[doc = "STATUSDSTTRAN (r) register accessor: IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusdsttran::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statusdsttran`]
module"]
pub type STATUSDSTTRAN = crate::Reg<statusdsttran::STATUSDSTTRAN_SPEC>;
#[doc = "IntBlock Status"]
pub mod statusdsttran;
#[doc = "STATUSERR (r) register accessor: IntErr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statuserr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statuserr`]
module"]
pub type STATUSERR = crate::Reg<statuserr::STATUSERR_SPEC>;
#[doc = "IntErr Status"]
pub mod statuserr;
#[doc = "MASKTFR (rw) register accessor: Mask for Raw IntTfr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masktfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masktfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`masktfr`]
module"]
pub type MASKTFR = crate::Reg<masktfr::MASKTFR_SPEC>;
#[doc = "Mask for Raw IntTfr Status"]
pub mod masktfr;
#[doc = "MASKBLOCK (rw) register accessor: Mask for Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskblock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskblock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maskblock`]
module"]
pub type MASKBLOCK = crate::Reg<maskblock::MASKBLOCK_SPEC>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskblock;
#[doc = "MASKSRCTRAN (rw) register accessor: Mask for Raw IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masksrctran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masksrctran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`masksrctran`]
module"]
pub type MASKSRCTRAN = crate::Reg<masksrctran::MASKSRCTRAN_SPEC>;
#[doc = "Mask for Raw IntSrcTran Status"]
pub mod masksrctran;
#[doc = "MASKDSTTRAN (rw) register accessor: Mask for Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskdsttran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskdsttran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maskdsttran`]
module"]
pub type MASKDSTTRAN = crate::Reg<maskdsttran::MASKDSTTRAN_SPEC>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskdsttran;
#[doc = "MASKERR (rw) register accessor: Mask for Raw IntErr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`maskerr`]
module"]
pub type MASKERR = crate::Reg<maskerr::MASKERR_SPEC>;
#[doc = "Mask for Raw IntErr Status"]
pub mod maskerr;
#[doc = "CLEARTFR (w) register accessor: IntTfr Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cleartfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cleartfr`]
module"]
pub type CLEARTFR = crate::Reg<cleartfr::CLEARTFR_SPEC>;
#[doc = "IntTfr Status"]
pub mod cleartfr;
#[doc = "CLEARBLOCK (w) register accessor: IntBlock Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearblock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clearblock`]
module"]
pub type CLEARBLOCK = crate::Reg<clearblock::CLEARBLOCK_SPEC>;
#[doc = "IntBlock Status"]
pub mod clearblock;
#[doc = "CLEARSRCTRAN (w) register accessor: IntSrcTran Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearsrctran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clearsrctran`]
module"]
pub type CLEARSRCTRAN = crate::Reg<clearsrctran::CLEARSRCTRAN_SPEC>;
#[doc = "IntSrcTran Status"]
pub mod clearsrctran;
#[doc = "CLEARDSTTRAN (w) register accessor: IntBlock Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cleardsttran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cleardsttran`]
module"]
pub type CLEARDSTTRAN = crate::Reg<cleardsttran::CLEARDSTTRAN_SPEC>;
#[doc = "IntBlock Status"]
pub mod cleardsttran;
#[doc = "CLEARERR (w) register accessor: IntErr Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearerr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clearerr`]
module"]
pub type CLEARERR = crate::Reg<clearerr::CLEARERR_SPEC>;
#[doc = "IntErr Status"]
pub mod clearerr;
#[doc = "STATUSINT (r) register accessor: Combined Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`statusint`]
module"]
pub type STATUSINT = crate::Reg<statusint::STATUSINT_SPEC>;
#[doc = "Combined Interrupt Status Register"]
pub mod statusint;
#[doc = "REQSRCREG (rw) register accessor: Source Software Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqsrcreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqsrcreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reqsrcreg`]
module"]
pub type REQSRCREG = crate::Reg<reqsrcreg::REQSRCREG_SPEC>;
#[doc = "Source Software Transaction Request Register"]
pub mod reqsrcreg;
#[doc = "REQDSTREG (rw) register accessor: Destination Software Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdstreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdstreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reqdstreg`]
module"]
pub type REQDSTREG = crate::Reg<reqdstreg::REQDSTREG_SPEC>;
#[doc = "Destination Software Transaction Request Register"]
pub mod reqdstreg;
#[doc = "SGLREQSRCREG (rw) register accessor: Single Source Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sglreqsrcreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sglreqsrcreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sglreqsrcreg`]
module"]
pub type SGLREQSRCREG = crate::Reg<sglreqsrcreg::SGLREQSRCREG_SPEC>;
#[doc = "Single Source Transaction Request Register"]
pub mod sglreqsrcreg;
#[doc = "SGLREQDSTREG (rw) register accessor: Single Destination Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sglreqdstreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sglreqdstreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sglreqdstreg`]
module"]
pub type SGLREQDSTREG = crate::Reg<sglreqdstreg::SGLREQDSTREG_SPEC>;
#[doc = "Single Destination Transaction Request Register"]
pub mod sglreqdstreg;
#[doc = "LSTSRCREG (rw) register accessor: Last Source Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstsrcreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstsrcreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lstsrcreg`]
module"]
pub type LSTSRCREG = crate::Reg<lstsrcreg::LSTSRCREG_SPEC>;
#[doc = "Last Source Transaction Request Register"]
pub mod lstsrcreg;
#[doc = "LSTDSTREG (rw) register accessor: Last Destination Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstdstreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstdstreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lstdstreg`]
module"]
pub type LSTDSTREG = crate::Reg<lstdstreg::LSTDSTREG_SPEC>;
#[doc = "Last Destination Transaction Request Register"]
pub mod lstdstreg;
#[doc = "DMACFGREG (rw) register accessor: GPDMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfgreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfgreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmacfgreg`]
module"]
pub type DMACFGREG = crate::Reg<dmacfgreg::DMACFGREG_SPEC>;
#[doc = "GPDMA Configuration Register"]
pub mod dmacfgreg;
#[doc = "CHENREG (rw) register accessor: GPDMA Channel Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chenreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chenreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`chenreg`]
module"]
pub type CHENREG = crate::Reg<chenreg::CHENREG_SPEC>;
#[doc = "GPDMA Channel Enable Register"]
pub mod chenreg;
#[doc = "ID (r) register accessor: GPDMA0 ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "GPDMA0 ID Register"]
pub mod id;
#[doc = "TYPE (r) register accessor: GPDMA Component Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`type_`]
module"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "GPDMA Component Type"]
pub mod type_;
#[doc = "VERSION (r) register accessor: DMA Component Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`version`]
module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "DMA Component Version"]
pub mod version;
