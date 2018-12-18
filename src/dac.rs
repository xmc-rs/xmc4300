#[doc = r" Register block"]
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
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "DAC0 Configuration Register 0"]
pub struct DAC0CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 Configuration Register 0"]
pub mod dac0cfg0;
#[doc = "DAC0 Configuration Register 1"]
pub struct DAC0CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 Configuration Register 1"]
pub mod dac0cfg1;
#[doc = "DAC1 Configuration Register 0"]
pub struct DAC1CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 Configuration Register 0"]
pub mod dac1cfg0;
#[doc = "DAC1 Configuration Register 1"]
pub struct DAC1CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 Configuration Register 1"]
pub mod dac1cfg1;
#[doc = "DAC0 Data Register"]
pub struct DAC0DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 Data Register"]
pub mod dac0data;
#[doc = "DAC1 Data Register"]
pub struct DAC1DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 Data Register"]
pub mod dac1data;
#[doc = "DAC01 Data Register"]
pub struct DAC01DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC01 Data Register"]
pub mod dac01data;
#[doc = "DAC0 Lower Pattern Register"]
pub struct DAC0PATL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 Lower Pattern Register"]
pub mod dac0patl;
#[doc = "DAC0 Higher Pattern Register"]
pub struct DAC0PATH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC0 Higher Pattern Register"]
pub mod dac0path;
#[doc = "DAC1 Lower Pattern Register"]
pub struct DAC1PATL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 Lower Pattern Register"]
pub mod dac1patl;
#[doc = "DAC1 Higher Pattern Register"]
pub struct DAC1PATH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC1 Higher Pattern Register"]
pub mod dac1path;
