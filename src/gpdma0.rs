#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rawtfr: Rawtfr,
    _reserved1: [u8; 0x04],
    rawblock: Rawblock,
    _reserved2: [u8; 0x04],
    rawsrctran: Rawsrctran,
    _reserved3: [u8; 0x04],
    rawdsttran: Rawdsttran,
    _reserved4: [u8; 0x04],
    rawerr: Rawerr,
    _reserved5: [u8; 0x04],
    statustfr: Statustfr,
    _reserved6: [u8; 0x04],
    statusblock: Statusblock,
    _reserved7: [u8; 0x04],
    statussrctran: Statussrctran,
    _reserved8: [u8; 0x04],
    statusdsttran: Statusdsttran,
    _reserved9: [u8; 0x04],
    statuserr: Statuserr,
    _reserved10: [u8; 0x04],
    masktfr: Masktfr,
    _reserved11: [u8; 0x04],
    maskblock: Maskblock,
    _reserved12: [u8; 0x04],
    masksrctran: Masksrctran,
    _reserved13: [u8; 0x04],
    maskdsttran: Maskdsttran,
    _reserved14: [u8; 0x04],
    maskerr: Maskerr,
    _reserved15: [u8; 0x04],
    cleartfr: Cleartfr,
    _reserved16: [u8; 0x04],
    clearblock: Clearblock,
    _reserved17: [u8; 0x04],
    clearsrctran: Clearsrctran,
    _reserved18: [u8; 0x04],
    cleardsttran: Cleardsttran,
    _reserved19: [u8; 0x04],
    clearerr: Clearerr,
    _reserved20: [u8; 0x04],
    statusint: Statusint,
    _reserved21: [u8; 0x04],
    reqsrcreg: Reqsrcreg,
    _reserved22: [u8; 0x04],
    reqdstreg: Reqdstreg,
    _reserved23: [u8; 0x04],
    sglreqsrcreg: Sglreqsrcreg,
    _reserved24: [u8; 0x04],
    sglreqdstreg: Sglreqdstreg,
    _reserved25: [u8; 0x04],
    lstsrcreg: Lstsrcreg,
    _reserved26: [u8; 0x04],
    lstdstreg: Lstdstreg,
    _reserved27: [u8; 0x04],
    dmacfgreg: Dmacfgreg,
    _reserved28: [u8; 0x04],
    chenreg: Chenreg,
    _reserved29: [u8; 0x04],
    id: Id,
    _reserved30: [u8; 0x4c],
    type_: Type,
    version: Version,
}
impl RegisterBlock {
    #[doc = "0x00 - Raw IntTfr Status"]
    #[inline(always)]
    pub const fn rawtfr(&self) -> &Rawtfr {
        &self.rawtfr
    }
    #[doc = "0x08 - Raw IntBlock Status"]
    #[inline(always)]
    pub const fn rawblock(&self) -> &Rawblock {
        &self.rawblock
    }
    #[doc = "0x10 - Raw IntSrcTran Status"]
    #[inline(always)]
    pub const fn rawsrctran(&self) -> &Rawsrctran {
        &self.rawsrctran
    }
    #[doc = "0x18 - Raw IntBlock Status"]
    #[inline(always)]
    pub const fn rawdsttran(&self) -> &Rawdsttran {
        &self.rawdsttran
    }
    #[doc = "0x20 - Raw IntErr Status"]
    #[inline(always)]
    pub const fn rawerr(&self) -> &Rawerr {
        &self.rawerr
    }
    #[doc = "0x28 - IntTfr Status"]
    #[inline(always)]
    pub const fn statustfr(&self) -> &Statustfr {
        &self.statustfr
    }
    #[doc = "0x30 - IntBlock Status"]
    #[inline(always)]
    pub const fn statusblock(&self) -> &Statusblock {
        &self.statusblock
    }
    #[doc = "0x38 - IntSrcTran Status"]
    #[inline(always)]
    pub const fn statussrctran(&self) -> &Statussrctran {
        &self.statussrctran
    }
    #[doc = "0x40 - IntBlock Status"]
    #[inline(always)]
    pub const fn statusdsttran(&self) -> &Statusdsttran {
        &self.statusdsttran
    }
    #[doc = "0x48 - IntErr Status"]
    #[inline(always)]
    pub const fn statuserr(&self) -> &Statuserr {
        &self.statuserr
    }
    #[doc = "0x50 - Mask for Raw IntTfr Status"]
    #[inline(always)]
    pub const fn masktfr(&self) -> &Masktfr {
        &self.masktfr
    }
    #[doc = "0x58 - Mask for Raw IntBlock Status"]
    #[inline(always)]
    pub const fn maskblock(&self) -> &Maskblock {
        &self.maskblock
    }
    #[doc = "0x60 - Mask for Raw IntSrcTran Status"]
    #[inline(always)]
    pub const fn masksrctran(&self) -> &Masksrctran {
        &self.masksrctran
    }
    #[doc = "0x68 - Mask for Raw IntBlock Status"]
    #[inline(always)]
    pub const fn maskdsttran(&self) -> &Maskdsttran {
        &self.maskdsttran
    }
    #[doc = "0x70 - Mask for Raw IntErr Status"]
    #[inline(always)]
    pub const fn maskerr(&self) -> &Maskerr {
        &self.maskerr
    }
    #[doc = "0x78 - IntTfr Status"]
    #[inline(always)]
    pub const fn cleartfr(&self) -> &Cleartfr {
        &self.cleartfr
    }
    #[doc = "0x80 - IntBlock Status"]
    #[inline(always)]
    pub const fn clearblock(&self) -> &Clearblock {
        &self.clearblock
    }
    #[doc = "0x88 - IntSrcTran Status"]
    #[inline(always)]
    pub const fn clearsrctran(&self) -> &Clearsrctran {
        &self.clearsrctran
    }
    #[doc = "0x90 - IntBlock Status"]
    #[inline(always)]
    pub const fn cleardsttran(&self) -> &Cleardsttran {
        &self.cleardsttran
    }
    #[doc = "0x98 - IntErr Status"]
    #[inline(always)]
    pub const fn clearerr(&self) -> &Clearerr {
        &self.clearerr
    }
    #[doc = "0xa0 - Combined Interrupt Status Register"]
    #[inline(always)]
    pub const fn statusint(&self) -> &Statusint {
        &self.statusint
    }
    #[doc = "0xa8 - Source Software Transaction Request Register"]
    #[inline(always)]
    pub const fn reqsrcreg(&self) -> &Reqsrcreg {
        &self.reqsrcreg
    }
    #[doc = "0xb0 - Destination Software Transaction Request Register"]
    #[inline(always)]
    pub const fn reqdstreg(&self) -> &Reqdstreg {
        &self.reqdstreg
    }
    #[doc = "0xb8 - Single Source Transaction Request Register"]
    #[inline(always)]
    pub const fn sglreqsrcreg(&self) -> &Sglreqsrcreg {
        &self.sglreqsrcreg
    }
    #[doc = "0xc0 - Single Destination Transaction Request Register"]
    #[inline(always)]
    pub const fn sglreqdstreg(&self) -> &Sglreqdstreg {
        &self.sglreqdstreg
    }
    #[doc = "0xc8 - Last Source Transaction Request Register"]
    #[inline(always)]
    pub const fn lstsrcreg(&self) -> &Lstsrcreg {
        &self.lstsrcreg
    }
    #[doc = "0xd0 - Last Destination Transaction Request Register"]
    #[inline(always)]
    pub const fn lstdstreg(&self) -> &Lstdstreg {
        &self.lstdstreg
    }
    #[doc = "0xd8 - GPDMA Configuration Register"]
    #[inline(always)]
    pub const fn dmacfgreg(&self) -> &Dmacfgreg {
        &self.dmacfgreg
    }
    #[doc = "0xe0 - GPDMA Channel Enable Register"]
    #[inline(always)]
    pub const fn chenreg(&self) -> &Chenreg {
        &self.chenreg
    }
    #[doc = "0xe8 - GPDMA0 ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x138 - GPDMA Component Type"]
    #[inline(always)]
    pub const fn type_(&self) -> &Type {
        &self.type_
    }
    #[doc = "0x13c - DMA Component Version"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
}
#[doc = "RAWTFR (rw) register accessor: Raw IntTfr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawtfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawtfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawtfr`]
module"]
#[doc(alias = "RAWTFR")]
pub type Rawtfr = crate::Reg<rawtfr::RawtfrSpec>;
#[doc = "Raw IntTfr Status"]
pub mod rawtfr;
#[doc = "RAWBLOCK (rw) register accessor: Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawblock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawblock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawblock`]
module"]
#[doc(alias = "RAWBLOCK")]
pub type Rawblock = crate::Reg<rawblock::RawblockSpec>;
#[doc = "Raw IntBlock Status"]
pub mod rawblock;
#[doc = "RAWSRCTRAN (rw) register accessor: Raw IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawsrctran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawsrctran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawsrctran`]
module"]
#[doc(alias = "RAWSRCTRAN")]
pub type Rawsrctran = crate::Reg<rawsrctran::RawsrctranSpec>;
#[doc = "Raw IntSrcTran Status"]
pub mod rawsrctran;
#[doc = "RAWDSTTRAN (rw) register accessor: Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawdsttran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawdsttran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawdsttran`]
module"]
#[doc(alias = "RAWDSTTRAN")]
pub type Rawdsttran = crate::Reg<rawdsttran::RawdsttranSpec>;
#[doc = "Raw IntBlock Status"]
pub mod rawdsttran;
#[doc = "RAWERR (rw) register accessor: Raw IntErr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawerr`]
module"]
#[doc(alias = "RAWERR")]
pub type Rawerr = crate::Reg<rawerr::RawerrSpec>;
#[doc = "Raw IntErr Status"]
pub mod rawerr;
#[doc = "STATUSTFR (r) register accessor: IntTfr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statustfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statustfr`]
module"]
#[doc(alias = "STATUSTFR")]
pub type Statustfr = crate::Reg<statustfr::StatustfrSpec>;
#[doc = "IntTfr Status"]
pub mod statustfr;
#[doc = "STATUSBLOCK (r) register accessor: IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusblock::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusblock`]
module"]
#[doc(alias = "STATUSBLOCK")]
pub type Statusblock = crate::Reg<statusblock::StatusblockSpec>;
#[doc = "IntBlock Status"]
pub mod statusblock;
#[doc = "STATUSSRCTRAN (r) register accessor: IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statussrctran::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statussrctran`]
module"]
#[doc(alias = "STATUSSRCTRAN")]
pub type Statussrctran = crate::Reg<statussrctran::StatussrctranSpec>;
#[doc = "IntSrcTran Status"]
pub mod statussrctran;
#[doc = "STATUSDSTTRAN (r) register accessor: IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusdsttran::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusdsttran`]
module"]
#[doc(alias = "STATUSDSTTRAN")]
pub type Statusdsttran = crate::Reg<statusdsttran::StatusdsttranSpec>;
#[doc = "IntBlock Status"]
pub mod statusdsttran;
#[doc = "STATUSERR (r) register accessor: IntErr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statuserr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statuserr`]
module"]
#[doc(alias = "STATUSERR")]
pub type Statuserr = crate::Reg<statuserr::StatuserrSpec>;
#[doc = "IntErr Status"]
pub mod statuserr;
#[doc = "MASKTFR (rw) register accessor: Mask for Raw IntTfr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masktfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masktfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masktfr`]
module"]
#[doc(alias = "MASKTFR")]
pub type Masktfr = crate::Reg<masktfr::MasktfrSpec>;
#[doc = "Mask for Raw IntTfr Status"]
pub mod masktfr;
#[doc = "MASKBLOCK (rw) register accessor: Mask for Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskblock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskblock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskblock`]
module"]
#[doc(alias = "MASKBLOCK")]
pub type Maskblock = crate::Reg<maskblock::MaskblockSpec>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskblock;
#[doc = "MASKSRCTRAN (rw) register accessor: Mask for Raw IntSrcTran Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`masksrctran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`masksrctran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@masksrctran`]
module"]
#[doc(alias = "MASKSRCTRAN")]
pub type Masksrctran = crate::Reg<masksrctran::MasksrctranSpec>;
#[doc = "Mask for Raw IntSrcTran Status"]
pub mod masksrctran;
#[doc = "MASKDSTTRAN (rw) register accessor: Mask for Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskdsttran::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskdsttran::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskdsttran`]
module"]
#[doc(alias = "MASKDSTTRAN")]
pub type Maskdsttran = crate::Reg<maskdsttran::MaskdsttranSpec>;
#[doc = "Mask for Raw IntBlock Status"]
pub mod maskdsttran;
#[doc = "MASKERR (rw) register accessor: Mask for Raw IntErr Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskerr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskerr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskerr`]
module"]
#[doc(alias = "MASKERR")]
pub type Maskerr = crate::Reg<maskerr::MaskerrSpec>;
#[doc = "Mask for Raw IntErr Status"]
pub mod maskerr;
#[doc = "CLEARTFR (w) register accessor: IntTfr Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cleartfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cleartfr`]
module"]
#[doc(alias = "CLEARTFR")]
pub type Cleartfr = crate::Reg<cleartfr::CleartfrSpec>;
#[doc = "IntTfr Status"]
pub mod cleartfr;
#[doc = "CLEARBLOCK (w) register accessor: IntBlock Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearblock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearblock`]
module"]
#[doc(alias = "CLEARBLOCK")]
pub type Clearblock = crate::Reg<clearblock::ClearblockSpec>;
#[doc = "IntBlock Status"]
pub mod clearblock;
#[doc = "CLEARSRCTRAN (w) register accessor: IntSrcTran Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearsrctran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearsrctran`]
module"]
#[doc(alias = "CLEARSRCTRAN")]
pub type Clearsrctran = crate::Reg<clearsrctran::ClearsrctranSpec>;
#[doc = "IntSrcTran Status"]
pub mod clearsrctran;
#[doc = "CLEARDSTTRAN (w) register accessor: IntBlock Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cleardsttran::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cleardsttran`]
module"]
#[doc(alias = "CLEARDSTTRAN")]
pub type Cleardsttran = crate::Reg<cleardsttran::CleardsttranSpec>;
#[doc = "IntBlock Status"]
pub mod cleardsttran;
#[doc = "CLEARERR (w) register accessor: IntErr Status\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearerr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearerr`]
module"]
#[doc(alias = "CLEARERR")]
pub type Clearerr = crate::Reg<clearerr::ClearerrSpec>;
#[doc = "IntErr Status"]
pub mod clearerr;
#[doc = "STATUSINT (r) register accessor: Combined Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statusint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statusint`]
module"]
#[doc(alias = "STATUSINT")]
pub type Statusint = crate::Reg<statusint::StatusintSpec>;
#[doc = "Combined Interrupt Status Register"]
pub mod statusint;
#[doc = "REQSRCREG (rw) register accessor: Source Software Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqsrcreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqsrcreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqsrcreg`]
module"]
#[doc(alias = "REQSRCREG")]
pub type Reqsrcreg = crate::Reg<reqsrcreg::ReqsrcregSpec>;
#[doc = "Source Software Transaction Request Register"]
pub mod reqsrcreg;
#[doc = "REQDSTREG (rw) register accessor: Destination Software Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdstreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdstreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdstreg`]
module"]
#[doc(alias = "REQDSTREG")]
pub type Reqdstreg = crate::Reg<reqdstreg::ReqdstregSpec>;
#[doc = "Destination Software Transaction Request Register"]
pub mod reqdstreg;
#[doc = "SGLREQSRCREG (rw) register accessor: Single Source Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sglreqsrcreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sglreqsrcreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sglreqsrcreg`]
module"]
#[doc(alias = "SGLREQSRCREG")]
pub type Sglreqsrcreg = crate::Reg<sglreqsrcreg::SglreqsrcregSpec>;
#[doc = "Single Source Transaction Request Register"]
pub mod sglreqsrcreg;
#[doc = "SGLREQDSTREG (rw) register accessor: Single Destination Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sglreqdstreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sglreqdstreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sglreqdstreg`]
module"]
#[doc(alias = "SGLREQDSTREG")]
pub type Sglreqdstreg = crate::Reg<sglreqdstreg::SglreqdstregSpec>;
#[doc = "Single Destination Transaction Request Register"]
pub mod sglreqdstreg;
#[doc = "LSTSRCREG (rw) register accessor: Last Source Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstsrcreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstsrcreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lstsrcreg`]
module"]
#[doc(alias = "LSTSRCREG")]
pub type Lstsrcreg = crate::Reg<lstsrcreg::LstsrcregSpec>;
#[doc = "Last Source Transaction Request Register"]
pub mod lstsrcreg;
#[doc = "LSTDSTREG (rw) register accessor: Last Destination Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstdstreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstdstreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lstdstreg`]
module"]
#[doc(alias = "LSTDSTREG")]
pub type Lstdstreg = crate::Reg<lstdstreg::LstdstregSpec>;
#[doc = "Last Destination Transaction Request Register"]
pub mod lstdstreg;
#[doc = "DMACFGREG (rw) register accessor: GPDMA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfgreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfgreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfgreg`]
module"]
#[doc(alias = "DMACFGREG")]
pub type Dmacfgreg = crate::Reg<dmacfgreg::DmacfgregSpec>;
#[doc = "GPDMA Configuration Register"]
pub mod dmacfgreg;
#[doc = "CHENREG (rw) register accessor: GPDMA Channel Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chenreg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chenreg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenreg`]
module"]
#[doc(alias = "CHENREG")]
pub type Chenreg = crate::Reg<chenreg::ChenregSpec>;
#[doc = "GPDMA Channel Enable Register"]
pub mod chenreg;
#[doc = "ID (r) register accessor: GPDMA0 ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "GPDMA0 ID Register"]
pub mod id;
#[doc = "TYPE (r) register accessor: GPDMA Component Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
#[doc(alias = "TYPE")]
pub type Type = crate::Reg<type_::TypeSpec>;
#[doc = "GPDMA Component Type"]
pub mod type_;
#[doc = "VERSION (r) register accessor: DMA Component Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`]
module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "DMA Component Version"]
pub mod version;
