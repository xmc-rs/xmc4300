#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral Bridge Status Register"]
    pub sts: STS,
    #[doc = "0x04 - PBA Write Error Address Register"]
    pub waddr: WADDR,
}
#[doc = "STS (rw) register accessor: Peripheral Bridge Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Peripheral Bridge Status Register"]
pub mod sts;
#[doc = "WADDR (r) register accessor: PBA Write Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`waddr`]
module"]
pub type WADDR = crate::Reg<waddr::WADDR_SPEC>;
#[doc = "PBA Write Error Address Register"]
pub mod waddr;
