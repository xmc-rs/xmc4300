#[doc = "Register `ARBCFG` reader"]
pub struct R(crate::R<ARBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARBCFG` writer"]
pub struct W(crate::W<ARBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARBCFG_SPEC>;
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
impl From<crate::W<ARBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANONC` reader - Analog Converter Control"]
pub struct ANONC_R(crate::FieldReader<u8, u8>);
impl ANONC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANONC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANONC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANONC` writer - Analog Converter Control"]
pub struct ANONC_W<'a> {
    w: &'a mut W,
}
impl<'a> ANONC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Arbitration Round Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARBRND_A {
    #[doc = "0: 4 arbitration slots per round (tARB = 4 / fADCD)"]
    VALUE1 = 0,
    #[doc = "1: 8 arbitration slots per round (tARB = 8 / fADCD)"]
    VALUE2 = 1,
    #[doc = "2: 16 arbitration slots per round (tARB = 16 / fADCD)"]
    VALUE3 = 2,
    #[doc = "3: 20 arbitration slots per round (tARB = 20 / fADCD)"]
    VALUE4 = 3,
}
impl From<ARBRND_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBRND_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ARBRND` reader - Arbitration Round Length"]
pub struct ARBRND_R(crate::FieldReader<u8, ARBRND_A>);
impl ARBRND_R {
    pub(crate) fn new(bits: u8) -> Self {
        ARBRND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBRND_A {
        match self.bits {
            0 => ARBRND_A::VALUE1,
            1 => ARBRND_A::VALUE2,
            2 => ARBRND_A::VALUE3,
            3 => ARBRND_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ARBRND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ARBRND_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ARBRND_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ARBRND_A::VALUE4
    }
}
impl core::ops::Deref for ARBRND_R {
    type Target = crate::FieldReader<u8, ARBRND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBRND` writer - Arbitration Round Length"]
pub struct ARBRND_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBRND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBRND_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "4 arbitration slots per round (tARB = 4 / fADCD)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBRND_A::VALUE1)
    }
    #[doc = "8 arbitration slots per round (tARB = 8 / fADCD)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBRND_A::VALUE2)
    }
    #[doc = "16 arbitration slots per round (tARB = 16 / fADCD)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBRND_A::VALUE3)
    }
    #[doc = "20 arbitration slots per round (tARB = 20 / fADCD)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBRND_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Arbitration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBM_A {
    #[doc = "0: The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    VALUE1 = 0,
    #[doc = "1: The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    VALUE2 = 1,
}
impl From<ARBM_A> for bool {
    #[inline(always)]
    fn from(variant: ARBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBM` reader - Arbitration Mode"]
pub struct ARBM_R(crate::FieldReader<bool, ARBM_A>);
impl ARBM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBM_A {
        match self.bits {
            false => ARBM_A::VALUE1,
            true => ARBM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ARBM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ARBM_A::VALUE2
    }
}
impl core::ops::Deref for ARBM_R {
    type Target = crate::FieldReader<bool, ARBM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBM` writer - Arbitration Mode"]
pub struct ARBM_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBM_A::VALUE1)
    }
    #[doc = "The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBM_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Analog Converter Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANONS_A {
    #[doc = "0: Analog converter off"]
    VALUE1 = 0,
    #[doc = "3: Normal operation (permanently on)"]
    VALUE4 = 3,
}
impl From<ANONS_A> for u8 {
    #[inline(always)]
    fn from(variant: ANONS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ANONS` reader - Analog Converter Control Status"]
pub struct ANONS_R(crate::FieldReader<u8, ANONS_A>);
impl ANONS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANONS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANONS_A> {
        match self.bits {
            0 => Some(ANONS_A::VALUE1),
            3 => Some(ANONS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ANONS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == ANONS_A::VALUE4
    }
}
impl core::ops::Deref for ANONS_R {
    type Target = crate::FieldReader<u8, ANONS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Start-Up Calibration Active Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_A {
    #[doc = "0: Completed or not yet started"]
    VALUE1 = 0,
    #[doc = "1: Start-up calibration phase is active"]
    VALUE2 = 1,
}
impl From<CAL_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL` reader - Start-Up Calibration Active Indication"]
pub struct CAL_R(crate::FieldReader<bool, CAL_A>);
impl CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_A {
        match self.bits {
            false => CAL_A::VALUE1,
            true => CAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CAL_A::VALUE2
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<bool, CAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Converter Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Not busy"]
    VALUE1 = 0,
    #[doc = "1: Converter is busy with a conversion"]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Converter Busy Flag"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUSY_A::VALUE2
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Sample Phase Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLE_A {
    #[doc = "0: Converting or idle"]
    VALUE1 = 0,
    #[doc = "1: Input signal is currently sampled"]
    VALUE2 = 1,
}
impl From<SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLE` reader - Sample Phase Flag"]
pub struct SAMPLE_R(crate::FieldReader<bool, SAMPLE_A>);
impl SAMPLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAMPLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLE_A {
        match self.bits {
            false => SAMPLE_A::VALUE1,
            true => SAMPLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SAMPLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SAMPLE_A::VALUE2
    }
}
impl core::ops::Deref for SAMPLE_R {
    type Target = crate::FieldReader<bool, SAMPLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    pub fn anonc(&self) -> ANONC_R {
        ANONC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    pub fn arbrnd(&self) -> ARBRND_R {
        ARBRND_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    pub fn arbm(&self) -> ARBM_R {
        ARBM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Analog Converter Control Status"]
    #[inline(always)]
    pub fn anons(&self) -> ANONS_R {
        ANONS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Start-Up Calibration Active Indication"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Converter Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Sample Phase Flag"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    pub fn anonc(&mut self) -> ANONC_W {
        ANONC_W { w: self }
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    pub fn arbrnd(&mut self) -> ARBRND_W {
        ARBRND_W { w: self }
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    pub fn arbm(&mut self) -> ARBM_W {
        ARBM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Arbitration Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arbcfg](index.html) module"]
pub struct ARBCFG_SPEC;
impl crate::RegisterSpec for ARBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arbcfg::R](R) reader structure"]
impl crate::Readable for ARBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arbcfg::W](W) writer structure"]
impl crate::Writable for ARBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARBCFG to value 0"]
impl crate::Resettable for ARBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
