#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    con: Con,
    conp0: Conp0,
    conp1: Conp1,
}
impl RegisterBlock {
    #[doc = "0x00 - EtherCAT 0 Control"]
    #[inline(always)]
    pub const fn con(&self) -> &Con {
        &self.con
    }
    #[doc = "0x04 - EtherCAT 0 Port 1 Control Register"]
    #[inline(always)]
    pub const fn conp0(&self) -> &Conp0 {
        &self.conp0
    }
    #[doc = "0x08 - EtherCAT 0 Port 1 Control Register"]
    #[inline(always)]
    pub const fn conp1(&self) -> &Conp1 {
        &self.conp1
    }
}
#[doc = "CON (rw) register accessor: EtherCAT 0 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
#[doc(alias = "CON")]
pub type Con = crate::Reg<con::ConSpec>;
#[doc = "EtherCAT 0 Control"]
#[path = "ecat0_con/con_.rs"]
pub mod con;
#[doc = "CONP0 (rw) register accessor: EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conp0`]
module"]
#[doc(alias = "CONP0")]
pub type Conp0 = crate::Reg<conp0::Conp0Spec>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp0;
#[doc = "CONP1 (rw) register accessor: EtherCAT 0 Port 1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conp1`]
module"]
#[doc(alias = "CONP1")]
pub type Conp1 = crate::Reg<conp1::Conp1Spec>;
#[doc = "EtherCAT 0 Port 1 Control Register"]
pub mod conp1;
