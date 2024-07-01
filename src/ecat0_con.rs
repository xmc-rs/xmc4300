#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    con: CON,
    conp0: CONP0,
    conp1: CONP1,
}
impl RegisterBlock {
    #[doc = "0x00 - EtherCAT 0 Control"]
    #[inline(always)]
    pub const fn con(&self) -> &CON {
        &self.con
    }
    #[doc = "0x04 - EtherCAT 0 Port 1 Control Register"]
    #[inline(always)]
    pub const fn conp0(&self) -> &CONP0 {
        &self.conp0
    }
    #[doc = "0x08 - EtherCAT 0 Port 1 Control Register"]
    #[inline(always)]
    pub const fn conp1(&self) -> &CONP1 {
        &self.conp1
    }
}
#[doc = "CON (rw) register accessor: EtherCAT 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
pub type CON = crate::Reg<con::CON_SPEC>;
#[doc = "EtherCAT 0 Control"]
#[path = "ecat0_con/con_.rs"]
pub mod con;
#[doc = "CONP0 (rw) register accessor: EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`conp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conp0`]
module"]
pub type CONP0 = crate::Reg<conp0::CONP0_SPEC>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp0;
#[doc = "CONP1 (rw) register accessor: EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`conp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conp1`]
module"]
pub type CONP1 = crate::Reg<conp1::CONP1_SPEC>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp1;
