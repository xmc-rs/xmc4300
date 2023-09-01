#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCU Status Register"]
    pub pwrstat: PWRSTAT,
    #[doc = "0x04 - PCU Set Control Register"]
    pub pwrset: PWRSET,
    #[doc = "0x08 - PCU Clear Control Register"]
    pub pwrclr: PWRCLR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - EVR Status Register"]
    pub evrstat: EVRSTAT,
    #[doc = "0x14 - EVR VADC Status Register"]
    pub evrvadcstat: EVRVADCSTAT,
    _reserved5: [u8; 0x14],
    #[doc = "0x2c - Power Monitor Control"]
    pub pwrmon: PWRMON,
}
#[doc = "PWRSTAT (r) register accessor: PCU Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwrstat`]
module"]
pub type PWRSTAT = crate::Reg<pwrstat::PWRSTAT_SPEC>;
#[doc = "PCU Status Register"]
pub mod pwrstat;
#[doc = "PWRSET (w) register accessor: PCU Set Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwrset`]
module"]
pub type PWRSET = crate::Reg<pwrset::PWRSET_SPEC>;
#[doc = "PCU Set Control Register"]
pub mod pwrset;
#[doc = "PWRCLR (w) register accessor: PCU Clear Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwrclr`]
module"]
pub type PWRCLR = crate::Reg<pwrclr::PWRCLR_SPEC>;
#[doc = "PCU Clear Control Register"]
pub mod pwrclr;
#[doc = "EVRSTAT (r) register accessor: EVR Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evrstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`evrstat`]
module"]
pub type EVRSTAT = crate::Reg<evrstat::EVRSTAT_SPEC>;
#[doc = "EVR Status Register"]
pub mod evrstat;
#[doc = "EVRVADCSTAT (r) register accessor: EVR VADC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evrvadcstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`evrvadcstat`]
module"]
pub type EVRVADCSTAT = crate::Reg<evrvadcstat::EVRVADCSTAT_SPEC>;
#[doc = "EVR VADC Status Register"]
pub mod evrvadcstat;
#[doc = "PWRMON (rw) register accessor: Power Monitor Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrmon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrmon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pwrmon`]
module"]
pub type PWRMON = crate::Reg<pwrmon::PWRMON_SPEC>;
#[doc = "Power Monitor Control"]
pub mod pwrmon;
