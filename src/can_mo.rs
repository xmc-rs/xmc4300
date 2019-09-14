#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Message Object Registers"]
    pub mo: [MO; 64],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MO {
    #[doc = "0x00 - Message Object Function Control Register"]
    pub mofcr: self::mo::MOFCR,
    #[doc = "0x04 - Message Object FIFO/Gateway Pointer Register"]
    pub mofgpr: self::mo::MOFGPR,
    #[doc = "0x08 - Message Object Interrupt Pointer Register"]
    pub moipr: self::mo::MOIPR,
    #[doc = "0x0c - Message Object Acceptance Mask Register"]
    pub moamr: self::mo::MOAMR,
    #[doc = "0x10 - Message Object Data Register Low"]
    pub modatal: self::mo::MODATAL,
    #[doc = "0x14 - Message Object Data Register High"]
    pub modatah: self::mo::MODATAH,
    #[doc = "0x18 - Message Object Arbitration Register"]
    pub moar: self::mo::MOAR,
    _reserved_7_moctr: [u8; 4usize],
}
impl MO {
    #[doc = "0x1c - Message Object Status Register"]
    #[inline(always)]
    pub fn mostat(&self) -> &self::mo::MOSTAT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const self::mo::MOSTAT) }
    }
    #[doc = "0x1c - Message Object Status Register"]
    #[inline(always)]
    pub fn mostat_mut(&self) -> &mut self::mo::MOSTAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut self::mo::MOSTAT) }
    }
    #[doc = "0x1c - Message Object Control Register"]
    #[inline(always)]
    pub fn moctr(&self) -> &self::mo::MOCTR {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const self::mo::MOCTR) }
    }
    #[doc = "0x1c - Message Object Control Register"]
    #[inline(always)]
    pub fn moctr_mut(&self) -> &mut self::mo::MOCTR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut self::mo::MOCTR) }
    }
}
#[doc = r"Register block"]
#[doc = "Message Object Registers"]
pub mod mo;
