#[doc = "Register `CLC` reader"]
pub type R = crate::R<ClcSpec>;
#[doc = "Register `CLC` writer"]
pub type W = crate::W<ClcSpec>;
#[doc = "Field `DISR` reader - Module Disable Request Bit"]
pub type DisrR = crate::BitReader;
#[doc = "Field `DISR` writer - Module Disable Request Bit"]
pub type DisrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISS` reader - Module Disable Status Bit"]
pub type DissR = crate::BitReader;
#[doc = "Field `EDIS` reader - Sleep Mode Enable Control"]
pub type EdisR = crate::BitReader;
#[doc = "Field `EDIS` writer - Sleep Mode Enable Control"]
pub type EdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    pub fn disr(&self) -> DisrR {
        DisrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(&self) -> DissR {
        DissR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    pub fn edis(&self) -> EdisR {
        EdisR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Disable Request Bit"]
    #[inline(always)]
    #[must_use]
    pub fn disr(&mut self) -> DisrW<ClcSpec> {
        DisrW::new(self, 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn edis(&mut self) -> EdisW<ClcSpec> {
        EdisW::new(self, 3)
    }
}
#[doc = "CAN Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClcSpec;
impl crate::RegisterSpec for ClcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clc::R`](R) reader structure"]
impl crate::Readable for ClcSpec {}
#[doc = "`write(|w| ..)` method takes [`clc::W`](W) writer structure"]
impl crate::Writable for ClcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLC to value 0x03"]
impl crate::Resettable for ClcSpec {
    const RESET_VALUE: u32 = 0x03;
}
