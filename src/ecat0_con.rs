#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EtherCAT 0 Control"]
    pub con: CON,
    #[doc = "0x04 - EtherCAT 0 Port 1 Control Register"]
    pub conp0: CONP0,
    #[doc = "0x08 - EtherCAT 0 Port 1 Control Register"]
    pub conp1: CONP1,
}
#[doc = "EtherCAT 0 Control"]
pub struct CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EtherCAT 0 Control"]
pub mod con;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub struct CONP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp0;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub struct CONP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp1;
