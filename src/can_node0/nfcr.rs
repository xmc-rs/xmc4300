#[doc = "Register `NFCR` reader"]
pub struct R(crate::R<NFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCR` writer"]
pub struct W(crate::W<NFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCR_SPEC>;
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
impl From<crate::W<NFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFC` reader - CAN Frame Counter"]
pub struct CFC_R(crate::FieldReader<u16, u16>);
impl CFC_R {
    pub(crate) fn new(bits: u16) -> Self {
        CFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFC` writer - CAN Frame Counter"]
pub struct CFC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CFSEL` reader - CAN Frame Count Selection"]
pub struct CFSEL_R(crate::FieldReader<u8, u8>);
impl CFSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFSEL` writer - CAN Frame Count Selection"]
pub struct CFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "CAN Frame Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFMOD_A {
    #[doc = "0: Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    VALUE1 = 0,
    #[doc = "1: Time Stamp Mode: The frame counter is used to count bit times."]
    VALUE2 = 1,
    #[doc = "2: Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    VALUE3 = 2,
    #[doc = "3: Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    VALUE4 = 3,
}
impl From<CFMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CFMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFMOD` reader - CAN Frame Counter Mode"]
pub struct CFMOD_R(crate::FieldReader<u8, CFMOD_A>);
impl CFMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFMOD_A {
        match self.bits {
            0 => CFMOD_A::VALUE1,
            1 => CFMOD_A::VALUE2,
            2 => CFMOD_A::VALUE3,
            3 => CFMOD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFMOD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFMOD_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CFMOD_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CFMOD_A::VALUE4
    }
}
impl core::ops::Deref for CFMOD_R {
    type Target = crate::FieldReader<u8, CFMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFMOD` writer - CAN Frame Counter Mode"]
pub struct CFMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE1)
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE2)
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE3)
    }
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFMOD_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "CAN Frame Count Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCIE_A {
    #[doc = "0: CAN frame counter overflow interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: CAN frame counter overflow interrupt is enabled."]
    VALUE2 = 1,
}
impl From<CFCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFCIE` reader - CAN Frame Count Interrupt Enable"]
pub struct CFCIE_R(crate::FieldReader<bool, CFCIE_A>);
impl CFCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCIE_A {
        match self.bits {
            false => CFCIE_A::VALUE1,
            true => CFCIE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFCIE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFCIE_A::VALUE2
    }
}
impl core::ops::Deref for CFCIE_R {
    type Target = crate::FieldReader<bool, CFCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFCIE` writer - CAN Frame Count Interrupt Enable"]
pub struct CFCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCIE_A::VALUE1)
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCIE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "CAN Frame Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFCOV_A {
    #[doc = "0: No overflow has occurred since last flag reset."]
    VALUE1 = 0,
    #[doc = "1: An overflow has occurred since last flag reset."]
    VALUE2 = 1,
}
impl From<CFCOV_A> for bool {
    #[inline(always)]
    fn from(variant: CFCOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFCOV` reader - CAN Frame Counter Overflow Flag"]
pub struct CFCOV_R(crate::FieldReader<bool, CFCOV_A>);
impl CFCOV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFCOV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFCOV_A {
        match self.bits {
            false => CFCOV_A::VALUE1,
            true => CFCOV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFCOV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFCOV_A::VALUE2
    }
}
impl core::ops::Deref for CFCOV_R {
    type Target = crate::FieldReader<bool, CFCOV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFCOV` writer - CAN Frame Counter Overflow Flag"]
pub struct CFCOV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCOV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFCOV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCOV_A::VALUE1)
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCOV_A::VALUE2)
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
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&self) -> CFC_R {
        CFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&self) -> CFSEL_R {
        CFSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&self) -> CFMOD_R {
        CFMOD_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&self) -> CFCIE_R {
        CFCIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&self) -> CFCOV_R {
        CFCOV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&mut self) -> CFC_W {
        CFC_W { w: self }
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&mut self) -> CFSEL_W {
        CFSEL_W { w: self }
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&mut self) -> CFMOD_W {
        CFMOD_W { w: self }
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&mut self) -> CFCIE_W {
        CFCIE_W { w: self }
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&mut self) -> CFCOV_W {
        CFCOV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcr](index.html) module"]
pub struct NFCR_SPEC;
impl crate::RegisterSpec for NFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcr::R](R) reader structure"]
impl crate::Readable for NFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcr::W](W) writer structure"]
impl crate::Writable for NFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NFCR to value 0"]
impl crate::Resettable for NFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
