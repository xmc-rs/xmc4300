#[doc = "Register `OUTDR` reader"]
pub type R = crate::R<OutdrSpec>;
#[doc = "Field `DSR` reader - Data from Shift Register"]
pub type DsrR = crate::FieldReader<u16>;
#[doc = "Field `RCI` reader - Receive Control Information from Shift Register"]
pub type RciR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Data from Shift Register"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Receive Control Information from Shift Register"]
    #[inline(always)]
    pub fn rci(&self) -> RciR {
        RciR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Receiver Buffer Output Register L for Debugger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutdrSpec;
impl crate::RegisterSpec for OutdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outdr::R`](R) reader structure"]
impl crate::Readable for OutdrSpec {}
#[doc = "`reset()` method sets OUTDR to value 0"]
impl crate::Resettable for OutdrSpec {
    const RESET_VALUE: u32 = 0;
}
