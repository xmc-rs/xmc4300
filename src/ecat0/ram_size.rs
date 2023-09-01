#[doc = "Register `RAM_SIZE` reader"]
pub type R = crate::R<RAM_SIZE_SPEC>;
#[doc = "Field `RAM_Size` reader - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
pub type RAM_SIZE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
    #[inline(always)]
    pub fn ram_size(&self) -> RAM_SIZE_R {
        RAM_SIZE_R::new(self.bits)
    }
}
#[doc = "RAM Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM_SIZE_SPEC;
impl crate::RegisterSpec for RAM_SIZE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ram_size::R`](R) reader structure"]
impl crate::Readable for RAM_SIZE_SPEC {}
#[doc = "`reset()` method sets RAM_SIZE to value 0x08"]
impl crate::Resettable for RAM_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
