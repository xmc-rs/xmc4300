#[doc = "Register `HAINT` reader"]
pub type R = crate::R<HaintSpec>;
#[doc = "Field `HAINT` reader - Channel Interrupts"]
pub type HaintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Channel Interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HaintR {
        HaintR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Host All Channels Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HaintSpec;
impl crate::RegisterSpec for HaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haint::R`](R) reader structure"]
impl crate::Readable for HaintSpec {}
#[doc = "`reset()` method sets HAINT to value 0"]
impl crate::Resettable for HaintSpec {
    const RESET_VALUE: u32 = 0;
}
