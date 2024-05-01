#[doc = "Register `HAINT` reader"]
pub type R = crate::R<HAINT_SPEC>;
#[doc = "Field `HAINT` reader - Channel Interrupts"]
pub type HAINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Channel Interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "Host All Channels Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HAINT_SPEC;
impl crate::RegisterSpec for HAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haint::R`](R) reader structure"]
impl crate::Readable for HAINT_SPEC {}
#[doc = "`reset()` method sets HAINT to value 0"]
impl crate::Resettable for HAINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
