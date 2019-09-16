#[doc = "Reader of register BLOCK_GAP_CTRL"]
pub type R = crate::R<u8, super::BLOCK_GAP_CTRL>;
#[doc = "Writer for register BLOCK_GAP_CTRL"]
pub type W = crate::W<u8, super::BLOCK_GAP_CTRL>;
#[doc = "Register BLOCK_GAP_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BLOCK_GAP_CTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INT_AT_BLOCK_GAP`"]
pub type INT_AT_BLOCK_GAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_AT_BLOCK_GAP`"]
pub struct INT_AT_BLOCK_GAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_AT_BLOCK_GAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Read Wait Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WAIT_CTRL_A {
    #[doc = "0: Disable Read Wait Control"]
    VALUE1,
    #[doc = "1: Enable Read Wait Control"]
    VALUE2,
}
impl From<READ_WAIT_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: READ_WAIT_CTRL_A) -> Self {
        match variant {
            READ_WAIT_CTRL_A::VALUE1 => false,
            READ_WAIT_CTRL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `READ_WAIT_CTRL`"]
pub type READ_WAIT_CTRL_R = crate::R<bool, READ_WAIT_CTRL_A>;
impl READ_WAIT_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_WAIT_CTRL_A {
        match self.bits {
            false => READ_WAIT_CTRL_A::VALUE1,
            true => READ_WAIT_CTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == READ_WAIT_CTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == READ_WAIT_CTRL_A::VALUE2
    }
}
#[doc = "Write proxy for field `READ_WAIT_CTRL`"]
pub struct READ_WAIT_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WAIT_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_WAIT_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable Read Wait Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(READ_WAIT_CTRL_A::VALUE1)
    }
    #[doc = "Enable Read Wait Control"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(READ_WAIT_CTRL_A::VALUE2)
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
#[doc = "Continue Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTINUE_REQ_A {
    #[doc = "0: Ignored"]
    VALUE1,
    #[doc = "1: Restart"]
    VALUE2,
}
impl From<CONTINUE_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: CONTINUE_REQ_A) -> Self {
        match variant {
            CONTINUE_REQ_A::VALUE1 => false,
            CONTINUE_REQ_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CONTINUE_REQ`"]
pub type CONTINUE_REQ_R = crate::R<bool, CONTINUE_REQ_A>;
impl CONTINUE_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONTINUE_REQ_A {
        match self.bits {
            false => CONTINUE_REQ_A::VALUE1,
            true => CONTINUE_REQ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CONTINUE_REQ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CONTINUE_REQ_A::VALUE2
    }
}
#[doc = "Write proxy for field `CONTINUE_REQ`"]
pub struct CONTINUE_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTINUE_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONTINUE_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CONTINUE_REQ_A::VALUE1)
    }
    #[doc = "Restart"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CONTINUE_REQ_A::VALUE2)
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
#[doc = "Stop At Block Gap Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AT_BLOCK_GAP_A {
    #[doc = "0: Transfer"]
    VALUE1,
    #[doc = "1: Stop"]
    VALUE2,
}
impl From<STOP_AT_BLOCK_GAP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_AT_BLOCK_GAP_A) -> Self {
        match variant {
            STOP_AT_BLOCK_GAP_A::VALUE1 => false,
            STOP_AT_BLOCK_GAP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `STOP_AT_BLOCK_GAP`"]
pub type STOP_AT_BLOCK_GAP_R = crate::R<bool, STOP_AT_BLOCK_GAP_A>;
impl STOP_AT_BLOCK_GAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_AT_BLOCK_GAP_A {
        match self.bits {
            false => STOP_AT_BLOCK_GAP_A::VALUE1,
            true => STOP_AT_BLOCK_GAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STOP_AT_BLOCK_GAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STOP_AT_BLOCK_GAP_A::VALUE2
    }
}
#[doc = "Write proxy for field `STOP_AT_BLOCK_GAP`"]
pub struct STOP_AT_BLOCK_GAP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_AT_BLOCK_GAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_AT_BLOCK_GAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STOP_AT_BLOCK_GAP_A::VALUE1)
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STOP_AT_BLOCK_GAP_A::VALUE2)
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
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn int_at_block_gap(&self) -> INT_AT_BLOCK_GAP_R {
        INT_AT_BLOCK_GAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn read_wait_ctrl(&self) -> READ_WAIT_CTRL_R {
        READ_WAIT_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn continue_req(&self) -> CONTINUE_REQ_R {
        CONTINUE_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn stop_at_block_gap(&self) -> STOP_AT_BLOCK_GAP_R {
        STOP_AT_BLOCK_GAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Interrupt At Block Gap"]
    #[inline(always)]
    pub fn int_at_block_gap(&mut self) -> INT_AT_BLOCK_GAP_W {
        INT_AT_BLOCK_GAP_W { w: self }
    }
    #[doc = "Bit 2 - Read Wait Control"]
    #[inline(always)]
    pub fn read_wait_ctrl(&mut self) -> READ_WAIT_CTRL_W {
        READ_WAIT_CTRL_W { w: self }
    }
    #[doc = "Bit 1 - Continue Request"]
    #[inline(always)]
    pub fn continue_req(&mut self) -> CONTINUE_REQ_W {
        CONTINUE_REQ_W { w: self }
    }
    #[doc = "Bit 0 - Stop At Block Gap Request"]
    #[inline(always)]
    pub fn stop_at_block_gap(&mut self) -> STOP_AT_BLOCK_GAP_W {
        STOP_AT_BLOCK_GAP_W { w: self }
    }
}
