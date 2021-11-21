#[doc = "Register `GLOBRES` reader"]
pub struct R(crate::R<GLOBRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBRES` writer"]
pub struct W(crate::W<GLOBRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBRES_SPEC>;
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
impl From<crate::W<GLOBRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
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
#[doc = "Field `RESULT` writer - Result of most recent conversion"]
pub struct RESULT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `GNR` reader - Group Number"]
pub struct GNR_R(crate::FieldReader<u8, u8>);
impl GNR_R {
    pub(crate) fn new(bits: u8) -> Self {
        GNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GNR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHNR` reader - Channel Number"]
pub struct CHNR_R(crate::FieldReader<u8, u8>);
impl CHNR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMUX` reader - External Multiplexer Setting"]
pub struct EMUX_R(crate::FieldReader<u8, u8>);
impl EMUX_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRS` reader - Converted Request Source"]
pub struct CRS_R(crate::FieldReader<u8, u8>);
impl CRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fast Compare Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCR_A {
    #[doc = "0: Signal level was below compare value"]
    VALUE1 = 0,
    #[doc = "1: Signal level was above compare value"]
    VALUE2 = 1,
}
impl From<FCR_A> for bool {
    #[inline(always)]
    fn from(variant: FCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCR` reader - Fast Compare Result"]
pub struct FCR_R(crate::FieldReader<bool, FCR_A>);
impl FCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCR_A {
        match self.bits {
            false => FCR_A::VALUE1,
            true => FCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == FCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FCR_A::VALUE2
    }
}
impl core::ops::Deref for FCR_R {
    type Target = crate::FieldReader<bool, FCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF_A> for bool {
    #[inline(always)]
    fn from(variant: VF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF` reader - Valid Flag"]
pub struct VF_R(crate::FieldReader<bool, VF_A>);
impl VF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF_A {
        match self.bits {
            false => VF_A::VALUE1,
            true => VF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VF_A::VALUE2
    }
}
impl core::ops::Deref for VF_R {
    type Target = crate::FieldReader<bool, VF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VF` writer - Valid Flag"]
pub struct VF_W<'a> {
    w: &'a mut W,
}
impl<'a> VF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF_A::VALUE1)
    }
    #[doc = "Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Group Number"]
    #[inline(always)]
    pub fn gnr(&self) -> GNR_R {
        GNR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - Channel Number"]
    #[inline(always)]
    pub fn chnr(&self) -> CHNR_R {
        CHNR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27 - External Multiplexer Setting"]
    #[inline(always)]
    pub fn emux(&self) -> EMUX_R {
        EMUX_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:29 - Converted Request Source"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Fast Compare Result"]
    #[inline(always)]
    pub fn fcr(&self) -> FCR_R {
        FCR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&mut self) -> RESULT_W {
        RESULT_W { w: self }
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&mut self) -> VF_W {
        VF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globres](index.html) module"]
pub struct GLOBRES_SPEC;
impl crate::RegisterSpec for GLOBRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globres::R](R) reader structure"]
impl crate::Readable for GLOBRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globres::W](W) writer structure"]
impl crate::Writable for GLOBRES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOBRES to value 0"]
impl crate::Resettable for GLOBRES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
