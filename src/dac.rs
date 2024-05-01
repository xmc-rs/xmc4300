#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: ID,
    dac0cfg0: DAC0CFG0,
    dac0cfg1: DAC0CFG1,
    dac1cfg0: DAC1CFG0,
    dac1cfg1: DAC1CFG1,
    dac0data: DAC0DATA,
    dac1data: DAC1DATA,
    dac01data: DAC01DATA,
    dac0patl: DAC0PATL,
    dac0path: DAC0PATH,
    dac1patl: DAC1PATL,
    dac1path: DAC1PATH,
}
impl RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - DAC0 Configuration Register 0"]
    #[inline(always)]
    pub const fn dac0cfg0(&self) -> &DAC0CFG0 {
        &self.dac0cfg0
    }
    #[doc = "0x08 - DAC0 Configuration Register 1"]
    #[inline(always)]
    pub const fn dac0cfg1(&self) -> &DAC0CFG1 {
        &self.dac0cfg1
    }
    #[doc = "0x0c - DAC1 Configuration Register 0"]
    #[inline(always)]
    pub const fn dac1cfg0(&self) -> &DAC1CFG0 {
        &self.dac1cfg0
    }
    #[doc = "0x10 - DAC1 Configuration Register 1"]
    #[inline(always)]
    pub const fn dac1cfg1(&self) -> &DAC1CFG1 {
        &self.dac1cfg1
    }
    #[doc = "0x14 - DAC0 Data Register"]
    #[inline(always)]
    pub const fn dac0data(&self) -> &DAC0DATA {
        &self.dac0data
    }
    #[doc = "0x18 - DAC1 Data Register"]
    #[inline(always)]
    pub const fn dac1data(&self) -> &DAC1DATA {
        &self.dac1data
    }
    #[doc = "0x1c - DAC01 Data Register"]
    #[inline(always)]
    pub const fn dac01data(&self) -> &DAC01DATA {
        &self.dac01data
    }
    #[doc = "0x20 - DAC0 Lower Pattern Register"]
    #[inline(always)]
    pub const fn dac0patl(&self) -> &DAC0PATL {
        &self.dac0patl
    }
    #[doc = "0x24 - DAC0 Higher Pattern Register"]
    #[inline(always)]
    pub const fn dac0path(&self) -> &DAC0PATH {
        &self.dac0path
    }
    #[doc = "0x28 - DAC1 Lower Pattern Register"]
    #[inline(always)]
    pub const fn dac1patl(&self) -> &DAC1PATL {
        &self.dac1patl
    }
    #[doc = "0x2c - DAC1 Higher Pattern Register"]
    #[inline(always)]
    pub const fn dac1path(&self) -> &DAC1PATH {
        &self.dac1path
    }
}
#[doc = "ID (r) register accessor: Module Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "DAC0CFG0 (rw) register accessor: DAC0 Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0cfg0`]
module"]
pub type DAC0CFG0 = crate::Reg<dac0cfg0::DAC0CFG0_SPEC>;
#[doc = "DAC0 Configuration Register 0"]
pub mod dac0cfg0;
#[doc = "DAC0CFG1 (rw) register accessor: DAC0 Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0cfg1`]
module"]
pub type DAC0CFG1 = crate::Reg<dac0cfg1::DAC0CFG1_SPEC>;
#[doc = "DAC0 Configuration Register 1"]
pub mod dac0cfg1;
#[doc = "DAC1CFG0 (rw) register accessor: DAC1 Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1cfg0`]
module"]
pub type DAC1CFG0 = crate::Reg<dac1cfg0::DAC1CFG0_SPEC>;
#[doc = "DAC1 Configuration Register 0"]
pub mod dac1cfg0;
#[doc = "DAC1CFG1 (rw) register accessor: DAC1 Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1cfg1`]
module"]
pub type DAC1CFG1 = crate::Reg<dac1cfg1::DAC1CFG1_SPEC>;
#[doc = "DAC1 Configuration Register 1"]
pub mod dac1cfg1;
#[doc = "DAC0DATA (rw) register accessor: DAC0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0data`]
module"]
pub type DAC0DATA = crate::Reg<dac0data::DAC0DATA_SPEC>;
#[doc = "DAC0 Data Register"]
pub mod dac0data;
#[doc = "DAC1DATA (rw) register accessor: DAC1 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1data`]
module"]
pub type DAC1DATA = crate::Reg<dac1data::DAC1DATA_SPEC>;
#[doc = "DAC1 Data Register"]
pub mod dac1data;
#[doc = "DAC01DATA (rw) register accessor: DAC01 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac01data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac01data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac01data`]
module"]
pub type DAC01DATA = crate::Reg<dac01data::DAC01DATA_SPEC>;
#[doc = "DAC01 Data Register"]
pub mod dac01data;
#[doc = "DAC0PATL (rw) register accessor: DAC0 Lower Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0patl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0patl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0patl`]
module"]
pub type DAC0PATL = crate::Reg<dac0patl::DAC0PATL_SPEC>;
#[doc = "DAC0 Lower Pattern Register"]
pub mod dac0patl;
#[doc = "DAC0PATH (rw) register accessor: DAC0 Higher Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac0path::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac0path::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0path`]
module"]
pub type DAC0PATH = crate::Reg<dac0path::DAC0PATH_SPEC>;
#[doc = "DAC0 Higher Pattern Register"]
pub mod dac0path;
#[doc = "DAC1PATL (rw) register accessor: DAC1 Lower Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1patl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1patl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1patl`]
module"]
pub type DAC1PATL = crate::Reg<dac1patl::DAC1PATL_SPEC>;
#[doc = "DAC1 Lower Pattern Register"]
pub mod dac1patl;
#[doc = "DAC1PATH (rw) register accessor: DAC1 Higher Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1path::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1path::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1path`]
module"]
pub type DAC1PATH = crate::Reg<dac1path::DAC1PATH_SPEC>;
#[doc = "DAC1 Higher Pattern Register"]
pub mod dac1path;
