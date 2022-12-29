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
pub type ANONC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANONC` writer - Analog Converter Control"]
pub type ANONC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARBCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `ARBRND` reader - Arbitration Round Length"]
pub type ARBRND_R = crate::FieldReader<u8, ARBRND_A>;
#[doc = "Arbitration Round Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ARBRND_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ARBRND_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBRND_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ARBRND_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ARBRND_A::VALUE4
    }
}
#[doc = "Field `ARBRND` writer - Arbitration Round Length"]
pub type ARBRND_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ARBCFG_SPEC, u8, ARBRND_A, 2, O>;
impl<'a, const O: u8> ARBRND_W<'a, O> {
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
}
#[doc = "Field `ARBM` reader - Arbitration Mode"]
pub type ARBM_R = crate::BitReader<ARBM_A>;
#[doc = "Arbitration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ARBM_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ARBM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBM_A::VALUE2
    }
}
#[doc = "Field `ARBM` writer - Arbitration Mode"]
pub type ARBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ARBCFG_SPEC, ARBM_A, O>;
impl<'a, const O: u8> ARBM_W<'a, O> {
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
}
#[doc = "Field `ANONS` reader - Analog Converter Control Status"]
pub type ANONS_R = crate::FieldReader<u8, ANONS_A>;
#[doc = "Analog Converter Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ANONS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == ANONS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ANONS_A::VALUE4
    }
}
#[doc = "Field `CAL` reader - Start-Up Calibration Active Indication"]
pub type CAL_R = crate::BitReader<CAL_A>;
#[doc = "Start-Up Calibration Active Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CAL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == CAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAL_A::VALUE2
    }
}
#[doc = "Field `BUSY` reader - Converter Busy Flag"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Converter Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
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
        *self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
#[doc = "Field `SAMPLE` reader - Sample Phase Flag"]
pub type SAMPLE_R = crate::BitReader<SAMPLE_A>;
#[doc = "Sample Phase Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SAMPLE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SAMPLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SAMPLE_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    pub fn anonc(&self) -> ANONC_R {
        ANONC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    pub fn arbrnd(&self) -> ARBRND_R {
        ARBRND_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    pub fn arbm(&self) -> ARBM_R {
        ARBM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Analog Converter Control Status"]
    #[inline(always)]
    pub fn anons(&self) -> ANONS_R {
        ANONS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - Start-Up Calibration Active Indication"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Converter Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Sample Phase Flag"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Converter Control"]
    #[inline(always)]
    #[must_use]
    pub fn anonc(&mut self) -> ANONC_W<0> {
        ANONC_W::new(self)
    }
    #[doc = "Bits 4:5 - Arbitration Round Length"]
    #[inline(always)]
    #[must_use]
    pub fn arbrnd(&mut self) -> ARBRND_W<4> {
        ARBRND_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn arbm(&mut self) -> ARBM_W<7> {
        ARBM_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARBCFG to value 0"]
impl crate::Resettable for ARBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
