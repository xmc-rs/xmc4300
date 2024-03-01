#[doc = "Register `TIMESTAMP_ADDEND` reader"]
pub type R = crate::R<TimestampAddendSpec>;
#[doc = "Register `TIMESTAMP_ADDEND` writer"]
pub type W = crate::W<TimestampAddendSpec>;
#[doc = "Field `TSAR` reader - Timestamp Addend Register"]
pub type TsarR = crate::FieldReader<u32>;
#[doc = "Field `TSAR` writer - Timestamp Addend Register"]
pub type TsarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TsarR {
        TsarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    #[must_use]
    pub fn tsar(&mut self) -> TsarW<TimestampAddendSpec> {
        TsarW::new(self, 0)
    }
}
#[doc = "Timestamp Addend Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_addend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_addend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampAddendSpec;
impl crate::RegisterSpec for TimestampAddendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_addend::R`](R) reader structure"]
impl crate::Readable for TimestampAddendSpec {}
#[doc = "`write(|w| ..)` method takes [`timestamp_addend::W`](W) writer structure"]
impl crate::Writable for TimestampAddendSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMESTAMP_ADDEND to value 0"]
impl crate::Resettable for TimestampAddendSpec {
    const RESET_VALUE: u32 = 0;
}
