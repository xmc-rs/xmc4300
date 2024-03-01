#[doc = "Register `RBUF0` reader"]
pub type R = crate::R<Rbuf0Spec>;
#[doc = "Field `DSR0` reader - Data of Shift Registers 0\\[3:0\\]"]
pub type Dsr0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data of Shift Registers 0\\[3:0\\]"]
    #[inline(always)]
    pub fn dsr0(&self) -> Dsr0R {
        Dsr0R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rbuf0Spec;
impl crate::RegisterSpec for Rbuf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbuf0::R`](R) reader structure"]
impl crate::Readable for Rbuf0Spec {}
#[doc = "`reset()` method sets RBUF0 to value 0"]
impl crate::Resettable for Rbuf0Spec {
    const RESET_VALUE: u32 = 0;
}
