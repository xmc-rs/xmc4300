#[doc = "Reader of register TYPE"]
pub type R = crate::R<u8, super::TYPE>;
#[doc = "Reader of field `Type`"]
pub type TYPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Type of EtherCAT controller"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0xff) as u8)
    }
}
