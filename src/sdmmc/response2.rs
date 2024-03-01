#[doc = "Register `RESPONSE2` reader"]
pub type R = crate::R<Response2Spec>;
#[doc = "Field `RESPONSE2` reader - Response2"]
pub type Response2R = crate::FieldReader<u16>;
#[doc = "Field `RESPONSE3` reader - Response3"]
pub type Response3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Response2"]
    #[inline(always)]
    pub fn response2(&self) -> Response2R {
        Response2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response3"]
    #[inline(always)]
    pub fn response3(&self) -> Response3R {
        Response3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response2Spec;
impl crate::RegisterSpec for Response2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response2::R`](R) reader structure"]
impl crate::Readable for Response2Spec {}
#[doc = "`reset()` method sets RESPONSE2 to value 0"]
impl crate::Resettable for Response2Spec {
    const RESET_VALUE: u32 = 0;
}
