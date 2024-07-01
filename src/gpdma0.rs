#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rawtfr: RAWTFR,
    _reserved1: [u8; 0x04],
    rawblock: RAWBLOCK,
    _reserved2: [u8; 0x04],
    rawsrctran: RAWSRCTRAN,
    _reserved3: [u8; 0x04],
    rawdsttran: RAWDSTTRAN,
    _reserved4: [u8; 0x04],
    rawerr: RAWERR,
    _reserved5: [u8; 0x04],
    statustfr: STATUSTFR,
    _reserved6: [u8; 0x04],
    statusblock: STATUSBLOCK,
    _reserved7: [u8; 0x04],
    statussrctran: STATUSSRCTRAN,
    _reserved8: [u8; 0x04],
    statusdsttran: STATUSDSTTRAN,
    _reserved9: [u8; 0x04],
    statuserr: STATUSERR,
    _reserved10: [u8; 0x04],
    masktfr: MASKTFR,
    _reserved11: [u8; 0x04],
    maskblock: MASKBLOCK,
    _reserved12: [u8; 0x04],
    masksrctran: MASKSRCTRAN,
    _reserved13: [u8; 0x04],
    maskdsttran: MASKDSTTRAN,
    _reserved14: [u8; 0x04],
    maskerr: MASKERR,
    _reserved15: [u8; 0x04],
    cleartfr: CLEARTFR,
    _reserved16: [u8; 0x04],
    clearblock: CLEARBLOCK,
    _reserved17: [u8; 0x04],
    clearsrctran: CLEARSRCTRAN,
    _reserved18: [u8; 0x04],
    cleardsttran: CLEARDSTTRAN,
    _reserved19: [u8; 0x04],
    clearerr: CLEARERR,
    _reserved20: [u8; 0x04],
    statusint: STATUSINT,
    _reserved21: [u8; 0x04],
    reqsrcreg: REQSRCREG,
    _reserved22: [u8; 0x04],
    reqdstreg: REQDSTREG,
    _reserved23: [u8; 0x04],
    sglreqsrcreg: SGLREQSRCREG,
    _reserved24: [u8; 0x04],
    sglreqdstreg: SGLREQDSTREG,
    _reserved25: [u8; 0x04],
    lstsrcreg: LSTSRCREG,
    _reserved26: [u8; 0x04],
    lstdstreg: LSTDSTREG,
    _reserved27: [u8; 0x04],
    dmacfgreg: DMACFGREG,
    _reserved28: [u8; 0x04],
    chenreg: CHENREG,
    _reserved29: [u8; 0x04],
    id: ID,
    _reserved30: [u8; 0x4c],
    type_: TYPE,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - Raw IntTfr Status"]
    #[inline(always)]
    pub const fn rawtfr(&self) -> &RAWTFR {
        &self.rawtfr
    }
    #[doc = "0x08 - Raw IntBlock Status"]
    #[inline(always)]
    pub const fn rawblock(&self) -> &RAWBLOCK {
        &self.rawblock
    }
    #[doc = "0x10 - Raw IntSrcTran Status"]
    #[inline(always)]
    pub const fn rawsrctran(&self) -> &RAWSRCTRAN {
        &self.rawsrctran
    }
    #[doc = "0x18 - Raw IntBlock Status"]
    #[inline(always)]
    pub const fn rawdsttran(&self) -> &RAWDSTTRAN {
        &self.rawdsttran
    }
    #[doc = "0x20 - Raw IntErr Status"]
    #[inline(always)]
    pub const fn rawerr(&self) -> &RAWERR {
        &self.rawerr
    }
    #[doc = "0x28 - IntTfr Status"]
    #[inline(always)]
    pub const fn statustfr(&self) -> &STATUSTFR {
        &self.statustfr
    }
    #[doc = "0x30 - IntBlock Status"]
    #[inline(always)]
    pub const fn statusblock(&self) -> &STATUSBLOCK {
        &self.statusblock
    }
    #[doc = "0x38 - IntSrcTran Status"]
    #[inline(always)]
    pub const fn statussrctran(&self) -> &STATUSSRCTRAN {
        &self.statussrctran
    }
    #[doc = "0x40 - IntBlock Status"]
    #[inline(always)]
    pub const fn statusdsttran(&self) -> &STATUSDSTTRAN {
        &self.statusdsttran
    }
    #[doc = "0x48 - IntErr Status"]
    #[inline(always)]
    pub const fn statuserr(&self) -> &STATUSERR {
        &self.statuserr
    }
    #[doc = "0x50 - Mask for Raw IntTfr Status"]
    #[inline(always)]
    pub const fn masktfr(&self) -> &MASKTFR {
        &self.masktfr
    }
    #[doc = "0x58 - Mask for Raw IntBlock Status"]
    #[inline(always)]
    pub const fn maskblock(&self) -> &MASKBLOCK {
        &self.maskblock
    }
    #[doc = "0x60 - Mask for Raw IntSrcTran Status"]
    #[inline(always)]
    pub const fn masksrctran(&self) -> &MASKSRCTRAN {
        &self.masksrctran
    }
    #[doc = "0x68 - Mask for Raw IntBlock Status"]
    #[inline(always)]
    pub const fn maskdsttran(&self) -> &MASKDSTTRAN {
        &self.maskdsttran
    }
    #[doc = "0x70 - Mask for Raw IntErr Status"]
    #[inline(always)]
    pub const fn maskerr(&self) -> &MASKERR {
        &self.maskerr
    }
    #[doc = "0x78 - IntTfr Status"]
    #[inline(always)]
    pub const fn cleartfr(&self) -> &CLEARTFR {
        &self.cleartfr
    }
    #[doc = "0x80 - IntBlock Status"]
    #[inline(always)]
    pub const fn clearblock(&self) -> &CLEARBLOCK {
        &self.clearblock
    }
    #[doc = "0x88 - IntSrcTran Status"]
    #[inline(always)]
    pub const fn clearsrctran(&self) -> &CLEARSRCTRAN {
        &self.clearsrctran
    }
    #[doc = "0x90 - IntBlock Status"]
    #[inline(always)]
    pub const fn cleardsttran(&self) -> &CLEARDSTTRAN {
        &self.cleardsttran
    }
    #[doc = "0x98 - IntErr Status"]
    #[inline(always)]
    pub const fn clearerr(&self) -> &CLEARERR {
        &self.clearerr
    }
    #[doc = "0xa0 - Combined Interrupt Status Register"]
    #[inline(always)]
    pub const fn statusint(&self) -> &STATUSINT {
        &self.statusint
    }
    #[doc = "0xa8 - Source Software Transaction Request Register"]
    #[inline(always)]
    pub const fn reqsrcreg(&self) -> &REQSRCREG {
        &self.reqsrcreg
    }
    #[doc = "0xb0 - Destination Software Transaction Request Register"]
    #[inline(always)]
    pub const fn reqdstreg(&self) -> &REQDSTREG {
        &self.reqdstreg
    }
    #[doc = "0xb8 - Single Source Transaction Request Register"]
    #[inline(always)]
    pub const fn sglreqsrcreg(&self) -> &SGLREQSRCREG {
        &self.sglreqsrcreg
    }
    #[doc = "0xc0 - Single Destination Transaction Request Register"]
    #[inline(always)]
    pub const fn sglreqdstreg(&self) -> &SGLREQDSTREG {
        &self.sglreqdstreg
    }
    #[doc = "0xc8 - Last Source Transaction Request Register"]
    #[inline(always)]
    pub const fn lstsrcreg(&self) -> &LSTSRCREG {
        &self.lstsrcreg
    }
    #[doc = "0xd0 - Last Destination Transaction Request Register"]
    #[inline(always)]
    pub const fn lstdstreg(&self) -> &LSTDSTREG {
        &self.lstdstreg
    }
    #[doc = "0xd8 - GPDMA Configuration Register"]
    #[inline(always)]
    pub const fn dmacfgreg(&self) -> &DMACFGREG {
        &self.dmacfgreg
    }
    #[doc = "0xe0 - GPDMA Channel Enable Register"]
    #[inline(always)]
    pub const fn chenreg(&self) -> &CHENREG {
        &self.chenreg
    }
    #[doc = "0xe8 - GPDMA0 ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x138 - GPDMA Component Type"]
    #[inline(always)]
    pub const fn type_(&self) -> &TYPE {
        &self.type_
    }
    #[doc = "0x13c - DMA Component Version"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "RAWTFR (rw) register accessor: Raw IntTfr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rawtfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawtfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawtfr`]
