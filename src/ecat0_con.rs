#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EtherCAT 0 Control"]
    pub con: CON,
    #[doc = "0x04 - EtherCAT 0 Port 1 Control Register"]
    pub conp0: CONP0,
    #[doc = "0x08 - EtherCAT 0 Port 1 Control Register"]
    pub conp1: CONP1,
}
#[doc = "CON (rw) register accessor: EtherCAT 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`con`]
module"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "EtherCAT 0 Control"]
pub mod con;
#[doc = "CONP0 (rw) register accessor: EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conp0`]
module"]
pub type CONP0 = crate::Reg<conp0::CONP0_SPEC>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp0;
#[doc = "CONP1 (rw) register accessor: EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conp1`]
module"]
pub type CONP1 = crate::Reg<conp1::CONP1_SPEC>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp1;
