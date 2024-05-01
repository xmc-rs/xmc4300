#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clc: Clc,
    _reserved1: [u8; 0x04],
    id: Id,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Control Register"]
    #[inline(always)]
    pub const fn clc(&self) -> &Clc {
        &self.clc
    }
    #[doc = "0x08 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
}
#[doc = "CLC (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clc`]
module"]
#[doc(alias = "CLC")]
pub type Clc = crate::Reg<clc::ClcSpec>;
#[doc = "Clock Control Register"]
pub mod clc;
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Module Identification Register"]
pub mod id;
