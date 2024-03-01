#[doc = "Register `RBUF1` reader"]
pub type R = crate::R<Rbuf1Spec>;
#[doc = "Field `DSR1` reader - Data of Shift Registers 1\\[3:0\\]"]
pub type Dsr1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data of Shift Registers 1\\[3:0\\]"]
    #[inline(always)]
    pub fn dsr1(&self) -> Dsr1R {
        Dsr1R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbuf1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rbuf1Spec;
impl crate::RegisterSpec for Rbuf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbuf1::R`](R) reader structure"]
impl crate::Readable for Rbuf1Spec {}
#[doc = "`reset()` method sets RBUF1 to value 0"]
impl crate::Resettable for Rbuf1Spec {
    const RESET_VALUE: u32 = 0;
}
