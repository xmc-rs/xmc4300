#[doc = "Reader of register DCFG"]
pub type R = crate::R<u32, super::DCFG>;
#[doc = "Writer for register DCFG"]
pub type W = crate::W<u32, super::DCFG>;
#[doc = "Register DCFG `reset()`'s with value 0x0820_0000"]
impl crate::ResetValue for super::DCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0820_0000
    }
}
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEVSPD_A {
    #[doc = "3: Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    VALUE4 = 3,
}
impl From<DEVSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DevSpd`"]
pub type DEVSPD_R = crate::R<u8, DEVSPD_A>;
impl DEVSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEVSPD_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(DEVSPD_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DEVSPD_A::VALUE4
    }
}
#[doc = "Write proxy for field `DevSpd`"]
pub struct DEVSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVSPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Full speed (USB 1.1 transceiver clock is 48 MHz)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(DEVSPD_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Non-Zero-Length Status OUT Handshake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NZSTSOUTHSHK_A {
    #[doc = "1: Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    VALUE1 = 1,
    #[doc = "0: Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    VALUE2 = 0,
}
impl From<NZSTSOUTHSHK_A> for bool {
    #[inline(always)]
    fn from(variant: NZSTSOUTHSHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NZStsOUTHShk`"]
pub type NZSTSOUTHSHK_R = crate::R<bool, NZSTSOUTHSHK_A>;
impl NZSTSOUTHSHK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NZSTSOUTHSHK_A {
        match self.bits {
            true => NZSTSOUTHSHK_A::VALUE1,
            false => NZSTSOUTHSHK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NZSTSOUTHSHK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NZSTSOUTHSHK_A::VALUE2
    }
}
#[doc = "Write proxy for field `NZStsOUTHShk`"]
pub struct NZSTSOUTHSHK_W<'a> {
    w: &'a mut W,
}
impl<'a> NZSTSOUTHSHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NZSTSOUTHSHK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Send a STALL handshake on a nonzero-length status OUT transaction and do not send the received OUT packet to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NZSTSOUTHSHK_A::VALUE1)
    }
    #[doc = "Send the received OUT packet to the application (zero-length or nonzero-length) and send a handshake based on the NAK and STALL bits for the endpoint in the Device Endpoint Control register."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NZSTSOUTHSHK_A::VALUE2)
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
#[doc = "Reader of field `DevAddr`"]
pub type DEVADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DevAddr`"]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERFRINT_A {
    #[doc = "0: 80% of the frame interval"]
    VALUE1 = 0,
    #[doc = "1: 85%"]
    VALUE2 = 1,
    #[doc = "2: 90%"]
    VALUE3 = 2,
    #[doc = "3: 95%"]
    VALUE4 = 3,
}
impl From<PERFRINT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFRINT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PerFrInt`"]
pub type PERFRINT_R = crate::R<u8, PERFRINT_A>;
impl PERFRINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERFRINT_A {
        match self.bits {
            0 => PERFRINT_A::VALUE1,
            1 => PERFRINT_A::VALUE2,
            2 => PERFRINT_A::VALUE3,
            3 => PERFRINT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PERFRINT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PERFRINT_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PERFRINT_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PERFRINT_A::VALUE4
    }
}
#[doc = "Write proxy for field `PerFrInt`"]
pub struct PERFRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERFRINT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "80% of the frame interval"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE1)
    }
    #[doc = "85%"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE2)
    }
    #[doc = "90%"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE3)
    }
    #[doc = "95%"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PERFRINT_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `DescDMA`"]
pub type DESCDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DescDMA`"]
pub struct DESCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Periodic Scheduling Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERSCHINTVL_A {
    #[doc = "0: 25% of frame."]
    VALUE1 = 0,
    #[doc = "1: 50% of frame."]
    VALUE2 = 1,
    #[doc = "2: 75% of frame."]
    VALUE3 = 2,
}
impl From<PERSCHINTVL_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSCHINTVL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PerSchIntvl`"]
pub type PERSCHINTVL_R = crate::R<u8, PERSCHINTVL_A>;
impl PERSCHINTVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERSCHINTVL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERSCHINTVL_A::VALUE1),
            1 => Val(PERSCHINTVL_A::VALUE2),
            2 => Val(PERSCHINTVL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PERSCHINTVL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PERSCHINTVL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PERSCHINTVL_A::VALUE3
    }
}
#[doc = "Write proxy for field `PerSchIntvl`"]
pub struct PERSCHINTVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSCHINTVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERSCHINTVL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "25% of frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PERSCHINTVL_A::VALUE1)
    }
    #[doc = "50% of frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PERSCHINTVL_A::VALUE2)
    }
    #[doc = "75% of frame."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PERSCHINTVL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&self) -> PERSCHINTVL_R {
        PERSCHINTVL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn dev_spd(&mut self) -> DEVSPD_W {
        DEVSPD_W { w: self }
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzsts_outhshk(&mut self) -> NZSTSOUTHSHK_W {
        NZSTSOUTHSHK_W { w: self }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn per_fr_int(&mut self) -> PERFRINT_W {
        PERFRINT_W { w: self }
    }
    #[doc = "Bit 23 - Enable Scatter/Gather DMA in Device mode."]
    #[inline(always)]
    pub fn desc_dma(&mut self) -> DESCDMA_W {
        DESCDMA_W { w: self }
    }
    #[doc = "Bits 24:25 - Periodic Scheduling Interval"]
    #[inline(always)]
    pub fn per_sch_intvl(&mut self) -> PERSCHINTVL_W {
        PERSCHINTVL_W { w: self }
    }
}
