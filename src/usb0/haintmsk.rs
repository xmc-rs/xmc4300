#[doc = "Register `HAINTMSK` reader"]
pub type R = crate::R<HaintmskSpec>;
#[doc = "Register `HAINTMSK` writer"]
pub type W = crate::W<HaintmskSpec>;
#[doc = "Field `HAINTMsk` reader - Channel Interrupt Mask"]
pub type HaintmskR = crate::FieldReader<u16>;
#[doc = "Field `HAINTMsk` writer - Channel Interrupt Mask"]
pub type HaintmskW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Channel Interrupt Mask"]
    #[inline(always)]
    pub fn haintmsk(&self) -> HaintmskR {
        HaintmskR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Channel Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn haintmsk(&mut self) -> HaintmskW<HaintmskSpec> {
        HaintmskW::new(self, 0)
    }
}
#[doc = "Host All Channels Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HaintmskSpec;
impl crate::RegisterSpec for HaintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haintmsk::R`](R) reader structure"]
impl crate::Readable for HaintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`haintmsk::W`](W) writer structure"]
impl crate::Writable for HaintmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HAINTMSK to value 0"]
impl crate::Resettable for HaintmskSpec {
    const RESET_VALUE: u32 = 0;
}
