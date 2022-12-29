#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMF` reader - CRC Mismatch Flag"]
pub type CMF_R = crate::BitReader<bool>;
#[doc = "Field `CMF` writer - CRC Mismatch Flag"]
pub type CMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `CEF` reader - Configuration Error Flag"]
pub type CEF_R = crate::BitReader<bool>;
#[doc = "Field `CEF` writer - Configuration Error Flag"]
pub type CEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `LEF` reader - Length Error Flag"]
pub type LEF_R = crate::BitReader<bool>;
#[doc = "Field `LEF` writer - Length Error Flag"]
pub type LEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
#[doc = "Field `BEF` reader - Bus Error Flag"]
pub type BEF_R = crate::BitReader<bool>;
#[doc = "Field `BEF` writer - Bus Error Flag"]
pub type BEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STS_SPEC, bool, O>;
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
    pub fn cmf(&mut self) -> CMF_W<0> {
        CMF_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cef(&mut self) -> CEF_W<1> {
        CEF_W::new(self)
    }
    #[doc = "Bit 2 - Length Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lef(&mut self) -> LEF_W<2> {
        LEF_W::new(self)
    }
    #[doc = "Bit 3 - Bus Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bef(&mut self) -> BEF_W<3> {
        BEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
