#[doc = "Register `TCSET` writer"]
pub type W = crate::W<TcsetSpec>;
#[doc = "Field `TRBS` writer - Timer Run Bit set"]
pub type TrbsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Timer Run Bit set"]
    #[inline(always)]
    #[must_use]
    pub fn trbs(&mut self) -> TrbsW<TcsetSpec> {
        TrbsW::new(self, 0)
    }
}
#[doc = "Slice Timer Run Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcsetSpec;
impl crate::RegisterSpec for TcsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tcset::W`](W) writer structure"]
impl crate::Writable for TcsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCSET to value 0"]
impl crate::Resettable for TcsetSpec {
    const RESET_VALUE: u32 = 0;
}
