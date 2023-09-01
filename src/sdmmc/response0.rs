#[doc = "Register `RESPONSE0` reader"]
pub type R = crate::R<RESPONSE0_SPEC>;
#[doc = "Field `RESPONSE0` reader - Response0"]
pub type RESPONSE0_R = crate::FieldReader<u16>;
#[doc = "Field `RESPONSE1` reader - Response1"]
pub type RESPONSE1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Response0"]
    #[inline(always)]
    pub fn response0(&self) -> RESPONSE0_R {
        RESPONSE0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response1"]
    #[inline(always)]
    pub fn response1(&self) -> RESPONSE1_R {
        RESPONSE1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPONSE0_SPEC;
impl crate::RegisterSpec for RESPONSE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response0::R`](R) reader structure"]
impl crate::Readable for RESPONSE0_SPEC {}
#[doc = "`reset()` method sets RESPONSE0 to value 0"]
impl crate::Resettable for RESPONSE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
