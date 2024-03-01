#[doc = "Register `FPCS` reader"]
pub type R = crate::R<FpcsSpec>;
#[doc = "Register `FPCS` writer"]
pub type W = crate::W<FpcsSpec>;
#[doc = "Field `PCMP` reader - Floating Prescaler Shadow Compare Value"]
pub type PcmpR = crate::FieldReader;
#[doc = "Field `PCMP` writer - Floating Prescaler Shadow Compare Value"]
pub type PcmpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    pub fn pcmp(&self) -> PcmpR {
        PcmpR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Floating Prescaler Shadow Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn pcmp(&mut self) -> PcmpW<FpcsSpec> {
        PcmpW::new(self, 0)
    }
}
#[doc = "Floating Prescaler Shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpcsSpec;
impl crate::RegisterSpec for FpcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpcs::R`](R) reader structure"]
impl crate::Readable for FpcsSpec {}
#[doc = "`write(|w| ..)` method takes [`fpcs::W`](W) writer structure"]
impl crate::Writable for FpcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPCS to value 0"]
impl crate::Resettable for FpcsSpec {
    const RESET_VALUE: u32 = 0;
}
