#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x800 - Message Object Registers"]
    pub mo: [MO; 64],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MO {
    #[doc = "0x00 - Message Object Function Control Register"]
    pub mofcr: crate::Reg<self::mo::mofcr::MOFCR_SPEC>,
    #[doc = "0x04 - Message Object FIFO/Gateway Pointer Register"]
    pub mofgpr: crate::Reg<self::mo::mofgpr::MOFGPR_SPEC>,
    #[doc = "0x08 - Message Object Interrupt Pointer Register"]
    pub moipr: crate::Reg<self::mo::moipr::MOIPR_SPEC>,
    #[doc = "0x0c - Message Object Acceptance Mask Register"]
    pub moamr: crate::Reg<self::mo::moamr::MOAMR_SPEC>,
    #[doc = "0x10 - Message Object Data Register Low"]
    pub modatal: crate::Reg<self::mo::modatal::MODATAL_SPEC>,
    #[doc = "0x14 - Message Object Data Register High"]
    pub modatah: crate::Reg<self::mo::modatah::MODATAH_SPEC>,
    #[doc = "0x18 - Message Object Arbitration Register"]
    pub moar: crate::Reg<self::mo::moar::MOAR_SPEC>,
    _reserved_7_moctr: [u8; 0x04],
}
impl MO {
    #[doc = "0x1c - Message Object Status Register"]
    #[inline(always)]
    pub fn mostat(&self) -> &crate::Reg<self::mo::mostat::MOSTAT_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const crate::Reg<self::mo::mostat::MOSTAT_SPEC>) }
    }
    #[doc = "0x1c - Message Object Control Register"]
    #[inline(always)]
    pub fn moctr(&self) -> &crate::Reg<self::mo::moctr::MOCTR_SPEC> {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const crate::Reg<self::mo::moctr::MOCTR_SPEC>) }
    }
}
#[doc = r"Register block"]
#[doc = "Message Object Registers"]
pub mod mo;
