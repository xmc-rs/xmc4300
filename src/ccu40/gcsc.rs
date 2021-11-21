#[doc = "Register `GCSC` writer"]
pub struct W(crate::W<GCSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCSC_SPEC>;
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
impl From<crate::W<GCSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0SC` writer - Slice 0 shadow transfer clear"]
pub struct S0SC_W<'a> {
    w: &'a mut W,
}
impl<'a> S0SC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `S0DSC` writer - Slice 0 Dither shadow transfer clear"]
pub struct S0DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S0DSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `S0PSC` writer - Slice 0 Prescaler shadow transfer clear"]
pub struct S0PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S0PSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `S1SC` writer - Slice 1 shadow transfer clear"]
pub struct S1SC_W<'a> {
    w: &'a mut W,
}
impl<'a> S1SC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `S1DSC` writer - Slice 1 Dither shadow transfer clear"]
pub struct S1DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S1DSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `S1PSC` writer - Slice 1 Prescaler shadow transfer clear"]
pub struct S1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S1PSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `S2SC` writer - Slice 2 shadow transfer clear"]
pub struct S2SC_W<'a> {
    w: &'a mut W,
}
impl<'a> S2SC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `S2DSC` writer - Slice 2 Dither shadow transfer clear"]
pub struct S2DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S2DSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `S2PSC` writer - Slice 2 Prescaler shadow transfer clear"]
pub struct S2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S2PSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `S3SC` writer - Slice 3 shadow transfer clear"]
pub struct S3SC_W<'a> {
    w: &'a mut W,
}
impl<'a> S3SC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `S3DSC` writer - Slice 3 Dither shadow transfer clear"]
pub struct S3DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S3DSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `S3PSC` writer - Slice 3 Prescaler shadow transfer clear"]
pub struct S3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> S3PSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `S0STC` writer - Slice 0 status bit clear"]
pub struct S0STC_W<'a> {
    w: &'a mut W,
}
impl<'a> S0STC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `S1STC` writer - Slice 1 status bit clear"]
pub struct S1STC_W<'a> {
    w: &'a mut W,
}
impl<'a> S1STC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `S2STC` writer - Slice 2 status bit clear"]
pub struct S2STC_W<'a> {
    w: &'a mut W,
}
impl<'a> S2STC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `S3STC` writer - Slice 3 status bit clear"]
pub struct S3STC_W<'a> {
    w: &'a mut W,
}
impl<'a> S3STC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Slice 0 shadow transfer clear"]
    #[inline(always)]
    pub fn s0sc(&mut self) -> S0SC_W {
        S0SC_W { w: self }
    }
    #[doc = "Bit 1 - Slice 0 Dither shadow transfer clear"]
    #[inline(always)]
    pub fn s0dsc(&mut self) -> S0DSC_W {
        S0DSC_W { w: self }
    }
    #[doc = "Bit 2 - Slice 0 Prescaler shadow transfer clear"]
    #[inline(always)]
    pub fn s0psc(&mut self) -> S0PSC_W {
        S0PSC_W { w: self }
    }
    #[doc = "Bit 4 - Slice 1 shadow transfer clear"]
    #[inline(always)]
    pub fn s1sc(&mut self) -> S1SC_W {
        S1SC_W { w: self }
    }
    #[doc = "Bit 5 - Slice 1 Dither shadow transfer clear"]
    #[inline(always)]
    pub fn s1dsc(&mut self) -> S1DSC_W {
        S1DSC_W { w: self }
    }
    #[doc = "Bit 6 - Slice 1 Prescaler shadow transfer clear"]
    #[inline(always)]
    pub fn s1psc(&mut self) -> S1PSC_W {
        S1PSC_W { w: self }
    }
    #[doc = "Bit 8 - Slice 2 shadow transfer clear"]
    #[inline(always)]
    pub fn s2sc(&mut self) -> S2SC_W {
        S2SC_W { w: self }
    }
    #[doc = "Bit 9 - Slice 2 Dither shadow transfer clear"]
    #[inline(always)]
    pub fn s2dsc(&mut self) -> S2DSC_W {
        S2DSC_W { w: self }
    }
    #[doc = "Bit 10 - Slice 2 Prescaler shadow transfer clear"]
    #[inline(always)]
    pub fn s2psc(&mut self) -> S2PSC_W {
        S2PSC_W { w: self }
    }
    #[doc = "Bit 12 - Slice 3 shadow transfer clear"]
    #[inline(always)]
    pub fn s3sc(&mut self) -> S3SC_W {
        S3SC_W { w: self }
    }
    #[doc = "Bit 13 - Slice 3 Dither shadow transfer clear"]
    #[inline(always)]
    pub fn s3dsc(&mut self) -> S3DSC_W {
        S3DSC_W { w: self }
    }
    #[doc = "Bit 14 - Slice 3 Prescaler shadow transfer clear"]
    #[inline(always)]
    pub fn s3psc(&mut self) -> S3PSC_W {
        S3PSC_W { w: self }
    }
    #[doc = "Bit 16 - Slice 0 status bit clear"]
    #[inline(always)]
    pub fn s0stc(&mut self) -> S0STC_W {
        S0STC_W { w: self }
    }
    #[doc = "Bit 17 - Slice 1 status bit clear"]
    #[inline(always)]
    pub fn s1stc(&mut self) -> S1STC_W {
        S1STC_W { w: self }
    }
    #[doc = "Bit 18 - Slice 2 status bit clear"]
    #[inline(always)]
    pub fn s2stc(&mut self) -> S2STC_W {
        S2STC_W { w: self }
    }
    #[doc = "Bit 19 - Slice 3 status bit clear"]
    #[inline(always)]
    pub fn s3stc(&mut self) -> S3STC_W {
        S3STC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcsc](index.html) module"]
pub struct GCSC_SPEC;
impl crate::RegisterSpec for GCSC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gcsc::W](W) writer structure"]
impl crate::Writable for GCSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCSC to value 0"]
impl crate::Resettable for GCSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
