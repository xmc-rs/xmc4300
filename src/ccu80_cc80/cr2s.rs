#[doc = "Register `CR2S` reader"]
pub type R = crate::R<Cr2sSpec>;
#[doc = "Register `CR2S` writer"]
pub type W = crate::W<Cr2sSpec>;
#[doc = "Field `CR2S` reader - Shadow Compare Register for Channel 2"]
pub type Cr2sR = crate::FieldReader<u16>;
#[doc = "Field `CR2S` writer - Shadow Compare Register for Channel 2"]
pub type Cr2sW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    pub fn cr2s(&self) -> Cr2sR {
        Cr2sR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow Compare Register for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr2s(&mut self) -> Cr2sW<Cr2sSpec> {
        Cr2sW::new(self, 0)
    }
}
#[doc = "Channel 2 Compare Shadow Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2sSpec;
impl crate::RegisterSpec for Cr2sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2s::R`](R) reader structure"]
impl crate::Readable for Cr2sSpec {}
#[doc = "`write(|w| ..)` method takes [`cr2s::W`](W) writer structure"]
impl crate::Writable for Cr2sSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2S to value 0"]
impl crate::Resettable for Cr2sSpec {
    const RESET_VALUE: u32 = 0;
}
