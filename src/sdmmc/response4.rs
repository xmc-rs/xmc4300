#[doc = "Register `RESPONSE4` reader"]
pub type R = crate::R<RESPONSE4_SPEC>;
#[doc = "Field `RESPONSE4` reader - Response4"]
pub type RESPONSE4_R = crate::FieldReader<u16>;
#[doc = "Field `RESPONSE5` reader - Response5"]
pub type RESPONSE5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Response4"]
    #[inline(always)]
    pub fn response4(&self) -> RESPONSE4_R {
        RESPONSE4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response5"]
    #[inline(always)]
    pub fn response5(&self) -> RESPONSE5_R {
        RESPONSE5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPONSE4_SPEC;
impl crate::RegisterSpec for RESPONSE4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response4::R`](R) reader structure"]
impl crate::Readable for RESPONSE4_SPEC {}
#[doc = "`reset()` method sets RESPONSE4 to value 0"]
impl crate::Resettable for RESPONSE4_SPEC {
    const RESET_VALUE: u32 = 0;
}
