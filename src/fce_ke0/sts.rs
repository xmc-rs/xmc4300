#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `CMF` reader - CRC Mismatch Flag"]
pub type CmfR = crate::BitReader;
#[doc = "Field `CMF` writer - CRC Mismatch Flag"]
pub type CmfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEF` reader - Configuration Error Flag"]
pub type CefR = crate::BitReader;
#[doc = "Field `CEF` writer - Configuration Error Flag"]
pub type CefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEF` reader - Length Error Flag"]
pub type LefR = crate::BitReader;
#[doc = "Field `LEF` writer - Length Error Flag"]
pub type LefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEF` reader - Bus Error Flag"]
pub type BefR = crate::BitReader;
#[doc = "Field `BEF` writer - Bus Error Flag"]
pub type BefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CmfR {
        CmfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    pub fn cef(&self) -> CefR {
        CefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    pub fn lef(&self) -> LefR {
        LefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    pub fn bef(&self) -> BefR {
        BefR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmf(&mut self) -> CmfW<StsSpec> {
        CmfW::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cef(&mut self) -> CefW<StsSpec> {
        CefW::new(self, 1)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lef(&mut self) -> LefW<StsSpec> {
        LefW::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bef(&mut self) -> BefW<StsSpec> {
        BefW::new(self, 3)
    }
}
#[doc = "CRC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
