#[doc = "Register `SRACT` writer"]
pub struct W(crate::W<SRACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRACT_SPEC>;
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
impl From<crate::W<SRACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Activate Group Service Request Node 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGSR0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR0_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR0` writer - Activate Group Service Request Node 0"]
pub struct AGSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> AGSR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGSR0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR0_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Activate Group Service Request Node 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGSR1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR1_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR1` writer - Activate Group Service Request Node 1"]
pub struct AGSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AGSR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGSR1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR1_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Activate Group Service Request Node 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGSR2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR2_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR2` writer - Activate Group Service Request Node 2"]
pub struct AGSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> AGSR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGSR2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR2_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR2_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Activate Group Service Request Node 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGSR3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<AGSR3_AW> for bool {
    #[inline(always)]
    fn from(variant: AGSR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AGSR3` writer - Activate Group Service Request Node 3"]
pub struct AGSR3_W<'a> {
    w: &'a mut W,
}
impl<'a> AGSR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AGSR3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AGSR3_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AGSR3_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Activate Shared Service Request Node 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSR0_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR0_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR0` writer - Activate Shared Service Request Node 0"]
pub struct ASSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSR0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR0_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR0_AW::VALUE2)
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
#[doc = "Activate Shared Service Request Node 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSR1_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR1_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR1` writer - Activate Shared Service Request Node 1"]
pub struct ASSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSR1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR1_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Activate Shared Service Request Node 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSR2_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR2_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR2` writer - Activate Shared Service Request Node 2"]
pub struct ASSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSR2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR2_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR2_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Activate Shared Service Request Node 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASSR3_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Activate the associated service request line"]
    VALUE2 = 1,
}
impl From<ASSR3_AW> for bool {
    #[inline(always)]
    fn from(variant: ASSR3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASSR3` writer - Activate Shared Service Request Node 3"]
pub struct ASSR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASSR3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ASSR3_AW::VALUE1)
    }
    #[doc = "Activate the associated service request line"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ASSR3_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Activate Group Service Request Node 0"]
    #[inline(always)]
    pub fn agsr0(&mut self) -> AGSR0_W {
        AGSR0_W { w: self }
    }
    #[doc = "Bit 1 - Activate Group Service Request Node 1"]
    #[inline(always)]
    pub fn agsr1(&mut self) -> AGSR1_W {
        AGSR1_W { w: self }
    }
    #[doc = "Bit 2 - Activate Group Service Request Node 2"]
    #[inline(always)]
    pub fn agsr2(&mut self) -> AGSR2_W {
        AGSR2_W { w: self }
    }
    #[doc = "Bit 3 - Activate Group Service Request Node 3"]
    #[inline(always)]
    pub fn agsr3(&mut self) -> AGSR3_W {
        AGSR3_W { w: self }
    }
    #[doc = "Bit 8 - Activate Shared Service Request Node 0"]
    #[inline(always)]
    pub fn assr0(&mut self) -> ASSR0_W {
        ASSR0_W { w: self }
    }
    #[doc = "Bit 9 - Activate Shared Service Request Node 1"]
    #[inline(always)]
    pub fn assr1(&mut self) -> ASSR1_W {
        ASSR1_W { w: self }
    }
    #[doc = "Bit 10 - Activate Shared Service Request Node 2"]
    #[inline(always)]
    pub fn assr2(&mut self) -> ASSR2_W {
        ASSR2_W { w: self }
    }
    #[doc = "Bit 11 - Activate Shared Service Request Node 3"]
    #[inline(always)]
    pub fn assr3(&mut self) -> ASSR3_W {
        ASSR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service Request Software Activation Trigger\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sract](index.html) module"]
pub struct SRACT_SPEC;
impl crate::RegisterSpec for SRACT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sract::W](W) writer structure"]
impl crate::Writable for SRACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRACT to value 0"]
impl crate::Resettable for SRACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
