#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `CMF` reader - CRC Mismatch Flag"]
pub type CMF_R = crate::BitReader;
#[doc = "Field `CMF` writer - CRC Mismatch Flag"]
pub type CMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEF` reader - Configuration Error Flag"]
pub type CEF_R = crate::BitReader;
#[doc = "Field `CEF` writer - Configuration Error Flag"]
pub type CEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEF` reader - Length Error Flag"]
pub type LEF_R = crate::BitReader;
#[doc = "Field `LEF` writer - Length Error Flag"]
pub type LEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEF` reader - Bus Error Flag"]
pub type BEF_R = crate::BitReader;
#[doc = "Field `BEF` writer - Bus Error Flag"]
pub type BEF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    pub fn cef(&self) -> CEF_R {
        CEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    pub fn lef(&self) -> LEF_R {
        LEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    pub fn bef(&self) -> BEF_R {
        BEF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Mismatch Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmf(&mut self) -> CMF_W<STS_SPEC> {
        CMF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cef(&mut self) -> CEF_W<STS_SPEC> {
        CEF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lef(&mut self) -> LEF_W<STS_SPEC> {
        LEF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bef(&mut self) -> BEF_W<STS_SPEC> {
        BEF_W::new(self, 3)
    }
}
#[doc = "CRC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
