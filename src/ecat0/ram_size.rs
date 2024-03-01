#[doc = "Register `RAM_SIZE` reader"]
pub type R = crate::R<RamSizeSpec>;
#[doc = "Field `RAM_Size` reader - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
pub type RamSizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
    #[inline(always)]
    pub fn ram_size(&self) -> RamSizeR {
        RamSizeR::new(self.bits)
    }
}
#[doc = "RAM Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamSizeSpec;
impl crate::RegisterSpec for RamSizeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_size::R`](R) reader structure"]
impl crate::Readable for RamSizeSpec {}
#[doc = "`reset()` method sets RAM_SIZE to value 0x08"]
impl crate::Resettable for RamSizeSpec {
    const RESET_VALUE: u8 = 0x08;
}
