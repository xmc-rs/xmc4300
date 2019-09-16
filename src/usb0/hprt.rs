#[doc = "Reader of register HPRT"]
pub type R = crate::R<u32, super::HPRT>;
#[doc = "Writer for register HPRT"]
pub type W = crate::W<u32, super::HPRT>;
#[doc = "Register HPRT `reset()`'s with value 0"]
impl crate::ResetValue for super::HPRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTCONNSTS_A {
    #[doc = "0: No device is attached to the port."]
    VALUE1,
    #[doc = "1: A device is attached to the port."]
    VALUE2,
}
impl From<PRTCONNSTS_A> for bool {
    #[inline(always)]
    fn from(variant: PRTCONNSTS_A) -> Self {
        match variant {
            PRTCONNSTS_A::VALUE1 => false,
            PRTCONNSTS_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PrtConnSts`"]
pub type PRTCONNSTS_R = crate::R<bool, PRTCONNSTS_A>;
impl PRTCONNSTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTCONNSTS_A {
        match self.bits {
            false => PRTCONNSTS_A::VALUE1,
            true => PRTCONNSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTCONNSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRTCONNSTS_A::VALUE2
    }
}
#[doc = "Reader of field `PrtConnDet`"]
pub type PRTCONNDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PrtConnDet`"]
pub struct PRTCONNDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCONNDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Port Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTENA_A {
    #[doc = "0: Port disabled"]
    VALUE1,
    #[doc = "1: Port enabled"]
    VALUE2,
}
impl From<PRTENA_A> for bool {
    #[inline(always)]
    fn from(variant: PRTENA_A) -> Self {
        match variant {
            PRTENA_A::VALUE1 => false,
            PRTENA_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PrtEna`"]
pub type PRTENA_R = crate::R<bool, PRTENA_A>;
impl PRTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTENA_A {
        match self.bits {
            false => PRTENA_A::VALUE1,
            true => PRTENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRTENA_A::VALUE2
    }
}
#[doc = "Write proxy for field `PrtEna`"]
pub struct PRTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTENA_A::VALUE1)
    }
    #[doc = "Port enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTENA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PrtEnChng`"]
pub type PRTENCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PrtEnChng`"]
pub struct PRTENCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTENCHNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Port Overcurrent Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTOVRCURRACT_A {
    #[doc = "0: No overcurrent condition"]
    VALUE1,
    #[doc = "1: Overcurrent condition"]
    VALUE2,
}
impl From<PRTOVRCURRACT_A> for bool {
    #[inline(always)]
    fn from(variant: PRTOVRCURRACT_A) -> Self {
        match variant {
            PRTOVRCURRACT_A::VALUE1 => false,
            PRTOVRCURRACT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PrtOvrCurrAct`"]
pub type PRTOVRCURRACT_R = crate::R<bool, PRTOVRCURRACT_A>;
impl PRTOVRCURRACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTOVRCURRACT_A {
        match self.bits {
            false => PRTOVRCURRACT_A::VALUE1,
            true => PRTOVRCURRACT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTOVRCURRACT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRTOVRCURRACT_A::VALUE2
    }
}
#[doc = "Reader of field `PrtOvrCurrChng`"]
pub type PRTOVRCURRCHNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PrtOvrCurrChng`"]
pub struct PRTOVRCURRCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTOVRCURRCHNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Port Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTRES_A {
    #[doc = "0: No resume driven"]
    VALUE1,
    #[doc = "1: Resume driven"]
    VALUE2,
}
impl From<PRTRES_A> for bool {
    #[inline(always)]
    fn from(variant: PRTRES_A) -> Self {
        match variant {
            PRTRES_A::VALUE1 => false,
            PRTRES_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PrtRes`"]
pub type PRTRES_R = crate::R<bool, PRTRES_A>;
impl PRTRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTRES_A {
        match self.bits {
            false => PRTRES_A::VALUE1,
            true => PRTRES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTRES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRTRES_A::VALUE2
    }
}
#[doc = "Write proxy for field `PrtRes`"]
pub struct PRTRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTRES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No resume driven"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTRES_A::VALUE1)
    }
    #[doc = "Resume driven"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTRES_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Port Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTSUSP_A {
    #[doc = "0: Port not in Suspend mode"]
    VALUE1,
    #[doc = "1: Port in Suspend mode"]
    VALUE2,
}
impl From<PRTSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: PRTSUSP_A) -> Self {
        match variant {
            PRTSUSP_A::VALUE1 => false,
            PRTSUSP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PrtSusp`"]
pub type PRTSUSP_R = crate::R<bool, PRTSUSP_A>;
impl PRTSUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTSUSP_A {
        match self.bits {
            false => PRTSUSP_A::VALUE1,
            true => PRTSUSP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTSUSP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRTSUSP_A::VALUE2
    }
}
#[doc = "Write proxy for field `PrtSusp`"]
pub struct PRTSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTSUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port not in Suspend mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTSUSP_A::VALUE1)
    }
    #[doc = "Port in Suspend mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTSUSP_A::VALUE2)
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
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTRST_A {
    #[doc = "0: Port not in reset"]
    VALUE1,
    #[doc = "1: Port in reset"]
    VALUE2,
}
impl From<PRTRST_A> for bool {
    #[inline(always)]
    fn from(variant: PRTRST_A) -> Self {
        match variant {
            PRTRST_A::VALUE1 => false,
            PRTRST_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PrtRst`"]
pub type PRTRST_R = crate::R<bool, PRTRST_A>;
impl PRTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTRST_A {
        match self.bits {
            false => PRTRST_A::VALUE1,
            true => PRTRST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTRST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRTRST_A::VALUE2
    }
}
#[doc = "Write proxy for field `PrtRst`"]
pub struct PRTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Port not in reset"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTRST_A::VALUE1)
    }
    #[doc = "Port in reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTRST_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PrtLnSts`"]
pub type PRTLNSTS_R = crate::R<u8, u8>;
#[doc = "Port Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTPWR_A {
    #[doc = "0: Power off"]
    VALUE1,
    #[doc = "1: Power on"]
    VALUE2,
}
impl From<PRTPWR_A> for bool {
    #[inline(always)]
    fn from(variant: PRTPWR_A) -> Self {
        match variant {
            PRTPWR_A::VALUE1 => false,
            PRTPWR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PrtPwr`"]
pub type PRTPWR_R = crate::R<bool, PRTPWR_A>;
impl PRTPWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRTPWR_A {
        match self.bits {
            false => PRTPWR_A::VALUE1,
            true => PRTPWR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTPWR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRTPWR_A::VALUE2
    }
}
#[doc = "Write proxy for field `PrtPwr`"]
pub struct PRTPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTPWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRTPWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRTPWR_A::VALUE1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRTPWR_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Port Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRTSPD_A {
    #[doc = "1: Full speed"]
    VALUE1,
}
impl From<PRTSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTSPD_A) -> Self {
        match variant {
            PRTSPD_A::VALUE1 => 1,
        }
    }
}
#[doc = "Reader of field `PrtSpd`"]
pub type PRTSPD_R = crate::R<u8, PRTSPD_A>;
impl PRTSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRTSPD_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(PRTSPD_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRTSPD_A::VALUE1
    }
}
impl R {
    #[doc = "Bit 0 - Port Connect Status"]
    #[inline(always)]
    pub fn prt_conn_sts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prt_conn_det(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prt_ena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prt_en_chng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Overcurrent Active"]
    #[inline(always)]
    pub fn prt_ovr_curr_act(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prt_ovr_curr_chng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prt_res(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prt_susp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prt_rst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Port Line Status"]
    #[inline(always)]
    pub fn prt_ln_sts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prt_pwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Port Speed"]
    #[inline(always)]
    pub fn prt_spd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port Connect Detected"]
    #[inline(always)]
    pub fn prt_conn_det(&mut self) -> PRTCONNDET_W {
        PRTCONNDET_W { w: self }
    }
    #[doc = "Bit 2 - Port Enable"]
    #[inline(always)]
    pub fn prt_ena(&mut self) -> PRTENA_W {
        PRTENA_W { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change"]
    #[inline(always)]
    pub fn prt_en_chng(&mut self) -> PRTENCHNG_W {
        PRTENCHNG_W { w: self }
    }
    #[doc = "Bit 5 - Port Overcurrent Change"]
    #[inline(always)]
    pub fn prt_ovr_curr_chng(&mut self) -> PRTOVRCURRCHNG_W {
        PRTOVRCURRCHNG_W { w: self }
    }
    #[doc = "Bit 6 - Port Resume"]
    #[inline(always)]
    pub fn prt_res(&mut self) -> PRTRES_W {
        PRTRES_W { w: self }
    }
    #[doc = "Bit 7 - Port Suspend"]
    #[inline(always)]
    pub fn prt_susp(&mut self) -> PRTSUSP_W {
        PRTSUSP_W { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn prt_rst(&mut self) -> PRTRST_W {
        PRTRST_W { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline(always)]
    pub fn prt_pwr(&mut self) -> PRTPWR_W {
        PRTPWR_W { w: self }
    }
}
