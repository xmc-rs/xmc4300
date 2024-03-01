#[doc = "Register `DVBUSPULSE` reader"]
pub type R = crate::R<DvbuspulseSpec>;
#[doc = "Register `DVBUSPULSE` writer"]
pub type W = crate::W<DvbuspulseSpec>;
#[doc = "Field `DVBUSPulse` reader - Device Vbus Pulsing Time"]
pub type DvbuspulseR = crate::FieldReader<u16>;
#[doc = "Field `DVBUSPulse` writer - Device Vbus Pulsing Time"]
pub type DvbuspulseW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Device Vbus Pulsing Time"]
    #[inline(always)]
    pub fn dvbuspulse(&self) -> DvbuspulseR {
        DvbuspulseR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device Vbus Pulsing Time"]
    #[inline(always)]
    #[must_use]
    pub fn dvbuspulse(&mut self) -> DvbuspulseW<DvbuspulseSpec> {
        DvbuspulseW::new(self, 0)
    }
}
#[doc = "Device VBUS Pulsing Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvbuspulseSpec;
impl crate::RegisterSpec for DvbuspulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbuspulse::R`](R) reader structure"]
impl crate::Readable for DvbuspulseSpec {}
#[doc = "`write(|w| ..)` method takes [`dvbuspulse::W`](W) writer structure"]
impl crate::Writable for DvbuspulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DVBUSPULSE to value 0x05b8"]
impl crate::Resettable for DvbuspulseSpec {
    const RESET_VALUE: u32 = 0x05b8;
}
