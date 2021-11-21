#[doc = "Register `RAM_SIZE` reader"]
pub struct R(crate::R<RAM_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAM_Size` reader - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
pub struct RAM_SIZE_R(crate::FieldReader<u8, u8>);
impl RAM_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Process Data RAM size supported by the EtherCAT Slave Controller in KByte"]
    #[inline(always)]
    pub fn ram_size(&self) -> RAM_SIZE_R {
        RAM_SIZE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RAM Size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_size](index.html) module"]
pub struct RAM_SIZE_SPEC;
impl crate::RegisterSpec for RAM_SIZE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ram_size::R](R) reader structure"]
impl crate::Readable for RAM_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAM_SIZE to value 0x08"]
impl crate::Resettable for RAM_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
