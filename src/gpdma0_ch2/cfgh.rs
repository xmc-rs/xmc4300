#[doc = "Reader of register CFGH"]
pub type R = crate::R<u32, super::CFGH>;
#[doc = "Writer for register CFGH"]
pub type W = crate::W<u32, super::CFGH>;
#[doc = "Register CFGH `reset()`'s with value 0x04"]
impl crate::ResetValue for super::CFGH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `DEST_PER`"]
pub type DEST_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEST_PER`"]
pub struct DEST_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `SRC_PER`"]
pub type SRC_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRC_PER`"]
pub struct SRC_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `PROTCTL`"]
pub type PROTCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROTCTL`"]
pub struct PROTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "FIFO Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_MODE_A {
    #[doc = "0: Space/data available for single AHB transfer of the specified transfer width."]
    VALUE1 = 0,
    #[doc = "1: Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    VALUE2 = 1,
}
impl From<FIFO_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_MODE`"]
pub type FIFO_MODE_R = crate::R<bool, FIFO_MODE_A>;
impl FIFO_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_MODE_A {
        match self.bits {
            false => FIFO_MODE_A::VALUE1,
            true => FIFO_MODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_MODE_A::VALUE2
    }
}
#[doc = "Write proxy for field `FIFO_MODE`"]
pub struct FIFO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FIFO_MODE_A::VALUE1)
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FIFO_MODE_A::VALUE2)
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
#[doc = "Flow Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCMODE_A {
    #[doc = "0: Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    VALUE1 = 0,
    #[doc = "1: Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    VALUE2 = 1,
}
impl From<FCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: FCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCMODE`"]
pub type FCMODE_R = crate::R<bool, FCMODE_A>;
impl FCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCMODE_A {
        match self.bits {
            false => FCMODE_A::VALUE1,
            true => FCMODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCMODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCMODE_A::VALUE2
    }
}
#[doc = "Write proxy for field `FCMODE`"]
pub struct FCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FCMODE_A::VALUE1)
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FCMODE_A::VALUE2)
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
impl R {
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&self) -> DEST_PER_R {
        DEST_PER_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&self) -> SRC_PER_R {
        SRC_PER_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&self) -> PROTCTL_R {
        PROTCTL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&self) -> FCMODE_R {
        FCMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&mut self) -> DEST_PER_W {
        DEST_PER_W { w: self }
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&mut self) -> SRC_PER_W {
        SRC_PER_W { w: self }
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&mut self) -> PROTCTL_W {
        PROTCTL_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&mut self) -> FIFO_MODE_W {
        FIFO_MODE_W { w: self }
    }
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&mut self) -> FCMODE_W {
        FCMODE_W { w: self }
    }
}
