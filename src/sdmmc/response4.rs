#[doc = "Register `RESPONSE4` reader"]
pub type R = crate::R<Response4Spec>;
#[doc = "Field `RESPONSE4` reader - Response4"]
pub type Response4R = crate::FieldReader<u16>;
#[doc = "Field `RESPONSE5` reader - Response5"]
pub type Response5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Response4"]
    #[inline(always)]
    pub fn response4(&self) -> Response4R {
        Response4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Response5"]
    #[inline(always)]
    pub fn response5(&self) -> Response5R {
        Response5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Response 4 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`response4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Response4Spec;
impl crate::RegisterSpec for Response4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`response4::R`](R) reader structure"]
impl crate::Readable for Response4Spec {}
#[doc = "`reset()` method sets RESPONSE4 to value 0"]
impl crate::Resettable for Response4Spec {
    const RESET_VALUE: u32 = 0;
}
