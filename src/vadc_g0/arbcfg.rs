#[doc = "Reader of register ARBCFG"]
pub type R = crate::R<u32, super::ARBCFG>;
#[doc = "Writer for register ARBCFG"]
pub type W = crate::W<u32, super::ARBCFG>;
#[doc = "Register ARBCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ARBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ANONC`"]
pub type ANONC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ANONC`"]
pub struct ANONC_W<'a> {
    w: &'a mut W,
}
impl<'a> ANONC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Arbitration Round Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBRND_A {
    #[doc = "0: 4 arbitration slots per round (tARB = 4 / fADCD)"]
    VALUE1,
    #[doc = "1: 8 arbitration slots per round (tARB = 8 / fADCD)"]
    VALUE2,
    #[doc = "2: 16 arbitration slots per round (tARB = 16 / fADCD)"]
    VALUE3,
    #[doc = "3: 20 arbitration slots per round (tARB = 20 / fADCD)"]
    VALUE4,
}
impl From<ARBRND_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBRND_A) -> Self {
        match variant {
            ARBRND_A::VALUE1 => 0,
            ARBRND_A::VALUE2 => 1,
            ARBRND_A::VALUE3 => 2,
            ARBRND_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `ARBRND`"]
pub type ARBRND_R = crate::R<u8, ARBRND_A>;
impl ARBRND_R {
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
#[doc = "Write proxy for field `ARBRND`"]
pub struct ARBRND_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBRND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBRND_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Arbitration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBM_A {
    #[doc = "0: The arbiter runs permanently. This setting is required for a synchronization slave (see ) and for equidistant sampling using the signal ARBCNT (see )."]
    VALUE1,
    #[doc = "1: The arbiter only runs if at least one conversion request of an enabled request source is pending. This setting ensures a reproducible latency from an incoming request to the conversion start, if the converter is idle. Synchronized conversions are not supported."]
    VALUE2,
}
impl From<ARBM_A> for bool {
    #[inline(always)]
    fn from(variant: ARBM_A) -> Self {
        match variant {
            ARBM_A::VALUE1 => false,
            ARBM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ARBM`"]
pub type ARBM_R = crate::R<bool, ARBM_A>;
impl ARBM_R {
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
        *self == ARBM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBM_A::VALUE2
    }
}
#[doc = "Write proxy for field `ARBM`"]
pub struct ARBM_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Analog Converter Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANONS_A {
    #[doc = "0: Analog converter off"]
    VALUE1,
    #[doc = "3: Normal operation (permanently on)"]
    VALUE4,
}
impl From<ANONS_A> for u8 {
    #[inline(always)]
    fn from(variant: ANONS_A) -> Self {
        match variant {
            ANONS_A::VALUE1 => 0,
            ANONS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `ANONS`"]
pub type ANONS_R = crate::R<u8, ANONS_A>;
impl ANONS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ANONS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ANONS_A::VALUE1),
            3 => Val(ANONS_A::VALUE4),
            i => Res(i),
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
#[doc = "Start-Up Calibration Active Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_A {
    #[doc = "0: Completed or not yet started"]
    VALUE1,
    #[doc = "1: Start-up calibration phase is active"]
    VALUE2,
}
impl From<CAL_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_A) -> Self {
        match variant {
            CAL_A::VALUE1 => false,
            CAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<bool, CAL_A>;
impl CAL_R {
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
        *self == CAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CAL_A::VALUE2
    }
}
#[doc = "Converter Busy Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Not busy"]
    VALUE1,
    #[doc = "1: Converter is busy with a conversion"]
    VALUE2,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::VALUE1 => false,
            BUSY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
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
        *self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
#[doc = "Sample Phase Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLE_A {
    #[doc = "0: Converting or idle"]
    VALUE1,
    #[doc = "1: Input signal is currently sampled"]
    VALUE2,
}
impl From<SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        match variant {
            SAMPLE_A::VALUE1 => false,
            SAMPLE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<bool, SAMPLE_A>;
impl SAMPLE_R {
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
}
