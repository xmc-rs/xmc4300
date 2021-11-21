#[doc = "Register `FDR` reader"]
pub struct R(crate::R<FDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDR` writer"]
pub struct W(crate::W<FDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDR_SPEC>;
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
impl From<crate::W<FDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEP` reader - Step Value"]
pub struct STEP_R(crate::FieldReader<u16, u16>);
impl STEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STEP` writer - Step Value"]
pub struct STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Divider Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DM_A {
    #[doc = "0: The divider is switched off, fFD = 0."]
    VALUE1 = 0,
    #[doc = "1: Normal divider mode selected."]
    VALUE2 = 1,
    #[doc = "2: Fractional divider mode selected."]
    VALUE3 = 2,
    #[doc = "3: The divider is switched off, fFD = 0."]
    VALUE4 = 3,
}
impl From<DM_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DM` reader - Divider Mode"]
pub struct DM_R(crate::FieldReader<u8, DM_A>);
impl DM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM_A {
        match self.bits {
            0 => DM_A::VALUE1,
            1 => DM_A::VALUE2,
            2 => DM_A::VALUE3,
            3 => DM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == DM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == DM_A::VALUE4
    }
}
impl core::ops::Deref for DM_R {
    type Target = crate::FieldReader<u8, DM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM` writer - Divider Mode"]
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DM_A::VALUE1)
    }
    #[doc = "Normal divider mode selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DM_A::VALUE2)
    }
    #[doc = "Fractional divider mode selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DM_A::VALUE3)
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `RESULT` reader - Result Value"]
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W {
        STEP_W { w: self }
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](index.html) module"]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdr::R](R) reader structure"]
impl crate::Readable for FDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdr::W](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
