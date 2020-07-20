#[doc = "Reader of register SW_RESET"]
pub type R = crate::R<u8, super::SW_RESET>;
#[doc = "Writer for register SW_RESET"]
pub type W = crate::W<u8, super::SW_RESET>;
#[doc = "Register SW_RESET `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_RESET {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software Reset for DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_DAT_LINE_A {
    #[doc = "0: Work"]
    VALUE1 = 0,
    #[doc = "1: Reset"]
    VALUE2 = 1,
}
impl From<SW_RST_DAT_LINE_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_DAT_LINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SW_RST_DAT_LINE`"]
pub type SW_RST_DAT_LINE_R = crate::R<bool, SW_RST_DAT_LINE_A>;
impl SW_RST_DAT_LINE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_RST_DAT_LINE_A {
        match self.bits {
            false => SW_RST_DAT_LINE_A::VALUE1,
            true => SW_RST_DAT_LINE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW_RST_DAT_LINE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW_RST_DAT_LINE_A::VALUE2
    }
}
#[doc = "Write proxy for field `SW_RST_DAT_LINE`"]
pub struct SW_RST_DAT_LINE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_DAT_LINE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_RST_DAT_LINE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SW_RST_DAT_LINE_A::VALUE1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SW_RST_DAT_LINE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Software Reset for CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RST_CMD_LINE_A {
    #[doc = "0: Work"]
    VALUE1 = 0,
    #[doc = "1: Reset"]
    VALUE2 = 1,
}
impl From<SW_RST_CMD_LINE_A> for bool {
    #[inline(always)]
    fn from(variant: SW_RST_CMD_LINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SW_RST_CMD_LINE`"]
pub type SW_RST_CMD_LINE_R = crate::R<bool, SW_RST_CMD_LINE_A>;
impl SW_RST_CMD_LINE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_RST_CMD_LINE_A {
        match self.bits {
            false => SW_RST_CMD_LINE_A::VALUE1,
            true => SW_RST_CMD_LINE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW_RST_CMD_LINE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW_RST_CMD_LINE_A::VALUE2
    }
}
#[doc = "Write proxy for field `SW_RST_CMD_LINE`"]
pub struct SW_RST_CMD_LINE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_CMD_LINE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_RST_CMD_LINE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Work"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SW_RST_CMD_LINE_A::VALUE1)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SW_RST_CMD_LINE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SW_RST_ALL`"]
pub type SW_RST_ALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_RST_ALL`"]
pub struct SW_RST_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_ALL_W<'a> {
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
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sw_rst_dat_line(&self) -> SW_RST_DAT_LINE_R {
        SW_RST_DAT_LINE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sw_rst_cmd_line(&self) -> SW_RST_CMD_LINE_R {
        SW_RST_CMD_LINE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Software Reset for DAT Line"]
    #[inline(always)]
    pub fn sw_rst_dat_line(&mut self) -> SW_RST_DAT_LINE_W {
        SW_RST_DAT_LINE_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset for CMD Line"]
    #[inline(always)]
    pub fn sw_rst_cmd_line(&mut self) -> SW_RST_CMD_LINE_W {
        SW_RST_CMD_LINE_W { w: self }
    }
    #[doc = "Bit 0 - Software Reset for All"]
    #[inline(always)]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W {
        SW_RST_ALL_W { w: self }
    }
}
