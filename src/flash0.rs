#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1008],
    id: Id,
    _reserved1: [u8; 0x04],
    fsr: Fsr,
    fcon: Fcon,
    marp: Marp,
    _reserved4: [u8; 0x04],
    procon0: Procon0,
    procon1: Procon1,
    procon2: Procon2,
}
impl RegisterBlock {
    #[doc = "0x1008 - Flash Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x1010 - Flash Status Register"]
    #[inline(always)]
    pub const fn fsr(&self) -> &Fsr {
        &self.fsr
    }
    #[doc = "0x1014 - Flash Configuration Register"]
    #[inline(always)]
    pub const fn fcon(&self) -> &Fcon {
        &self.fcon
    }
    #[doc = "0x1018 - Margin Control Register PFLASH"]
    #[inline(always)]
    pub const fn marp(&self) -> &Marp {
        &self.marp
    }
    #[doc = "0x1020 - Flash Protection Configuration Register User 0"]
    #[inline(always)]
    pub const fn procon0(&self) -> &Procon0 {
        &self.procon0
    }
    #[doc = "0x1024 - Flash Protection Configuration Register User 1"]
    #[inline(always)]
    pub const fn procon1(&self) -> &Procon1 {
        &self.procon1
    }
    #[doc = "0x1028 - Flash Protection Configuration Register User 2"]
    #[inline(always)]
    pub const fn procon2(&self) -> &Procon2 {
        &self.procon2
    }
}
#[doc = "ID (r) register accessor: Flash Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Flash Module Identification Register"]
pub mod id;
#[doc = "FSR (r) register accessor: Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`]
module"]
#[doc(alias = "FSR")]
pub type Fsr = crate::Reg<fsr::FsrSpec>;
#[doc = "Flash Status Register"]
pub mod fsr;
#[doc = "FCON (rw) register accessor: Flash Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcon`]
module"]
#[doc(alias = "FCON")]
pub type Fcon = crate::Reg<fcon::FconSpec>;
#[doc = "Flash Configuration Register"]
pub mod fcon;
#[doc = "MARP (rw) register accessor: Margin Control Register PFLASH\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`marp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`marp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@marp`]
module"]
#[doc(alias = "MARP")]
pub type Marp = crate::Reg<marp::MarpSpec>;
#[doc = "Margin Control Register PFLASH"]
pub mod marp;
#[doc = "PROCON0 (r) register accessor: Flash Protection Configuration Register User 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procon0`]
module"]
#[doc(alias = "PROCON0")]
pub type Procon0 = crate::Reg<procon0::Procon0Spec>;
#[doc = "Flash Protection Configuration Register User 0"]
pub mod procon0;
#[doc = "PROCON1 (r) register accessor: Flash Protection Configuration Register User 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procon1`]
module"]
#[doc(alias = "PROCON1")]
pub type Procon1 = crate::Reg<procon1::Procon1Spec>;
#[doc = "Flash Protection Configuration Register User 1"]
pub mod procon1;
#[doc = "PROCON2 (r) register accessor: Flash Protection Configuration Register User 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`procon2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procon2`]
module"]
#[doc(alias = "PROCON2")]
pub type Procon2 = crate::Reg<procon2::Procon2Spec>;
#[doc = "Flash Protection Configuration Register User 2"]
pub mod procon2;
