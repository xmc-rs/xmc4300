#[doc = "Register `RESPONSE6` reader"]
pub type R = crate::R<RESPONSE6_SPEC>;
#[doc = "Field `RESPONSE6` reader - Response6"]
pub type RESPONSE6_R = crate::FieldReader<u16>;
#[doc = "Field `RESPONSE7` reader - Response7"]
pub type RESPONSE7_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Response6"]
    #[inline(always)]
    pub fn response6(&self) -> RESPONSE6_R {
        RESPONSE6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response7"]
    #[inline(always)]
    pub fn response7(&self) -> RESPONSE7_R {
        RESPONSE7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPONSE6_SPEC;
impl crate::RegisterSpec for RESPONSE6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response6::R`](R) reader structure"]
impl crate::Readable for RESPONSE6_SPEC {}
#[doc = "`reset()` method sets RESPONSE6 to value 0"]
impl crate::Resettable for RESPONSE6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
