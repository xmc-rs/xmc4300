#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    dac0cfg0: Dac0cfg0,
    dac0cfg1: Dac0cfg1,
    dac1cfg0: Dac1cfg0,
    dac1cfg1: Dac1cfg1,
    dac0data: Dac0data,
    dac1data: Dac1data,
    dac01data: Dac01data,
    dac0patl: Dac0patl,
    dac0path: Dac0path,
    dac1patl: Dac1patl,
    dac1path: Dac1path,
}
impl RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - DAC0 Configuration Register 0"]
    #[inline(always)]
    pub const fn dac0cfg0(&self) -> &Dac0cfg0 {
        &self.dac0cfg0
    }
    #[doc = "0x08 - DAC0 Configuration Register 1"]
    #[inline(always)]
    pub const fn dac0cfg1(&self) -> &Dac0cfg1 {
        &self.dac0cfg1
    }
    #[doc = "0x0c - DAC1 Configuration Register 0"]
    #[inline(always)]
    pub const fn dac1cfg0(&self) -> &Dac1cfg0 {
        &self.dac1cfg0
    }
    #[doc = "0x10 - DAC1 Configuration Register 1"]
    #[inline(always)]
    pub const fn dac1cfg1(&self) -> &Dac1cfg1 {
        &self.dac1cfg1
    }
    #[doc = "0x14 - DAC0 Data Register"]
    #[inline(always)]
    pub const fn dac0data(&self) -> &Dac0data {
        &self.dac0data
    }
    #[doc = "0x18 - DAC1 Data Register"]
    #[inline(always)]
    pub const fn dac1data(&self) -> &Dac1data {
        &self.dac1data
    }
    #[doc = "0x1c - DAC01 Data Register"]
    #[inline(always)]
    pub const fn dac01data(&self) -> &Dac01data {
        &self.dac01data
    }
    #[doc = "0x20 - DAC0 Lower Pattern Register"]
    #[inline(always)]
    pub const fn dac0patl(&self) -> &Dac0patl {
        &self.dac0patl
    }
    #[doc = "0x24 - DAC0 Higher Pattern Register"]
    #[inline(always)]
    pub const fn dac0path(&self) -> &Dac0path {
        &self.dac0path
    }
    #[doc = "0x28 - DAC1 Lower Pattern Register"]
    #[inline(always)]
    pub const fn dac1patl(&self) -> &Dac1patl {
        &self.dac1patl
    }
    #[doc = "0x2c - DAC1 Higher Pattern Register"]
    #[inline(always)]
    pub const fn dac1path(&self) -> &Dac1path {
        &self.dac1path
    }
}
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "DAC0CFG0 (rw) register accessor: DAC0 Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0cfg0`]
module"]
#[doc(alias = "DAC0CFG0")]
pub type Dac0cfg0 = crate::Reg<dac0cfg0::Dac0cfg0Spec>;
#[doc = "DAC0 Configuration Register 0"]
pub mod dac0cfg0;
#[doc = "DAC0CFG1 (rw) register accessor: DAC0 Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0cfg1`]
module"]
#[doc(alias = "DAC0CFG1")]
pub type Dac0cfg1 = crate::Reg<dac0cfg1::Dac0cfg1Spec>;
#[doc = "DAC0 Configuration Register 1"]
pub mod dac0cfg1;
#[doc = "DAC1CFG0 (rw) register accessor: DAC1 Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1cfg0`]
module"]
#[doc(alias = "DAC1CFG0")]
pub type Dac1cfg0 = crate::Reg<dac1cfg0::Dac1cfg0Spec>;
#[doc = "DAC1 Configuration Register 0"]
pub mod dac1cfg0;
#[doc = "DAC1CFG1 (rw) register accessor: DAC1 Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1cfg1`]
module"]
#[doc(alias = "DAC1CFG1")]
pub type Dac1cfg1 = crate::Reg<dac1cfg1::Dac1cfg1Spec>;
#[doc = "DAC1 Configuration Register 1"]
pub mod dac1cfg1;
#[doc = "DAC0DATA (rw) register accessor: DAC0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0data`]
module"]
#[doc(alias = "DAC0DATA")]
pub type Dac0data = crate::Reg<dac0data::Dac0dataSpec>;
#[doc = "DAC0 Data Register"]
pub mod dac0data;
#[doc = "DAC1DATA (rw) register accessor: DAC1 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1data`]
module"]
#[doc(alias = "DAC1DATA")]
pub type Dac1data = crate::Reg<dac1data::Dac1dataSpec>;
#[doc = "DAC1 Data Register"]
pub mod dac1data;
#[doc = "DAC01DATA (rw) register accessor: DAC01 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac01data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac01data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac01data`]
module"]
#[doc(alias = "DAC01DATA")]
pub type Dac01data = crate::Reg<dac01data::Dac01dataSpec>;
#[doc = "DAC01 Data Register"]
pub mod dac01data;
#[doc = "DAC0PATL (rw) register accessor: DAC0 Lower Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0patl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0patl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0patl`]
module"]
#[doc(alias = "DAC0PATL")]
pub type Dac0patl = crate::Reg<dac0patl::Dac0patlSpec>;
#[doc = "DAC0 Lower Pattern Register"]
pub mod dac0patl;
#[doc = "DAC0PATH (rw) register accessor: DAC0 Higher Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0path::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0path::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0path`]
module"]
#[doc(alias = "DAC0PATH")]
pub type Dac0path = crate::Reg<dac0path::Dac0pathSpec>;
#[doc = "DAC0 Higher Pattern Register"]
pub mod dac0path;
#[doc = "DAC1PATL (rw) register accessor: DAC1 Lower Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1patl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1patl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1patl`]
module"]
#[doc(alias = "DAC1PATL")]
pub type Dac1patl = crate::Reg<dac1patl::Dac1patlSpec>;
#[doc = "DAC1 Lower Pattern Register"]
pub mod dac1patl;
#[doc = "DAC1PATH (rw) register accessor: DAC1 Higher Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1path::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1path::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1path`]
module"]
#[doc(alias = "DAC1PATH")]
pub type Dac1path = crate::Reg<dac1path::Dac1pathSpec>;
#[doc = "DAC1 Higher Pattern Register"]
pub mod dac1path;
