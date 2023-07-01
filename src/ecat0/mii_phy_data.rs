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
pub type PHY_RW_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RW_DATA` writer - PHY Read/Write Data"]
pub type PHY_RW_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, MII_PHY_DATA_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    pub fn phy_rw_data(&self) -> PHY_RW_DATA_R {
        PHY_RW_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY Read/Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rw_data(&mut self) -> PHY_RW_DATA_W<0> {
        PHY_RW_DATA_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MII_PHY_DATA to value 0"]
impl crate::Resettable for MII_PHY_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
