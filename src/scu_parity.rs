#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    peen: Peen,
    mchkcon: Mchkcon,
    pete: Pete,
    persten: Persten,
    _reserved4: [u8; 0x04],
    peflag: Peflag,
    pmtpr: Pmtpr,
    pmtsr: Pmtsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Parity Error Enable Register"]
    #[inline(always)]
    pub const fn peen(&self) -> &Peen {
        &self.peen
    }
    #[doc = "0x04 - Memory Checking Control Register"]
    #[inline(always)]
    pub const fn mchkcon(&self) -> &Mchkcon {
        &self.mchkcon
    }
    #[doc = "0x08 - Parity Error Trap Enable Register"]
    #[inline(always)]
    pub const fn pete(&self) -> &Pete {
        &self.pete
    }
    #[doc = "0x0c - Parity Error Reset Enable Register"]
    #[inline(always)]
    pub const fn persten(&self) -> &Persten {
        &self.persten
    }
    #[doc = "0x14 - Parity Error Flag Register"]
    #[inline(always)]
    pub const fn peflag(&self) -> &Peflag {
        &self.peflag
    }
    #[doc = "0x18 - Parity Memory Test Pattern Register"]
    #[inline(always)]
    pub const fn pmtpr(&self) -> &Pmtpr {
        &self.pmtpr
    }
    #[doc = "0x1c - Parity Memory Test Select Register"]
    #[inline(always)]
    pub const fn pmtsr(&self) -> &Pmtsr {
        &self.pmtsr
    }
}
#[doc = "PEEN (rw) register accessor: Parity Error Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peen`]
module"]
#[doc(alias = "PEEN")]
pub type Peen = crate::Reg<peen::PeenSpec>;
#[doc = "Parity Error Enable Register"]
pub mod peen;
#[doc = "MCHKCON (rw) register accessor: Memory Checking Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mchkcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mchkcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mchkcon`]
module"]
#[doc(alias = "MCHKCON")]
pub type Mchkcon = crate::Reg<mchkcon::MchkconSpec>;
#[doc = "Memory Checking Control Register"]
pub mod mchkcon;
#[doc = "PETE (rw) register accessor: Parity Error Trap Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pete::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pete::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pete`]
module"]
#[doc(alias = "PETE")]
pub type Pete = crate::Reg<pete::PeteSpec>;
#[doc = "Parity Error Trap Enable Register"]
pub mod pete;
#[doc = "PERSTEN (rw) register accessor: Parity Error Reset Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`persten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`persten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@persten`]
module"]
#[doc(alias = "PERSTEN")]
pub type Persten = crate::Reg<persten::PerstenSpec>;
#[doc = "Parity Error Reset Enable Register"]
pub mod persten;
#[doc = "PEFLAG (rw) register accessor: Parity Error Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peflag`]
module"]
#[doc(alias = "PEFLAG")]
pub type Peflag = crate::Reg<peflag::PeflagSpec>;
#[doc = "Parity Error Flag Register"]
pub mod peflag;
#[doc = "PMTPR (rw) register accessor: Parity Memory Test Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmtpr`]
module"]
#[doc(alias = "PMTPR")]
pub type Pmtpr = crate::Reg<pmtpr::PmtprSpec>;
#[doc = "Parity Memory Test Pattern Register"]
pub mod pmtpr;
#[doc = "PMTSR (rw) register accessor: Parity Memory Test Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmtsr`]
module"]
#[doc(alias = "PMTSR")]
pub type Pmtsr = crate::Reg<pmtsr::PmtsrSpec>;
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;
