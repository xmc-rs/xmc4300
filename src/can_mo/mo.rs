#[repr(C)]
#[doc = "Message Object Registers"]
pub struct MO {
    mofcr: MOFCR,
    mofgpr: MOFGPR,
    moipr: MOIPR,
    moamr: MOAMR,
    modatal: MODATAL,
    modatah: MODATAH,
    moar: MOAR,
    _reserved_7_moctr: [u8; 0x04],
}
impl MO {
    #[doc = "0x00 - Message Object Function Control Register"]
    #[inline(always)]
    pub const fn mofcr(&self) -> &MOFCR {
        &self.mofcr
    }
    #[doc = "0x04 - Message Object FIFO/Gateway Pointer Register"]
    #[inline(always)]
    pub const fn mofgpr(&self) -> &MOFGPR {
        &self.mofgpr
    }
    #[doc = "0x08 - Message Object Interrupt Pointer Register"]
    #[inline(always)]
    pub const fn moipr(&self) -> &MOIPR {
        &self.moipr
    }
    #[doc = "0x0c - Message Object Acceptance Mask Register"]
    #[inline(always)]
    pub const fn moamr(&self) -> &MOAMR {
        &self.moamr
    }
    #[doc = "0x10 - Message Object Data Register Low"]
    #[inline(always)]
    pub const fn modatal(&self) -> &MODATAL {
        &self.modatal
    }
    #[doc = "0x14 - Message Object Data Register High"]
    #[inline(always)]
    pub const fn modatah(&self) -> &MODATAH {
        &self.modatah
    }
    #[doc = "0x18 - Message Object Arbitration Register"]
    #[inline(always)]
    pub const fn moar(&self) -> &MOAR {
        &self.moar
    }
    #[doc = "0x1c - Message Object Status Register"]
    #[inline(always)]
    pub const fn mostat(&self) -> &MOSTAT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Message Object Control Register"]
    #[inline(always)]
    pub const fn moctr(&self) -> &MOCTR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
}
#[doc = "MOFCR (rw) register accessor: Message Object Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mofcr`]
module"]
pub type MOFCR = crate::Reg<mofcr::MOFCR_SPEC>;
#[doc = "Message Object Function Control Register"]
pub mod mofcr;
#[doc = "MOFGPR (rw) register accessor: Message Object FIFO/Gateway Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofgpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofgpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mofgpr`]
module"]
pub type MOFGPR = crate::Reg<mofgpr::MOFGPR_SPEC>;
#[doc = "Message Object FIFO/Gateway Pointer Register"]
pub mod mofgpr;
#[doc = "MOIPR (rw) register accessor: Message Object Interrupt Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moipr`]
module"]
pub type MOIPR = crate::Reg<moipr::MOIPR_SPEC>;
#[doc = "Message Object Interrupt Pointer Register"]
pub mod moipr;
#[doc = "MOAMR (rw) register accessor: Message Object Acceptance Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moamr`]
module"]
pub type MOAMR = crate::Reg<moamr::MOAMR_SPEC>;
#[doc = "Message Object Acceptance Mask Register"]
pub mod moamr;
#[doc = "MODATAL (rw) register accessor: Message Object Data Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modatal`]
module"]
pub type MODATAL = crate::Reg<modatal::MODATAL_SPEC>;
#[doc = "Message Object Data Register Low"]
pub mod modatal;
#[doc = "MODATAH (rw) register accessor: Message Object Data Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modatah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modatah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modatah`]
module"]
pub type MODATAH = crate::Reg<modatah::MODATAH_SPEC>;
#[doc = "Message Object Data Register High"]
pub mod modatah;
#[doc = "MOAR (rw) register accessor: Message Object Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moar`]
module"]
pub type MOAR = crate::Reg<moar::MOAR_SPEC>;
#[doc = "Message Object Arbitration Register"]
pub mod moar;
#[doc = "MOCTR (w) register accessor: Message Object Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moctr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moctr`]
module"]
pub type MOCTR = crate::Reg<moctr::MOCTR_SPEC>;
#[doc = "Message Object Control Register"]
pub mod moctr;
#[doc = "MOSTAT (r) register accessor: Message Object Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mostat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mostat`]
module"]
pub type MOSTAT = crate::Reg<mostat::MOSTAT_SPEC>;
#[doc = "Message Object Status Register"]
pub mod mostat;
