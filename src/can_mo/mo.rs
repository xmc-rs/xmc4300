#[repr(C)]
#[doc = "Message Object Registers"]
#[doc(alias = "MO")]
pub struct Mo {
    mofcr: Mofcr,
    mofgpr: Mofgpr,
    moipr: Moipr,
    moamr: Moamr,
    modatal: Modatal,
    modatah: Modatah,
    moar: Moar,
    _reserved_7_moctr: [u8; 0x04],
}
impl Mo {
    #[doc = "0x00 - Message Object Function Control Register"]
    #[inline(always)]
    pub const fn mofcr(&self) -> &Mofcr {
        &self.mofcr
    }
    #[doc = "0x04 - Message Object FIFO/Gateway Pointer Register"]
    #[inline(always)]
    pub const fn mofgpr(&self) -> &Mofgpr {
        &self.mofgpr
    }
    #[doc = "0x08 - Message Object Interrupt Pointer Register"]
    #[inline(always)]
    pub const fn moipr(&self) -> &Moipr {
        &self.moipr
    }
    #[doc = "0x0c - Message Object Acceptance Mask Register"]
    #[inline(always)]
    pub const fn moamr(&self) -> &Moamr {
        &self.moamr
    }
    #[doc = "0x10 - Message Object Data Register Low"]
    #[inline(always)]
    pub const fn modatal(&self) -> &Modatal {
        &self.modatal
    }
    #[doc = "0x14 - Message Object Data Register High"]
    #[inline(always)]
    pub const fn modatah(&self) -> &Modatah {
        &self.modatah
    }
    #[doc = "0x18 - Message Object Arbitration Register"]
    #[inline(always)]
    pub const fn moar(&self) -> &Moar {
        &self.moar
    }
    #[doc = "0x1c - Message Object Status Register"]
    #[inline(always)]
    pub const fn mostat(&self) -> &Mostat {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Message Object Control Register"]
    #[inline(always)]
    pub const fn moctr(&self) -> &Moctr {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
}
#[doc = "MOFCR (rw) register accessor: Message Object Function Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mofcr`]
module"]
#[doc(alias = "MOFCR")]
pub type Mofcr = crate::Reg<mofcr::MofcrSpec>;
#[doc = "Message Object Function Control Register"]
pub mod mofcr;
#[doc = "MOFGPR (rw) register accessor: Message Object FIFO/Gateway Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mofgpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mofgpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mofgpr`]
module"]
#[doc(alias = "MOFGPR")]
pub type Mofgpr = crate::Reg<mofgpr::MofgprSpec>;
#[doc = "Message Object FIFO/Gateway Pointer Register"]
pub mod mofgpr;
#[doc = "MOIPR (rw) register accessor: Message Object Interrupt Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moipr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moipr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moipr`]
module"]
#[doc(alias = "MOIPR")]
pub type Moipr = crate::Reg<moipr::MoiprSpec>;
#[doc = "Message Object Interrupt Pointer Register"]
pub mod moipr;
#[doc = "MOAMR (rw) register accessor: Message Object Acceptance Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moamr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moamr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moamr`]
module"]
#[doc(alias = "MOAMR")]
pub type Moamr = crate::Reg<moamr::MoamrSpec>;
#[doc = "Message Object Acceptance Mask Register"]
pub mod moamr;
#[doc = "MODATAL (rw) register accessor: Message Object Data Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modatal`]
module"]
#[doc(alias = "MODATAL")]
pub type Modatal = crate::Reg<modatal::ModatalSpec>;
#[doc = "Message Object Data Register Low"]
pub mod modatal;
#[doc = "MODATAH (rw) register accessor: Message Object Data Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modatah::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modatah::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modatah`]
module"]
#[doc(alias = "MODATAH")]
pub type Modatah = crate::Reg<modatah::ModatahSpec>;
#[doc = "Message Object Data Register High"]
pub mod modatah;
#[doc = "MOAR (rw) register accessor: Message Object Arbitration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moar`]
module"]
#[doc(alias = "MOAR")]
pub type Moar = crate::Reg<moar::MoarSpec>;
#[doc = "Message Object Arbitration Register"]
pub mod moar;
#[doc = "MOCTR (w) register accessor: Message Object Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moctr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moctr`]
module"]
#[doc(alias = "MOCTR")]
pub type Moctr = crate::Reg<moctr::MoctrSpec>;
#[doc = "Message Object Control Register"]
pub mod moctr;
#[doc = "MOSTAT (r) register accessor: Message Object Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mostat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mostat`]
module"]
#[doc(alias = "MOSTAT")]
pub type Mostat = crate::Reg<mostat::MostatSpec>;
#[doc = "Message Object Status Register"]
pub mod mostat;
