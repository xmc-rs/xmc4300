#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `CMF` reader - CRC Mismatch Flag"]
pub type CMF_R = crate::BitReader;
#[doc = "Field `CMF` writer - CRC Mismatch Flag"]
pub type CMF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CEF` reader - Configuration Error Flag"]
pub type CEF_R = crate::BitReader;
#[doc = "Field `CEF` writer - Configuration Error Flag"]
pub type CEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEF` reader - Length Error Flag"]
pub type LEF_R = crate::BitReader;
#[doc = "Field `LEF` writer - Length Error Flag"]
pub type LEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEF` reader - Bus Error Flag"]
pub type BEF_R = crate::BitReader;
#[doc = "Field `BEF` writer - Bus Error Flag"]
pub type BEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn cmf(&mut self) -> CMF_W<STS_SPEC, 0> {
        CMF_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cef(&mut self) -> CEF_W<STS_SPEC, 1> {
        CEF_W::new(self)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lef(&mut self) -> LEF_W<STS_SPEC, 2> {
        LEF_W::new(self)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bef(&mut self) -> BEF_W<STS_SPEC, 3> {
        BEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
