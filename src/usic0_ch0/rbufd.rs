#[doc = "Register `RBUFD` reader"]
pub type R = crate::R<RbufdSpec>;
#[doc = "Field `DSR` reader - Data from Shift Register"]
pub type DsrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data from Shift Register"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver Buffer Register for Debugger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbufd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbufdSpec;
impl crate::RegisterSpec for RbufdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbufd::R`](R) reader structure"]
impl crate::Readable for RbufdSpec {}
#[doc = "`reset()` method sets RBUFD to value 0"]
impl crate::Resettable for RbufdSpec {
    const RESET_VALUE: u32 = 0;
}
