#[doc = "Register `QCTRL0` reader"]
pub struct R(crate::R<QCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QCTRL0` writer"]
pub struct W(crate::W<QCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QCTRL0_SPEC>;
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
impl From<crate::W<QCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Source-specific Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRCRESREG_A {
    #[doc = "0: Use GxCHCTRy.RESREG to select a group result register"]
    VALUE1 = 0,
    #[doc = "1: Store result in group result register GxRES1"]
    VALUE2 = 1,
    #[doc = "15: Store result in group result register GxRES15"]
    VALUE3 = 15,
}
impl From<SRCRESREG_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCRESREG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRCRESREG` reader - Source-specific Result Register"]
pub struct SRCRESREG_R(crate::FieldReader<u8, SRCRESREG_A>);
impl SRCRESREG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRCRESREG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRCRESREG_A> {
        match self.bits {
            0 => Some(SRCRESREG_A::VALUE1),
            1 => Some(SRCRESREG_A::VALUE2),
            15 => Some(SRCRESREG_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRCRESREG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SRCRESREG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SRCRESREG_A::VALUE3
    }
}
impl core::ops::Deref for SRCRESREG_R {
    type Target = crate::FieldReader<u8, SRCRESREG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCRESREG` writer - Source-specific Result Register"]
pub struct SRCRESREG_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCRESREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCRESREG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRCRESREG_A::VALUE1)
    }
    #[doc = "Store result in group result register GxRES1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRCRESREG_A::VALUE2)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRCRESREG_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `XTSEL` reader - External Trigger Input Selection"]
pub struct XTSEL_R(crate::FieldReader<u8, u8>);
impl XTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTSEL` writer - External Trigger Input Selection"]
pub struct XTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `XTLVL` reader - External Trigger Level"]
pub struct XTLVL_R(crate::FieldReader<bool, bool>);
impl XTLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Trigger Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTMODE_A {
    #[doc = "0: No external trigger"]
    VALUE1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    VALUE2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    VALUE3 = 2,
    #[doc = "3: Trigger event upon any edge"]
    VALUE4 = 3,
}
impl From<XTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: XTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XTMODE` reader - Trigger Operating Mode"]
pub struct XTMODE_R(crate::FieldReader<u8, XTMODE_A>);
impl XTMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTMODE_A {
        match self.bits {
            0 => XTMODE_A::VALUE1,
            1 => XTMODE_A::VALUE2,
            2 => XTMODE_A::VALUE3,
            3 => XTMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == XTMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == XTMODE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == XTMODE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == XTMODE_A::VALUE4
    }
}
impl core::ops::Deref for XTMODE_R {
    type Target = crate::FieldReader<u8, XTMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTMODE` writer - Trigger Operating Mode"]
pub struct XTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE3)
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(XTMODE_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Write Control for Trigger Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTWC_AW {
    #[doc = "0: No write access to trigger configuration"]
    VALUE1 = 0,
    #[doc = "1: Bitfields XTMODE and XTSEL can be written"]
    VALUE2 = 1,
}
impl From<XTWC_AW> for bool {
    #[inline(always)]
    fn from(variant: XTWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTWC` writer - Write Control for Trigger Configuration"]
pub struct XTWC_W<'a> {
    w: &'a mut W,
}
impl<'a> XTWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTWC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write access to trigger configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XTWC_AW::VALUE1)
    }
    #[doc = "Bitfields XTMODE and XTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XTWC_AW::VALUE2)
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
#[doc = "Field `GTSEL` reader - Gate Input Selection"]
pub struct GTSEL_R(crate::FieldReader<u8, u8>);
impl GTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTSEL` writer - Gate Input Selection"]
pub struct GTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `GTLVL` reader - Gate Input Level"]
pub struct GTLVL_R(crate::FieldReader<bool, bool>);
impl GTLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GTLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTLVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write Control for Gate Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTWC_AW {
    #[doc = "0: No write access to gate configuration"]
    VALUE1 = 0,
    #[doc = "1: Bitfield GTSEL can be written"]
    VALUE2 = 1,
}
impl From<GTWC_AW> for bool {
    #[inline(always)]
    fn from(variant: GTWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTWC` writer - Write Control for Gate Configuration"]
pub struct GTWC_W<'a> {
    w: &'a mut W,
}
impl<'a> GTWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GTWC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write access to gate configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GTWC_AW::VALUE1)
    }
    #[doc = "Bitfield GTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GTWC_AW::VALUE2)
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
#[doc = "Timer Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMEN_A {
    #[doc = "0: No timer mode: standard gating mechanism can be used"]
    VALUE1 = 0,
    #[doc = "1: Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    VALUE2 = 1,
}
impl From<TMEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMEN` reader - Timer Mode Enable"]
pub struct TMEN_R(crate::FieldReader<bool, TMEN_A>);
impl TMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMEN_A {
        match self.bits {
            false => TMEN_A::VALUE1,
            true => TMEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TMEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TMEN_A::VALUE2
    }
}
impl core::ops::Deref for TMEN_R {
    type Target = crate::FieldReader<bool, TMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMEN` writer - Timer Mode Enable"]
pub struct TMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No timer mode: standard gating mechanism can be used"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE1)
    }
    #[doc = "Timer mode for equidistant sampling enabled: standard gating mechanism must be disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMEN_A::VALUE2)
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
#[doc = "Write Control for Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMWC_AW {
    #[doc = "0: No write access to timer mode"]
    VALUE1 = 0,
    #[doc = "1: Bitfield TMEN can be written"]
    VALUE2 = 1,
}
impl From<TMWC_AW> for bool {
    #[inline(always)]
    fn from(variant: TMWC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMWC` writer - Write Control for Timer Mode"]
pub struct TMWC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMWC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write access to timer mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TMWC_AW::VALUE1)
    }
    #[doc = "Bitfield TMEN can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TMWC_AW::VALUE2)
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
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    pub fn srcresreg(&self) -> SRCRESREG_R {
        SRCRESREG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    pub fn xtsel(&self) -> XTSEL_R {
        XTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Trigger Level"]
    #[inline(always)]
    pub fn xtlvl(&self) -> XTLVL_R {
        XTLVL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    pub fn xtmode(&self) -> XTMODE_R {
        XTMODE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    pub fn gtsel(&self) -> GTSEL_R {
        GTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Gate Input Level"]
    #[inline(always)]
    pub fn gtlvl(&self) -> GTLVL_R {
        GTLVL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline(always)]
    pub fn tmen(&self) -> TMEN_R {
        TMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    pub fn srcresreg(&mut self) -> SRCRESREG_W {
        SRCRESREG_W { w: self }
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    pub fn xtsel(&mut self) -> XTSEL_W {
        XTSEL_W { w: self }
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    pub fn xtmode(&mut self) -> XTMODE_W {
        XTMODE_W { w: self }
    }
    #[doc = "Bit 15 - Write Control for Trigger Configuration"]
    #[inline(always)]
    pub fn xtwc(&mut self) -> XTWC_W {
        XTWC_W { w: self }
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    pub fn gtsel(&mut self) -> GTSEL_W {
        GTSEL_W { w: self }
    }
    #[doc = "Bit 23 - Write Control for Gate Configuration"]
    #[inline(always)]
    pub fn gtwc(&mut self) -> GTWC_W {
        GTWC_W { w: self }
    }
    #[doc = "Bit 28 - Timer Mode Enable"]
    #[inline(always)]
    pub fn tmen(&mut self) -> TMEN_W {
        TMEN_W { w: self }
    }
    #[doc = "Bit 31 - Write Control for Timer Mode"]
    #[inline(always)]
    pub fn tmwc(&mut self) -> TMWC_W {
        TMWC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Queue 0 Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qctrl0](index.html) module"]
pub struct QCTRL0_SPEC;
impl crate::RegisterSpec for QCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qctrl0::R](R) reader structure"]
impl crate::Readable for QCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qctrl0::W](W) writer structure"]
impl crate::Writable for QCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QCTRL0 to value 0"]
impl crate::Resettable for QCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
