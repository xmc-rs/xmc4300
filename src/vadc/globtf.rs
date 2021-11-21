#[doc = "Register `GLOBTF` reader"]
pub struct R(crate::R<GLOBTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBTF` writer"]
pub struct W(crate::W<GLOBTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBTF_SPEC>;
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
impl From<crate::W<GLOBTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDGR` reader - Converter Diagnostics Group"]
pub struct CDGR_R(crate::FieldReader<u8, u8>);
impl CDGR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CDGR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDGR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDGR` writer - Converter Diagnostics Group"]
pub struct CDGR_W<'a> {
    w: &'a mut W,
}
impl<'a> CDGR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Converter Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDEN_A {
    #[doc = "0: All diagnostic pull devices are disconnected"]
    VALUE1 = 0,
    #[doc = "1: Diagnostic pull devices connected as selected by bitfield CDSEL"]
    VALUE2 = 1,
}
impl From<CDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN` reader - Converter Diagnostics Enable"]
pub struct CDEN_R(crate::FieldReader<bool, CDEN_A>);
impl CDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDEN_A {
        match self.bits {
            false => CDEN_A::VALUE1,
            true => CDEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CDEN_A::VALUE2
    }
}
impl core::ops::Deref for CDEN_R {
    type Target = crate::FieldReader<bool, CDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDEN` writer - Converter Diagnostics Enable"]
pub struct CDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All diagnostic pull devices are disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE1)
    }
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Converter Diagnostics Pull-Devices Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CDSEL_A {
    #[doc = "0: Connected to VAREF"]
    VALUE1 = 0,
    #[doc = "1: Connected to VAGND"]
    VALUE2 = 1,
    #[doc = "2: Connected to 1/3rd VAREF"]
    VALUE3 = 2,
    #[doc = "3: Connected to 2/3rd VAREF"]
    VALUE4 = 3,
}
impl From<CDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CDSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CDSEL` reader - Converter Diagnostics Pull-Devices Select"]
pub struct CDSEL_R(crate::FieldReader<u8, CDSEL_A>);
impl CDSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDSEL_A {
        match self.bits {
            0 => CDSEL_A::VALUE1,
            1 => CDSEL_A::VALUE2,
            2 => CDSEL_A::VALUE3,
            3 => CDSEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CDSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CDSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CDSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CDSEL_A::VALUE4
    }
}
impl core::ops::Deref for CDSEL_R {
    type Target = crate::FieldReader<u8, CDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDSEL` writer - Converter Diagnostics Pull-Devices Select"]
pub struct CDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Connected to VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE1)
    }
    #[doc = "Connected to VAGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE2)
    }
    #[doc = "Connected to 1/3rd VAREF"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE3)
    }
    #[doc = "Connected to 2/3rd VAREF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CDSEL_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Write Control for Conversion Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDWC_AW {
    #[doc = "0: No write access to parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfields CDSEL, CDEN, CDGR can be written"]
    VALUE2 = 1,
}
impl From<CDWC_AW> for bool {
    #[inline(always)]
    fn from(variant: CDWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDWC` writer - Write Control for Conversion Diagnostics"]
pub struct CDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> CDWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDWC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CDWC_AW::VALUE1)
    }
    #[doc = "Bitfields CDSEL, CDEN, CDGR can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CDWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Pull-Down Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD_A {
    #[doc = "0: Disconnected"]
    VALUE1 = 0,
    #[doc = "1: The pull-down diagnostics device is active"]
    VALUE2 = 1,
}
impl From<PDD_A> for bool {
    #[inline(always)]
    fn from(variant: PDD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDD` reader - Pull-Down Diagnostics Enable"]
pub struct PDD_R(crate::FieldReader<bool, PDD_A>);
impl PDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD_A {
        match self.bits {
            false => PDD_A::VALUE1,
            true => PDD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PDD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PDD_A::VALUE2
    }
}
impl core::ops::Deref for PDD_R {
    type Target = crate::FieldReader<bool, PDD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDD` writer - Pull-Down Diagnostics Enable"]
pub struct PDD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PDD_A::VALUE1)
    }
    #[doc = "The pull-down diagnostics device is active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PDD_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Write Control for Multiplexer Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDWC_AW {
    #[doc = "0: No write access to parameters"]
    VALUE1 = 0,
    #[doc = "1: Bitfield PDD can be written"]
    VALUE2 = 1,
}
impl From<MDWC_AW> for bool {
    #[inline(always)]
    fn from(variant: MDWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDWC` writer - Write Control for Multiplexer Diagnostics"]
pub struct MDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDWC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDWC_AW::VALUE1)
    }
    #[doc = "Bitfield PDD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDWC_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    pub fn cdgr(&self) -> CDGR_R {
        CDGR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    pub fn cden(&self) -> CDEN_R {
        CDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    pub fn cdsel(&self) -> CDSEL_R {
        CDSEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    pub fn cdgr(&mut self) -> CDGR_W {
        CDGR_W { w: self }
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    pub fn cden(&mut self) -> CDEN_W {
        CDEN_W { w: self }
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    pub fn cdsel(&mut self) -> CDSEL_W {
        CDSEL_W { w: self }
    }
    #[doc = "Bit 15 - Write Control for Conversion Diagnostics"]
    #[inline(always)]
    pub fn cdwc(&mut self) -> CDWC_W {
        CDWC_W { w: self }
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    pub fn pdd(&mut self) -> PDD_W {
        PDD_W { w: self }
    }
    #[doc = "Bit 23 - Write Control for Multiplexer Diagnostics"]
    #[inline(always)]
    pub fn mdwc(&mut self) -> MDWC_W {
        MDWC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Test Functions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [globtf](index.html) module"]
pub struct GLOBTF_SPEC;
impl crate::RegisterSpec for GLOBTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [globtf::R](R) reader structure"]
impl crate::Readable for GLOBTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [globtf::W](W) writer structure"]
impl crate::Writable for GLOBTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOBTF to value 0"]
impl crate::Resettable for GLOBTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
