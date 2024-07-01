#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eth0_con: ETH0_CON,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet 0 Port Control Register"]
    #[inline(always)]
    pub const fn eth0_con(&self) -> &ETH0_CON {
        &self.eth0_con
    }
}
#[doc = "ETH0_CON (rw) register accessor: Ethernet 0 Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eth0_con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth0_con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth0_con`]
module"]
pub type ETH0_CON = crate::Reg<eth0_con::ETH0_CON_SPEC>;
#[doc = "Ethernet 0 Port Control Register"]
pub mod eth0_con;
