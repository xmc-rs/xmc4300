#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x04 - DAC0 Configuration Register 0"]
    pub dac0cfg0: DAC0CFG0,
    #[doc = "0x08 - DAC0 Configuration Register 1"]
    pub dac0cfg1: DAC0CFG1,
    #[doc = "0x0c - DAC1 Configuration Register 0"]
    pub dac1cfg0: DAC1CFG0,
    #[doc = "0x10 - DAC1 Configuration Register 1"]
    pub dac1cfg1: DAC1CFG1,
    #[doc = "0x14 - DAC0 Data Register"]
    pub dac0data: DAC0DATA,
    #[doc = "0x18 - DAC1 Data Register"]
    pub dac1data: DAC1DATA,
    #[doc = "0x1c - DAC01 Data Register"]
    pub dac01data: DAC01DATA,
    #[doc = "0x20 - DAC0 Lower Pattern Register"]
    pub dac0patl: DAC0PATL,
    #[doc = "0x24 - DAC0 Higher Pattern Register"]
    pub dac0path: DAC0PATH,
    #[doc = "0x28 - DAC1 Lower Pattern Register"]
    pub dac1patl: DAC1PATL,
    #[doc = "0x2c - DAC1 Higher Pattern Register"]
    pub dac1path: DAC1PATH,
}
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "DAC0CFG0 (rw) register accessor: an alias for `Reg<DAC0CFG0_SPEC>`"]
pub type DAC0CFG0 = crate::Reg<dac0cfg0::DAC0CFG0_SPEC>;
#[doc = "DAC0 Configuration Register 0"]
pub mod dac0cfg0;
#[doc = "DAC0CFG1 (rw) register accessor: an alias for `Reg<DAC0CFG1_SPEC>`"]
pub type DAC0CFG1 = crate::Reg<dac0cfg1::DAC0CFG1_SPEC>;
#[doc = "DAC0 Configuration Register 1"]
pub mod dac0cfg1;
#[doc = "DAC1CFG0 (rw) register accessor: an alias for `Reg<DAC1CFG0_SPEC>`"]
pub type DAC1CFG0 = crate::Reg<dac1cfg0::DAC1CFG0_SPEC>;
#[doc = "DAC1 Configuration Register 0"]
pub mod dac1cfg0;
#[doc = "DAC1CFG1 (rw) register accessor: an alias for `Reg<DAC1CFG1_SPEC>`"]
pub type DAC1CFG1 = crate::Reg<dac1cfg1::DAC1CFG1_SPEC>;
#[doc = "DAC1 Configuration Register 1"]
pub mod dac1cfg1;
#[doc = "DAC0DATA (rw) register accessor: an alias for `Reg<DAC0DATA_SPEC>`"]
pub type DAC0DATA = crate::Reg<dac0data::DAC0DATA_SPEC>;
#[doc = "DAC0 Data Register"]
pub mod dac0data;
#[doc = "DAC1DATA (rw) register accessor: an alias for `Reg<DAC1DATA_SPEC>`"]
pub type DAC1DATA = crate::Reg<dac1data::DAC1DATA_SPEC>;
#[doc = "DAC1 Data Register"]
pub mod dac1data;
#[doc = "DAC01DATA (rw) register accessor: an alias for `Reg<DAC01DATA_SPEC>`"]
pub type DAC01DATA = crate::Reg<dac01data::DAC01DATA_SPEC>;
#[doc = "DAC01 Data Register"]
pub mod dac01data;
#[doc = "DAC0PATL (rw) register accessor: an alias for `Reg<DAC0PATL_SPEC>`"]
pub type DAC0PATL = crate::Reg<dac0patl::DAC0PATL_SPEC>;
#[doc = "DAC0 Lower Pattern Register"]
pub mod dac0patl;
#[doc = "DAC0PATH (rw) register accessor: an alias for `Reg<DAC0PATH_SPEC>`"]
pub type DAC0PATH = crate::Reg<dac0path::DAC0PATH_SPEC>;
#[doc = "DAC0 Higher Pattern Register"]
pub mod dac0path;
#[doc = "DAC1PATL (rw) register accessor: an alias for `Reg<DAC1PATL_SPEC>`"]
pub type DAC1PATL = crate::Reg<dac1patl::DAC1PATL_SPEC>;
#[doc = "DAC1 Lower Pattern Register"]
pub mod dac1patl;
#[doc = "DAC1PATH (rw) register accessor: an alias for `Reg<DAC1PATH_SPEC>`"]
pub type DAC1PATH = crate::Reg<dac1path::DAC1PATH_SPEC>;
#[doc = "DAC1 Higher Pattern Register"]
pub mod dac1path;
