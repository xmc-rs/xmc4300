#[doc = "Reader of register RAM_SIZE"]
pub type R = crate::R<u8, super::RAM_SIZE>;
#[doc = "Reader of field `RAM_Size`"]
pub type RAM_SIZE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
    #[inline(always)]
    pub fn ram_size(&self) -> RAM_SIZE_R {
        RAM_SIZE_R::new((self.bits & 0xff) as u8)
    }
}
