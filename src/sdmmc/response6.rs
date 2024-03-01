#[doc = "Register `RESPONSE6` reader"]
pub type R = crate::R<Response6Spec>;
#[doc = "Field `RESPONSE6` reader - Response6"]
pub type Response6R = crate::FieldReader<u16>;
#[doc = "Field `RESPONSE7` reader - Response7"]
pub type Response7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Response6"]
    #[inline(always)]
    pub fn response6(&self) -> Response6R {
        Response6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response7"]
    #[inline(always)]
    pub fn response7(&self) -> Response7R {
        Response7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 6 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response6Spec;
impl crate::RegisterSpec for Response6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response6::R`](R) reader structure"]
impl crate::Readable for Response6Spec {}
#[doc = "`reset()` method sets RESPONSE6 to value 0"]
impl crate::Resettable for Response6Spec {
    const RESET_VALUE: u32 = 0;
}
