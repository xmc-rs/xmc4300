#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: Sts,
    waddr: Waddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral Bridge Status Register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x04 - PBA Write Error Address Register"]
    #[inline(always)]
    pub const fn waddr(&self) -> &Waddr {
        &self.waddr
    }
}
#[doc = "STS (rw) register accessor: Peripheral Bridge Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Peripheral Bridge Status Register"]
pub mod sts;
#[doc = "WADDR (r) register accessor: PBA Write Error Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waddr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waddr`]
module"]
#[doc(alias = "WADDR")]
pub type Waddr = crate::Reg<waddr::WaddrSpec>;
#[doc = "PBA Write Error Address Register"]
pub mod waddr;
