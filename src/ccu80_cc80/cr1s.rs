#[doc = "Register `CR1S` reader"]
pub type R = crate::R<Cr1sSpec>;
#[doc = "Register `CR1S` writer"]
pub type W = crate::W<Cr1sSpec>;
#[doc = "Field `CR1S` reader - Shadow Compare Register for Channel 1"]
pub type Cr1sR = crate::FieldReader<u16>;
#[doc = "Field `CR1S` writer - Shadow Compare Register for Channel 1"]
pub type Cr1sW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    pub fn cr1s(&self) -> Cr1sR {
        Cr1sR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr1s(&mut self) -> Cr1sW<Cr1sSpec> {
        Cr1sW::new(self, 0)
    }
}
#[doc = "Channel 1 Compare Shadow Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1sSpec;
impl crate::RegisterSpec for Cr1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1s::R`](R) reader structure"]
impl crate::Readable for Cr1sSpec {}
#[doc = "`write(|w| ..)` method takes [`cr1s::W`](W) writer structure"]
impl crate::Writable for Cr1sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1S to value 0"]
impl crate::Resettable for Cr1sSpec {
    const RESET_VALUE: u32 = 0;
}
