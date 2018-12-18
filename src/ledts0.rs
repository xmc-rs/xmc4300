#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x04 - Global Control Register"]
    pub globctl: GLOBCTL,
    #[doc = "0x08 - Function Control Register"]
    pub fnctl: FNCTL,
    #[doc = "0x0c - Event Flag Register"]
    pub evfr: EVFR,
    #[doc = "0x10 - Touch-sense TS-Counter Value"]
    pub tsval: TSVAL,
    #[doc = "0x14 - Line Pattern Register 0"]
    pub line0: LINE0,
    #[doc = "0x18 - Line Pattern Register 1"]
    pub line1: LINE1,
    #[doc = "0x1c - LED Compare Register 0"]
    pub ldcmp0: LDCMP0,
    #[doc = "0x20 - LED Compare Register 1"]
    pub ldcmp1: LDCMP1,
    #[doc = "0x24 - Touch-sense Compare Register 0"]
    pub tscmp0: TSCMP0,
    #[doc = "0x28 - Touch-sense Compare Register 1"]
    pub tscmp1: TSCMP1,
}
#[doc = "Module Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "Global Control Register"]
pub struct GLOBCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod globctl;
#[doc = "Function Control Register"]
pub struct FNCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function Control Register"]
pub mod fnctl;
#[doc = "Event Flag Register"]
pub struct EVFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flag Register"]
pub mod evfr;
#[doc = "Touch-sense TS-Counter Value"]
pub struct TSVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Touch-sense TS-Counter Value"]
pub mod tsval;
#[doc = "Line Pattern Register 0"]
pub struct LINE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Pattern Register 0"]
pub mod line0;
#[doc = "Line Pattern Register 1"]
pub struct LINE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Pattern Register 1"]
pub mod line1;
#[doc = "LED Compare Register 0"]
pub struct LDCMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LED Compare Register 0"]
pub mod ldcmp0;
#[doc = "LED Compare Register 1"]
pub struct LDCMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LED Compare Register 1"]
pub mod ldcmp1;
#[doc = "Touch-sense Compare Register 0"]
pub struct TSCMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Touch-sense Compare Register 0"]
pub mod tscmp0;
#[doc = "Touch-sense Compare Register 1"]
pub struct TSCMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Touch-sense Compare Register 1"]
pub mod tscmp1;
