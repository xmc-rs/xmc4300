#[doc = "Reader of register MPU_CTRL"]
pub type R = crate::R<u32, super::MPU_CTRL>;
#[doc = "Writer for register MPU_CTRL"]
pub type W = crate::W<u32, super::MPU_CTRL>;
#[doc = "Register MPU_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable MPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: MPU disabled"]
    VALUE1 = 0,
    #[doc = "1: MPU enabled."]
    VALUE2 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::VALUE1,
            true => ENABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENABLE_A::VALUE2
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MPU disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE1)
    }
    #[doc = "MPU enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENABLE_A::VALUE2)
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
#[doc = "Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFNMIENA_A {
    #[doc = "0: MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    VALUE1 = 0,
    #[doc = "1: the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    VALUE2 = 1,
}
impl From<HFNMIENA_A> for bool {
    #[inline(always)]
    fn from(variant: HFNMIENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFNMIENA`"]
pub type HFNMIENA_R = crate::R<bool, HFNMIENA_A>;
impl HFNMIENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFNMIENA_A {
        match self.bits {
            false => HFNMIENA_A::VALUE1,
            true => HFNMIENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HFNMIENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HFNMIENA_A::VALUE2
    }
}
#[doc = "Write proxy for field `HFNMIENA`"]
pub struct HFNMIENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HFNMIENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFNMIENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MPU is disabled during hard fault, NMI, and FAULTMASK handlers, regardless of the value of the ENABLE bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HFNMIENA_A::VALUE1)
    }
    #[doc = "the MPU is enabled during hard fault, NMI, and FAULTMASK handlers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HFNMIENA_A::VALUE2)
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
#[doc = "Enables privileged software access to the default memory map\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRIVDEFENA_A {
    #[doc = "0: If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    VALUE1 = 0,
    #[doc = "1: If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    VALUE2 = 1,
}
impl From<PRIVDEFENA_A> for bool {
    #[inline(always)]
    fn from(variant: PRIVDEFENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRIVDEFENA`"]
pub type PRIVDEFENA_R = crate::R<bool, PRIVDEFENA_A>;
impl PRIVDEFENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIVDEFENA_A {
        match self.bits {
            false => PRIVDEFENA_A::VALUE1,
            true => PRIVDEFENA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRIVDEFENA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRIVDEFENA_A::VALUE2
    }
}
#[doc = "Write proxy for field `PRIVDEFENA`"]
pub struct PRIVDEFENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVDEFENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRIVDEFENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRIVDEFENA_A::VALUE1)
    }
    #[doc = "If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRIVDEFENA_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable MPU"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Enable the operation of MPU during hard fault, NMI, and FAULTMASK handlers"]
    #[inline(always)]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W {
        HFNMIENA_W { w: self }
    }
    #[doc = "Bit 2 - Enables privileged software access to the default memory map"]
    #[inline(always)]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W {
        PRIVDEFENA_W { w: self }
    }
}
