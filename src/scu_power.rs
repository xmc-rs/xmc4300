#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwrstat: Pwrstat,
    pwrset: Pwrset,
    pwrclr: Pwrclr,
    _reserved3: [u8; 0x04],
    evrstat: Evrstat,
    evrvadcstat: Evrvadcstat,
    _reserved5: [u8; 0x14],
    pwrmon: Pwrmon,
}
impl RegisterBlock {
    #[doc = "0x00 - PCU Status Register"]
    #[inline(always)]
    pub const fn pwrstat(&self) -> &Pwrstat {
        &self.pwrstat
    }
    #[doc = "0x04 - PCU Set Control Register"]
    #[inline(always)]
    pub const fn pwrset(&self) -> &Pwrset {
        &self.pwrset
    }
    #[doc = "0x08 - PCU Clear Control Register"]
    #[inline(always)]
    pub const fn pwrclr(&self) -> &Pwrclr {
        &self.pwrclr
    }
    #[doc = "0x10 - EVR Status Register"]
    #[inline(always)]
    pub const fn evrstat(&self) -> &Evrstat {
        &self.evrstat
    }
    #[doc = "0x14 - EVR VADC Status Register"]
    #[inline(always)]
    pub const fn evrvadcstat(&self) -> &Evrvadcstat {
        &self.evrvadcstat
    }
    #[doc = "0x2c - Power Monitor Control"]
    #[inline(always)]
    pub const fn pwrmon(&self) -> &Pwrmon {
        &self.pwrmon
    }
}
#[doc = "PWRSTAT (r) register accessor: PCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrstat`]
module"]
#[doc(alias = "PWRSTAT")]
pub type Pwrstat = crate::Reg<pwrstat::PwrstatSpec>;
#[doc = "PCU Status Register"]
pub mod pwrstat;
#[doc = "PWRSET (w) register accessor: PCU Set Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrset`]
module"]
#[doc(alias = "PWRSET")]
pub type Pwrset = crate::Reg<pwrset::PwrsetSpec>;
#[doc = "PCU Set Control Register"]
pub mod pwrset;
#[doc = "PWRCLR (w) register accessor: PCU Clear Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrclr`]
module"]
#[doc(alias = "PWRCLR")]
pub type Pwrclr = crate::Reg<pwrclr::PwrclrSpec>;
#[doc = "PCU Clear Control Register"]
pub mod pwrclr;
#[doc = "EVRSTAT (r) register accessor: EVR Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evrstat`]
module"]
#[doc(alias = "EVRSTAT")]
pub type Evrstat = crate::Reg<evrstat::EvrstatSpec>;
#[doc = "EVR Status Register"]
pub mod evrstat;
#[doc = "EVRVADCSTAT (r) register accessor: EVR VADC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evrvadcstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evrvadcstat`]
module"]
#[doc(alias = "EVRVADCSTAT")]
pub type Evrvadcstat = crate::Reg<evrvadcstat::EvrvadcstatSpec>;
#[doc = "EVR VADC Status Register"]
pub mod evrvadcstat;
#[doc = "PWRMON (rw) register accessor: Power Monitor Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrmon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrmon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrmon`]
module"]
#[doc(alias = "PWRMON")]
pub type Pwrmon = crate::Reg<pwrmon::PwrmonSpec>;
#[doc = "Power Monitor Control"]
pub mod pwrmon;
