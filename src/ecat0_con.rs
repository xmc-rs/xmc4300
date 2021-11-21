#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EtherCAT 0 Control"]
    pub con: crate::Reg<con::CON_SPEC>,
    #[doc = "0x04 - EtherCAT 0 Port 1 Control Register"]
    pub conp0: crate::Reg<conp0::CONP0_SPEC>,
    #[doc = "0x08 - EtherCAT 0 Port 1 Control Register"]
    pub conp1: crate::Reg<conp1::CONP1_SPEC>,
}
#[doc = "CON register accessor: an alias for `Reg<CON_SPEC>`"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "EtherCAT 0 Control"]
pub mod con;
#[doc = "CONP0 register accessor: an alias for `Reg<CONP0_SPEC>`"]
pub type CONP0 = crate::Reg<conp0::CONP0_SPEC>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp0;
#[doc = "CONP1 register accessor: an alias for `Reg<CONP1_SPEC>`"]
pub type CONP1 = crate::Reg<conp1::CONP1_SPEC>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp1;
