#[doc = "Register `DVBUSDIS` reader"]
pub type R = crate::R<DvbusdisSpec>;
#[doc = "Register `DVBUSDIS` writer"]
pub type W = crate::W<DvbusdisSpec>;
#[doc = "Field `DVBUSDis` reader - Device Vbus Discharge Time"]
pub type DvbusdisR = crate::FieldReader<u16>;
#[doc = "Field `DVBUSDis` writer - Device Vbus Discharge Time"]
pub type DvbusdisW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Device Vbus Discharge Time"]
    #[inline(always)]
    pub fn dvbusdis(&self) -> DvbusdisR {
        DvbusdisR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device Vbus Discharge Time"]
    #[inline(always)]
    #[must_use]
    pub fn dvbusdis(&mut self) -> DvbusdisW<DvbusdisSpec> {
        DvbusdisW::new(self, 0)
    }
}
#[doc = "Device VBUS Discharge Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvbusdisSpec;
impl crate::RegisterSpec for DvbusdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbusdis::R`](R) reader structure"]
impl crate::Readable for DvbusdisSpec {}
#[doc = "`write(|w| ..)` method takes [`dvbusdis::W`](W) writer structure"]
impl crate::Writable for DvbusdisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DVBUSDIS to value 0x17d7"]
impl crate::Resettable for DvbusdisSpec {
    const RESET_VALUE: u32 = 0x17d7;
}
