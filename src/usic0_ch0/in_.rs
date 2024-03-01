#[doc = "Register `IN[%s]` writer"]
pub type W = crate::W<InSpec>;
#[doc = "Field `TDATA` writer - Transmit Data"]
pub type TdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn tdata(&mut self) -> TdataW<InSpec> {
        TdataW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Buffer\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`in_::W`](W) writer structure"]
impl crate::Writable for InSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN[%s]
to value 0"]
impl crate::Resettable for InSpec {
    const RESET_VALUE: u32 = 0;
}
