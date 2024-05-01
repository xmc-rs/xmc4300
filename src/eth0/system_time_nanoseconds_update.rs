#[doc = "Register `SYSTEM_TIME_NANOSECONDS_UPDATE` reader"]
pub type R = crate::R<SystemTimeNanosecondsUpdateSpec>;
#[doc = "Register `SYSTEM_TIME_NANOSECONDS_UPDATE` writer"]
pub type W = crate::W<SystemTimeNanosecondsUpdateSpec>;
#[doc = "Field `TSSS` reader - Timestamp Sub Second"]
pub type TsssR = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - Timestamp Sub Second"]
pub type TsssW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADDSUB` reader - Add or subtract time"]
pub type AddsubR = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Add or subtract time"]
pub type AddsubW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Second"]
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    pub fn addsub(&self) -> AddsubR {
        AddsubR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub Second"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TsssW<SystemTimeNanosecondsUpdateSpec> {
        TsssW::new(self, 0)
    }
    #[doc = "Bit 31 - Add or subtract time"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> AddsubW<SystemTimeNanosecondsUpdateSpec> {
        AddsubW::new(self, 31)
    }
}
#[doc = "System Time Nanoseconds Update Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_time_nanoseconds_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_time_nanoseconds_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemTimeNanosecondsUpdateSpec;
impl crate::RegisterSpec for SystemTimeNanosecondsUpdateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_time_nanoseconds_update::R`](R) reader structure"]
impl crate::Readable for SystemTimeNanosecondsUpdateSpec {}
#[doc = "`write(|w| ..)` method takes [`system_time_nanoseconds_update::W`](W) writer structure"]
impl crate::Writable for SystemTimeNanosecondsUpdateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEM_TIME_NANOSECONDS_UPDATE to value 0"]
impl crate::Resettable for SystemTimeNanosecondsUpdateSpec {
    const RESET_VALUE: u32 = 0;
}
