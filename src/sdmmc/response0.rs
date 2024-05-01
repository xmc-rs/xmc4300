#[doc = "Register `RESPONSE0` reader"]
pub type R = crate::R<Response0Spec>;
#[doc = "Field `RESPONSE0` reader - Response0"]
pub type Response0R = crate::FieldReader<u16>;
#[doc = "Field `RESPONSE1` reader - Response1"]
pub type Response1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Response0"]
    #[inline(always)]
    pub fn response0(&self) -> Response0R {
        Response0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response1"]
    #[inline(always)]
    pub fn response1(&self) -> Response1R {
        Response1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response0Spec;
impl crate::RegisterSpec for Response0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response0::R`](R) reader structure"]
impl crate::Readable for Response0Spec {}
#[doc = "`reset()` method sets RESPONSE0 to value 0"]
impl crate::Resettable for Response0Spec {
    const RESET_VALUE: u32 = 0;
}
