#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Parity Error Enable Register"]
    pub peen: PEEN,
    #[doc = "0x04 - Memory Checking Control Register"]
    pub mchkcon: MCHKCON,
    #[doc = "0x08 - Parity Error Trap Enable Register"]
    pub pete: PETE,
    #[doc = "0x0c - Parity Error Reset Enable Register"]
    pub persten: PERSTEN,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Parity Error Flag Register"]
    pub peflag: PEFLAG,
    #[doc = "0x18 - Parity Memory Test Pattern Register"]
    pub pmtpr: PMTPR,
    #[doc = "0x1c - Parity Memory Test Select Register"]
    pub pmtsr: PMTSR,
}
#[doc = "PEEN (rw) register accessor: Parity Error Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peen`]
module"]
pub type PEEN = crate::Reg<peen::PEEN_SPEC>;
#[doc = "Parity Error Enable Register"]
pub mod peen;
#[doc = "MCHKCON (rw) register accessor: Memory Checking Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mchkcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mchkcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mchkcon`]
module"]
pub type MCHKCON = crate::Reg<mchkcon::MCHKCON_SPEC>;
#[doc = "Memory Checking Control Register"]
pub mod mchkcon;
#[doc = "PETE (rw) register accessor: Parity Error Trap Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pete::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pete::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pete`]
module"]
pub type PETE = crate::Reg<pete::PETE_SPEC>;
#[doc = "Parity Error Trap Enable Register"]
pub mod pete;
#[doc = "PERSTEN (rw) register accessor: Parity Error Reset Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`persten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`persten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`persten`]
module"]
pub type PERSTEN = crate::Reg<persten::PERSTEN_SPEC>;
#[doc = "Parity Error Reset Enable Register"]
pub mod persten;
#[doc = "PEFLAG (rw) register accessor: Parity Error Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`peflag`]
module"]
pub type PEFLAG = crate::Reg<peflag::PEFLAG_SPEC>;
#[doc = "Parity Error Flag Register"]
pub mod peflag;
#[doc = "PMTPR (rw) register accessor: Parity Memory Test Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pmtpr`]
module"]
pub type PMTPR = crate::Reg<pmtpr::PMTPR_SPEC>;
#[doc = "Parity Memory Test Pattern Register"]
pub mod pmtpr;
#[doc = "PMTSR (rw) register accessor: Parity Memory Test Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pmtsr`]
module"]
pub type PMTSR = crate::Reg<pmtsr::PMTSR_SPEC>;
#[doc = "Parity Memory Test Select Register"]
pub mod pmtsr;
