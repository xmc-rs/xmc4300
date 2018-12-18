#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Message Object Registers"]
    pub mo: [MO; 64],
}
#[doc = r" Register block"]
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
    #[doc = "0x1c - Message Object Control Register"]
    pub moctr: self::mo::MOCTR,
}
#[doc = r" Register block"]
#[doc = "Message Object Registers"]
pub mod mo;
