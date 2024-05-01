#[doc = "Register `IDMANUF` reader"]
pub type R = crate::R<IDMANUF_SPEC>;
#[doc = "Field `DEPT` reader - Department Identification Number"]
pub type DEPT_R = crate::FieldReader;
#[doc = "Field `MANUF` reader - Manufacturer Identification Number"]
pub type MANUF_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Department Identification Number"]
    #[inline(always)]
    pub fn dept(&self) -> DEPT_R {
        DEPT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Manufacturer Identification Number"]
    #[inline(always)]
    pub fn manuf(&self) -> MANUF_R {
        MANUF_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
}
#[doc = "Manufactory ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmanuf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDMANUF_SPEC;
impl crate::RegisterSpec for IDMANUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmanuf::R`](R) reader structure"]
impl crate::Readable for IDMANUF_SPEC {}
#[doc = "`reset()` method sets IDMANUF to value 0x1820"]
impl crate::Resettable for IDMANUF_SPEC {
    const RESET_VALUE: u32 = 0x1820;
}
