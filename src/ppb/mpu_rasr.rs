#[doc = "Register `MPU_RASR` reader"]
pub struct R(crate::R<MPU_RASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RASR` writer"]
pub struct W(crate::W<MPU_RASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RASR_SPEC>;
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
impl From<crate::W<MPU_RASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Region enable bit."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Region enable bit."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `SIZE` reader - MPU protection region size"]
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - MPU protection region size"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Subregion disable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRD_A {
    #[doc = "0: corresponding sub-region is enabled"]
    VALUE1 = 0,
    #[doc = "1: corresponding sub-region is disabled"]
    VALUE2 = 1,
}
impl From<SRD_A> for u8 {
    #[inline(always)]
    fn from(variant: SRD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRD` reader - Subregion disable bits"]
pub struct SRD_R(crate::FieldReader<u8, SRD_A>);
impl SRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRD_A> {
        match self.bits {
            0 => Some(SRD_A::VALUE1),
            1 => Some(SRD_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRD_A::VALUE2
    }
}
impl core::ops::Deref for SRD_R {
    type Target = crate::FieldReader<u8, SRD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRD` writer - Subregion disable bits"]
pub struct SRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "corresponding sub-region is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRD_A::VALUE1)
    }
    #[doc = "corresponding sub-region is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRD_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `B` reader - Memory access attribute"]
pub struct B_R(crate::FieldReader<bool, bool>);
impl B_R {
    pub(crate) fn new(bits: bool) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B` writer - Memory access attribute"]
pub struct B_W<'a> {
    w: &'a mut W,
}
impl<'a> B_W<'a> {
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
#[doc = "Field `C` reader - Memory access attribute"]
pub struct C_R(crate::FieldReader<bool, bool>);
impl C_R {
    pub(crate) fn new(bits: bool) -> Self {
        C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C` writer - Memory access attribute"]
pub struct C_W<'a> {
    w: &'a mut W,
}
impl<'a> C_W<'a> {
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
#[doc = "Field `S` reader - Shareable bit"]
pub struct S_R(crate::FieldReader<bool, bool>);
impl S_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S` writer - Shareable bit"]
pub struct S_W<'a> {
    w: &'a mut W,
}
impl<'a> S_W<'a> {
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
#[doc = "Field `TEX` reader - Memory access attribute"]
pub struct TEX_R(crate::FieldReader<u8, u8>);
impl TEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEX` writer - Memory access attribute"]
pub struct TEX_W<'a> {
    w: &'a mut W,
}
impl<'a> TEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `AP` reader - Access permission field"]
pub struct AP_R(crate::FieldReader<u8, u8>);
impl AP_R {
    pub(crate) fn new(bits: u8) -> Self {
        AP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP` writer - Access permission field"]
pub struct AP_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Instruction access disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XN_A {
    #[doc = "0: instruction fetches enabled"]
    VALUE1 = 0,
    #[doc = "1: instruction fetches disabled."]
    VALUE2 = 1,
}
impl From<XN_A> for bool {
    #[inline(always)]
    fn from(variant: XN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XN` reader - Instruction access disable bit"]
pub struct XN_R(crate::FieldReader<bool, XN_A>);
impl XN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XN_A {
        match self.bits {
            false => XN_A::VALUE1,
            true => XN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == XN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == XN_A::VALUE2
    }
}
impl core::ops::Deref for XN_R {
    type Target = crate::FieldReader<bool, XN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XN` writer - Instruction access disable bit"]
pub struct XN_W<'a> {
    w: &'a mut W,
}
impl<'a> XN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "instruction fetches enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XN_A::VALUE1)
    }
    #[doc = "instruction fetches disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XN_A::VALUE2)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    pub fn tex(&self) -> TEX_R {
        TEX_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    pub fn xn(&self) -> XN_R {
        XN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    pub fn srd(&mut self) -> SRD_W {
        SRD_W { w: self }
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W {
        B_W { w: self }
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    pub fn c(&mut self) -> C_W {
        C_W { w: self }
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W {
        S_W { w: self }
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    pub fn tex(&mut self) -> TEX_W {
        TEX_W { w: self }
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    pub fn ap(&mut self) -> AP_W {
        AP_W { w: self }
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    pub fn xn(&mut self) -> XN_W {
        XN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Region Attribute and Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr](index.html) module"]
pub struct MPU_RASR_SPEC;
impl crate::RegisterSpec for MPU_RASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rasr::R](R) reader structure"]
impl crate::Readable for MPU_RASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rasr::W](W) writer structure"]
impl crate::Writable for MPU_RASR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPU_RASR to value 0"]
impl crate::Resettable for MPU_RASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
