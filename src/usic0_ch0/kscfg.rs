#[doc = "Reader of register KSCFG"]
pub type R = crate::R<u32, super::KSCFG>;
#[doc = "Writer for register KSCFG"]
pub type W = crate::W<u32, super::KSCFG>;
#[doc = "Register KSCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::KSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEN_A {
    #[doc = "0: The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    VALUE1 = 0,
    #[doc = "1: The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    VALUE2 = 1,
}
impl From<MODEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODEN`"]
pub type MODEN_R = crate::R<bool, MODEN_A>;
impl MODEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEN_A {
        match self.bits {
            false => MODEN_A::VALUE1,
            true => MODEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `MODEN`"]
pub struct MODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MODEN_A::VALUE1)
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MODEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Bit Protection for MODEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPMODEN_AW {
    #[doc = "0: MODEN is not changed."]
    VALUE1 = 0,
    #[doc = "1: MODEN is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPMODEN_AW> for bool {
    #[inline(always)]
    fn from(variant: BPMODEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BPMODEN`"]
pub struct BPMODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BPMODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPMODEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MODEN is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPMODEN_AW::VALUE1)
    }
    #[doc = "MODEN is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPMODEN_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Normal Operation Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NOMCFG_A {
    #[doc = "0: Run mode 0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Run mode 1 is selected."]
    VALUE2 = 1,
    #[doc = "2: Stop mode 0 is selected."]
    VALUE3 = 2,
    #[doc = "3: Stop mode 1 is selected."]
    VALUE4 = 3,
}
impl From<NOMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: NOMCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NOMCFG`"]
pub type NOMCFG_R = crate::R<u8, NOMCFG_A>;
impl NOMCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOMCFG_A {
        match self.bits {
            0 => NOMCFG_A::VALUE1,
            1 => NOMCFG_A::VALUE2,
            2 => NOMCFG_A::VALUE3,
            3 => NOMCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NOMCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NOMCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NOMCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NOMCFG_A::VALUE4
    }
}
#[doc = "Write proxy for field `NOMCFG`"]
pub struct NOMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> NOMCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOMCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Run mode 0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE1)
    }
    #[doc = "Run mode 1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE2)
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE3)
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(NOMCFG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Bit Protection for NOMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPNOM_AW {
    #[doc = "0: NOMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: NOMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPNOM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPNOM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BPNOM`"]
pub struct BPNOM_W<'a> {
    w: &'a mut W,
}
impl<'a> BPNOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPNOM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NOMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPNOM_AW::VALUE1)
    }
    #[doc = "NOMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPNOM_AW::VALUE2)
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
#[doc = "Reader of field `SUMCFG`"]
pub type SUMCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUMCFG`"]
pub struct SUMCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUMCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Bit Protection for SUMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BPSUM_AW {
    #[doc = "0: SUMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: SUMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPSUM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPSUM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `BPSUM`"]
pub struct BPSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BPSUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BPSUM_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SUMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BPSUM_AW::VALUE1)
    }
    #[doc = "SUMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BPSUM_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn moden(&self) -> MODEN_R {
        MODEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    pub fn nomcfg(&self) -> NOMCFG_R {
        NOMCFG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn sumcfg(&self) -> SUMCFG_R {
        SUMCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn moden(&mut self) -> MODEN_W {
        MODEN_W { w: self }
    }
    #[doc = "Bit 1 - Bit Protection for MODEN"]
    #[inline(always)]
    pub fn bpmoden(&mut self) -> BPMODEN_W {
        BPMODEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    pub fn nomcfg(&mut self) -> NOMCFG_W {
        NOMCFG_W { w: self }
    }
    #[doc = "Bit 7 - Bit Protection for NOMCFG"]
    #[inline(always)]
    pub fn bpnom(&mut self) -> BPNOM_W {
        BPNOM_W { w: self }
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn sumcfg(&mut self) -> SUMCFG_W {
        SUMCFG_W { w: self }
    }
    #[doc = "Bit 11 - Bit Protection for SUMCFG"]
    #[inline(always)]
    pub fn bpsum(&mut self) -> BPSUM_W {
        BPSUM_W { w: self }
    }
}
