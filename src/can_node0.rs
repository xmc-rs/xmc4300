#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ncr: NCR,
    nsr: NSR,
    nipr: NIPR,
    npcr: NPCR,
    nbtr: NBTR,
    necnt: NECNT,
    nfcr: NFCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Node Control Register"]
    #[inline(always)]
    pub const fn ncr(&self) -> &NCR {
        &self.ncr
    }
    #[doc = "0x04 - Node Status Register"]
    #[inline(always)]
    pub const fn nsr(&self) -> &NSR {
        &self.nsr
    }
    #[doc = "0x08 - Node Interrupt Pointer Register"]
    #[inline(always)]
    pub const fn nipr(&self) -> &NIPR {
        &self.nipr
    }
    #[doc = "0x0c - Node Port Control Register"]
    #[inline(always)]
    pub const fn npcr(&self) -> &NPCR {
        &self.npcr
    }
    #[doc = "0x10 - Node Bit Timing Register"]
    #[inline(always)]
    pub const fn nbtr(&self) -> &NBTR {
        &self.nbtr
    }
    #[doc = "0x14 - Node Error Counter Register"]
    #[inline(always)]
    pub const fn necnt(&self) -> &NECNT {
        &self.necnt
    }
    #[doc = "0x18 - Node Frame Counter Register"]
    #[inline(always)]
    pub const fn nfcr(&self) -> &NFCR {
        &self.nfcr
    }
}
#[doc = "NCR (rw) register accessor: Node Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ncr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ncr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncr`]
module"]
pub type NCR = crate::Reg<ncr::NCR_SPEC>;
#[doc = "Node Control Register"]
pub mod ncr;
#[doc = "NSR (rw) register accessor: Node Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsr`]
module"]
pub type NSR = crate::Reg<nsr::NSR_SPEC>;
#[doc = "Node Status Register"]
pub mod nsr;
#[doc = "NIPR (rw) register accessor: Node Interrupt Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nipr`]
module"]
pub type NIPR = crate::Reg<nipr::NIPR_SPEC>;
#[doc = "Node Interrupt Pointer Register"]
pub mod nipr;
#[doc = "NPCR (rw) register accessor: Node Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`npcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`npcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@npcr`]
module"]
pub type NPCR = crate::Reg<npcr::NPCR_SPEC>;
#[doc = "Node Port Control Register"]
pub mod npcr;
#[doc = "NBTR (rw) register accessor: Node Bit Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtr`]
module"]
pub type NBTR = crate::Reg<nbtr::NBTR_SPEC>;
#[doc = "Node Bit Timing Register"]
pub mod nbtr;
#[doc = "NECNT (rw) register accessor: Node Error Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`necnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`necnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@necnt`]
module"]
pub type NECNT = crate::Reg<necnt::NECNT_SPEC>;
#[doc = "Node Error Counter Register"]
pub mod necnt;
#[doc = "NFCR (rw) register accessor: Node Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nfcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nfcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfcr`]
module"]
pub type NFCR = crate::Reg<nfcr::NFCR_SPEC>;
#[doc = "Node Frame Counter Register"]
pub mod nfcr;
