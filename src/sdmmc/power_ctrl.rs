#[doc = "Reader of register POWER_CTRL"]
pub type R = crate::R<u8, super::POWER_CTRL>;
#[doc = "Writer for register POWER_CTRL"]
pub type W = crate::W<u8, super::POWER_CTRL>;
#[doc = "Register POWER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::POWER_CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HARDWARE_RESET`"]
pub type HARDWARE_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HARDWARE_RESET`"]
pub struct HARDWARE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> HARDWARE_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "SD Bus Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SD_BUS_VOLTAGE_SEL_A {
    #[doc = "7: 3.3V (Flattop.)"]
    VALUE1 = 7,
}
impl From<SD_BUS_VOLTAGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SD_BUS_VOLTAGE_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SD_BUS_VOLTAGE_SEL`"]
pub type SD_BUS_VOLTAGE_SEL_R = crate::R<u8, SD_BUS_VOLTAGE_SEL_A>;
impl SD_BUS_VOLTAGE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SD_BUS_VOLTAGE_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(SD_BUS_VOLTAGE_SEL_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SD_BUS_VOLTAGE_SEL_A::VALUE1
    }
}
#[doc = "Write proxy for field `SD_BUS_VOLTAGE_SEL`"]
pub struct SD_BUS_VOLTAGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_BUS_VOLTAGE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_BUS_VOLTAGE_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "3.3V (Flattop.)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SD_BUS_VOLTAGE_SEL_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u8) & 0x07) << 1);
        self.w
    }
}
#[doc = "SD Bus Power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SD_BUS_POWER_A {
    #[doc = "0: Power off"]
    VALUE1 = 0,
    #[doc = "1: Power on"]
    VALUE2 = 1,
}
impl From<SD_BUS_POWER_A> for bool {
    #[inline(always)]
    fn from(variant: SD_BUS_POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SD_BUS_POWER`"]
pub type SD_BUS_POWER_R = crate::R<bool, SD_BUS_POWER_A>;
impl SD_BUS_POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SD_BUS_POWER_A {
        match self.bits {
            false => SD_BUS_POWER_A::VALUE1,
            true => SD_BUS_POWER_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SD_BUS_POWER_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SD_BUS_POWER_A::VALUE2
    }
}
#[doc = "Write proxy for field `SD_BUS_POWER`"]
pub struct SD_BUS_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_BUS_POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SD_BUS_POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SD_BUS_POWER_A::VALUE1)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SD_BUS_POWER_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Hardware reset"]
    #[inline(always)]
    pub fn hardware_reset(&self) -> HARDWARE_RESET_R {
        HARDWARE_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sd_bus_voltage_sel(&self) -> SD_BUS_VOLTAGE_SEL_R {
        SD_BUS_VOLTAGE_SEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sd_bus_power(&self) -> SD_BUS_POWER_R {
        SD_BUS_POWER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Hardware reset"]
    #[inline(always)]
    pub fn hardware_reset(&mut self) -> HARDWARE_RESET_W {
        HARDWARE_RESET_W { w: self }
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select"]
    #[inline(always)]
    pub fn sd_bus_voltage_sel(&mut self) -> SD_BUS_VOLTAGE_SEL_W {
        SD_BUS_VOLTAGE_SEL_W { w: self }
    }
    #[doc = "Bit 0 - SD Bus Power"]
    #[inline(always)]
    pub fn sd_bus_power(&mut self) -> SD_BUS_POWER_W {
        SD_BUS_POWER_W { w: self }
    }
}
