#[doc = "Register `MII_PHY_DATA` reader"]
pub struct R(crate::R<MII_PHY_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MII_PHY_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MII_PHY_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MII_PHY_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MII_PHY_DATA` writer"]
pub struct W(crate::W<MII_PHY_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MII_PHY_DATA_SPEC>;
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
impl From<crate::W<MII_PHY_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MII_PHY_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_RW_DATA` reader - PHY Read/Write Data"]
pub struct PHY_RW_DATA_R(crate::FieldReader<u16, u16>);
impl PHY_RW_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        PHY_RW_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHY_RW_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHY_RW_DATA` writer - PHY Read/Write Data"]
pub struct PHY_RW_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHY_RW_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    pub fn phy_rw_data(&self) -> PHY_RW_DATA_R {
        PHY_RW_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    pub fn phy_rw_data(&mut self) -> PHY_RW_DATA_W {
        PHY_RW_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_phy_data](index.html) module"]
pub struct MII_PHY_DATA_SPEC;
impl crate::RegisterSpec for MII_PHY_DATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mii_phy_data::R](R) reader structure"]
impl crate::Readable for MII_PHY_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mii_phy_data::W](W) writer structure"]
impl crate::Writable for MII_PHY_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MII_PHY_DATA to value 0"]
impl crate::Resettable for MII_PHY_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