module"]
pub type RAWTFR = crate::Reg<rawtfr::RAWTFR_SPEC>;
#[doc = "Raw IntTfr Status"]
pub mod rawtfr;
#[doc = "RAWBLOCK (rw) register accessor: Raw IntBlock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rawblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawblock`]
module"]
pub type RAWBLOCK = crate::Reg<rawblock::RAWBLOCK_SPEC>;
#[doc = "Raw IntBlock Status"]
pub mod rawblock;
#[doc = "RAWSRCTRAN (rw) register accessor: Raw IntSrcTran Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rawsrctran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawsrctran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawsrctran`]
module"]
pub type RAWSRCTRAN = crate::Reg<rawsrctran::RAWSRCTRAN_SPEC>;
#[doc = "Raw IntSrcTran Status"]
pub mod rawsrctran;
#[doc = "RAWDSTTRAN (rw) register accessor: Raw IntBlock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rawdsttran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawdsttran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawdsttran`]
module"]
pub type RAWDSTTRAN = crate::Reg<rawdsttran::RAWDSTTRAN_SPEC>;
#[doc = "Raw IntBlock Status"]
pub mod rawdsttran;
#[doc = "RAWERR (rw) register accessor: Raw IntErr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`rawerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rawerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawerr`]
module"]
pub type RAWERR = crate::Reg<rawerr::RAWERR_SPEC>;
#[doc = "Raw IntErr Status"]
pub mod rawerr;
#[doc = "STATUSTFR (r) register accessor: IntTfr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`statustfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statustfr`]
module"]
pub type STATUSTFR = crate::Reg<statustfr::STATUSTFR_SPEC>;
#[doc = "IntTfr Status"]
pub mod statustfr;
#[doc = "STATUSBLOCK (r) register accessor: IntBlock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`statusblock::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusblock`]
module"]
pub type STATUSBLOCK = crate::Reg<statusblock::STATUSBLOCK_SPEC>;
#[doc = "IntBlock Status"]
pub mod statusblock;
#[doc = "STATUSSRCTRAN (r) register accessor: IntSrcTran Status\n\nYou can [`read`](crate::Reg::read) this register and get [`statussrctran::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statussrctran`]
module"]
pub type STATUSSRCTRAN = crate::Reg<statussrctran::STATUSSRCTRAN_SPEC>;
#[doc = "IntSrcTran Status"]
pub mod statussrctran;
#[doc = "STATUSDSTTRAN (r) register accessor: IntBlock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`statusdsttran::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusdsttran`]
module"]
pub type STATUSDSTTRAN = crate::Reg<statusdsttran::STATUSDSTTRAN_SPEC>;
#[doc = "IntBlock Status"]
pub mod statusdsttran;
#[doc = "STATUSERR (r) register accessor: IntErr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`statuserr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statuserr`]
module"]
pub type STATUSERR = crate::Reg<statuserr::STATUSERR_SPEC>;
#[doc = "IntErr Status"]
pub mod statuserr;
#[doc = "MASKTFR (rw) register accessor: Mask for Raw IntTfr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`masktfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masktfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masktfr`]
module"]
pub type MASKTFR = crate::Reg<masktfr::MASKTFR_SPEC>;
#[doc = "Mask for Raw IntTfr Status"]
pub mod masktfr;
#[doc = "MASKBLOCK (rw) register accessor: Mask for Raw IntBlock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`maskblock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskblock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskblock`]
module"]
pub type MASKBLOCK = crate::Reg<maskblock::MASKBLOCK_SPEC>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskblock;
#[doc = "MASKSRCTRAN (rw) register accessor: Mask for Raw IntSrcTran Status\n\nYou can [`read`](crate::Reg::read) this register and get [`masksrctran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`masksrctran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masksrctran`]
module"]
pub type MASKSRCTRAN = crate::Reg<masksrctran::MASKSRCTRAN_SPEC>;
#[doc = "Mask for Raw IntSrcTran Status"]
pub mod masksrctran;
#[doc = "MASKDSTTRAN (rw) register accessor: Mask for Raw IntBlock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`maskdsttran::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskdsttran::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskdsttran`]
module"]
pub type MASKDSTTRAN = crate::Reg<maskdsttran::MASKDSTTRAN_SPEC>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskdsttran;
#[doc = "MASKERR (rw) register accessor: Mask for Raw IntErr Status\n\nYou can [`read`](crate::Reg::read) this register and get [`maskerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskerr`]
module"]
pub type MASKERR = crate::Reg<maskerr::MASKERR_SPEC>;
#[doc = "Mask for Raw IntErr Status"]
pub mod maskerr;
#[doc = "CLEARTFR (w) register accessor: IntTfr Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cleartfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cleartfr`]
module"]
pub type CLEARTFR = crate::Reg<cleartfr::CLEARTFR_SPEC>;
#[doc = "IntTfr Status"]
pub mod cleartfr;
#[doc = "CLEARBLOCK (w) register accessor: IntBlock Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearblock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearblock`]
module"]
pub type CLEARBLOCK = crate::Reg<clearblock::CLEARBLOCK_SPEC>;
#[doc = "IntBlock Status"]
pub mod clearblock;
#[doc = "CLEARSRCTRAN (w) register accessor: IntSrcTran Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearsrctran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearsrctran`]
module"]
pub type CLEARSRCTRAN = crate::Reg<clearsrctran::CLEARSRCTRAN_SPEC>;
#[doc = "IntSrcTran Status"]
pub mod clearsrctran;
#[doc = "CLEARDSTTRAN (w) register accessor: IntBlock Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cleardsttran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cleardsttran`]
module"]
pub type CLEARDSTTRAN = crate::Reg<cleardsttran::CLEARDSTTRAN_SPEC>;
#[doc = "IntBlock Status"]
pub mod cleardsttran;
#[doc = "CLEARERR (w) register accessor: IntErr Status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearerr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearerr`]
module"]
pub type CLEARERR = crate::Reg<clearerr::CLEARERR_SPEC>;
#[doc = "IntErr Status"]
pub mod clearerr;
#[doc = "STATUSINT (r) register accessor: Combined Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`statusint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusint`]
module"]
pub type STATUSINT = crate::Reg<statusint::STATUSINT_SPEC>;
#[doc = "Combined Interrupt Status Register"]
pub mod statusint;
#[doc = "REQSRCREG (rw) register accessor: Source Software Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reqsrcreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqsrcreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqsrcreg`]
module"]
pub type REQSRCREG = crate::Reg<reqsrcreg::REQSRCREG_SPEC>;
#[doc = "Source Software Transaction Request Register"]
pub mod reqsrcreg;
#[doc = "REQDSTREG (rw) register accessor: Destination Software Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reqdstreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdstreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdstreg`]
module"]
pub type REQDSTREG = crate::Reg<reqdstreg::REQDSTREG_SPEC>;
#[doc = "Destination Software Transaction Request Register"]
pub mod reqdstreg;
#[doc = "SGLREQSRCREG (rw) register accessor: Single Source Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sglreqsrcreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sglreqsrcreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sglreqsrcreg`]
module"]
pub type SGLREQSRCREG = crate::Reg<sglreqsrcreg::SGLREQSRCREG_SPEC>;
#[doc = "Single Source Transaction Request Register"]
pub mod sglreqsrcreg;
#[doc = "SGLREQDSTREG (rw) register accessor: Single Destination Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sglreqdstreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sglreqdstreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sglreqdstreg`]
module"]
pub type SGLREQDSTREG = crate::Reg<sglreqdstreg::SGLREQDSTREG_SPEC>;
#[doc = "Single Destination Transaction Request Register"]
pub mod sglreqdstreg;
#[doc = "LSTSRCREG (rw) register accessor: Last Source Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lstsrcreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lstsrcreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lstsrcreg`]
module"]
pub type LSTSRCREG = crate::Reg<lstsrcreg::LSTSRCREG_SPEC>;
#[doc = "Last Source Transaction Request Register"]
pub mod lstsrcreg;
#[doc = "LSTDSTREG (rw) register accessor: Last Destination Transaction Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lstdstreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lstdstreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lstdstreg`]
module"]
pub type LSTDSTREG = crate::Reg<lstdstreg::LSTDSTREG_SPEC>;
#[doc = "Last Destination Transaction Request Register"]
pub mod lstdstreg;
#[doc = "DMACFGREG (rw) register accessor: GPDMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfgreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfgreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfgreg`]
module"]
pub type DMACFGREG = crate::Reg<dmacfgreg::DMACFGREG_SPEC>;
#[doc = "GPDMA Configuration Register"]
pub mod dmacfgreg;
#[doc = "CHENREG (rw) register accessor: GPDMA Channel Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chenreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenreg`]
module"]
pub type CHENREG = crate::Reg<chenreg::CHENREG_SPEC>;
#[doc = "GPDMA Channel Enable Register"]
pub mod chenreg;
#[doc = "ID (r) register accessor: GPDMA0 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "GPDMA0 ID Register"]
pub mod id;
#[doc = "TYPE (r) register accessor: GPDMA Component Type\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "GPDMA Component Type"]
pub mod type_;
#[doc = "VERSION (r) register accessor: DMA Component Version\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "DMA Component Version"]
pub mod version;
