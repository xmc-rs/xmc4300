#[doc = "Reader of register MMC_IPC_RECEIVE_INTERRUPT_MASK"]
pub type R = crate::R<u32, super::MMC_IPC_RECEIVE_INTERRUPT_MASK>;
#[doc = "Writer for register MMC_IPC_RECEIVE_INTERRUPT_MASK"]
pub type W = crate::W<u32, super::MMC_IPC_RECEIVE_INTERRUPT_MASK>;
#[doc = "Register MMC_IPC_RECEIVE_INTERRUPT_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MMC_IPC_RECEIVE_INTERRUPT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXIPV4GFIM`"]
pub type RXIPV4GFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4GFIM`"]
pub struct RXIPV4GFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4GFIM_W<'a> {
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
#[doc = "Reader of field `RXIPV4HERFIM`"]
pub type RXIPV4HERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4HERFIM`"]
pub struct RXIPV4HERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4HERFIM_W<'a> {
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
#[doc = "Reader of field `RXIPV4NOPAYFIM`"]
pub type RXIPV4NOPAYFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4NOPAYFIM`"]
pub struct RXIPV4NOPAYFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4NOPAYFIM_W<'a> {
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
#[doc = "Reader of field `RXIPV4FRAGFIM`"]
pub type RXIPV4FRAGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4FRAGFIM`"]
pub struct RXIPV4FRAGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4FRAGFIM_W<'a> {
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
#[doc = "Reader of field `RXIPV4UDSBLFIM`"]
pub type RXIPV4UDSBLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4UDSBLFIM`"]
pub struct RXIPV4UDSBLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4UDSBLFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RXIPV6GFIM`"]
pub type RXIPV6GFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV6GFIM`"]
pub struct RXIPV6GFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6GFIM_W<'a> {
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
#[doc = "Reader of field `RXIPV6HERFIM`"]
pub type RXIPV6HERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV6HERFIM`"]
pub struct RXIPV6HERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6HERFIM_W<'a> {
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
#[doc = "Reader of field `RXIPV6NOPAYFIM`"]
pub type RXIPV6NOPAYFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV6NOPAYFIM`"]
pub struct RXIPV6NOPAYFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6NOPAYFIM_W<'a> {
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
#[doc = "Reader of field `RXUDPGFIM`"]
pub type RXUDPGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUDPGFIM`"]
pub struct RXUDPGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPGFIM_W<'a> {
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
#[doc = "Reader of field `RXUDPERFIM`"]
pub type RXUDPERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUDPERFIM`"]
pub struct RXUDPERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPERFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RXTCPGFIM`"]
pub type RXTCPGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTCPGFIM`"]
pub struct RXTCPGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPGFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RXTCPERFIM`"]
pub type RXTCPERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTCPERFIM`"]
pub struct RXTCPERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPERFIM_W<'a> {
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
#[doc = "Reader of field `RXICMPGFIM`"]
pub type RXICMPGFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXICMPGFIM`"]
pub struct RXICMPGFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPGFIM_W<'a> {
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
#[doc = "Reader of field `RXICMPERFIM`"]
pub type RXICMPERFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXICMPERFIM`"]
pub struct RXICMPERFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPERFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RXIPV4GOIM`"]
pub type RXIPV4GOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4GOIM`"]
pub struct RXIPV4GOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4GOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXIPV4HEROIM`"]
pub type RXIPV4HEROIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4HEROIM`"]
pub struct RXIPV4HEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4HEROIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RXIPV4NOPAYOIM`"]
pub type RXIPV4NOPAYOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4NOPAYOIM`"]
pub struct RXIPV4NOPAYOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4NOPAYOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RXIPV4FRAGOIM`"]
pub type RXIPV4FRAGOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4FRAGOIM`"]
pub struct RXIPV4FRAGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4FRAGOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RXIPV4UDSBLOIM`"]
pub type RXIPV4UDSBLOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV4UDSBLOIM`"]
pub struct RXIPV4UDSBLOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV4UDSBLOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RXIPV6GOIM`"]
pub type RXIPV6GOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV6GOIM`"]
pub struct RXIPV6GOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6GOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RXIPV6HEROIM`"]
pub type RXIPV6HEROIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV6HEROIM`"]
pub struct RXIPV6HEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6HEROIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RXIPV6NOPAYOIM`"]
pub type RXIPV6NOPAYOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIPV6NOPAYOIM`"]
pub struct RXIPV6NOPAYOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIPV6NOPAYOIM_W<'a> {
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
#[doc = "Reader of field `RXUDPGOIM`"]
pub type RXUDPGOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUDPGOIM`"]
pub struct RXUDPGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPGOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RXUDPEROIM`"]
pub type RXUDPEROIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUDPEROIM`"]
pub struct RXUDPEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUDPEROIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RXTCPGOIM`"]
pub type RXTCPGOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTCPGOIM`"]
pub struct RXTCPGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPGOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RXTCPEROIM`"]
pub type RXTCPEROIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXTCPEROIM`"]
pub struct RXTCPEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTCPEROIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RXICMPGOIM`"]
pub type RXICMPGOIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXICMPGOIM`"]
pub struct RXICMPGOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPGOIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RXICMPEROIM`"]
pub type RXICMPEROIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXICMPEROIM`"]
pub struct RXICMPEROIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXICMPEROIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4gfim(&self) -> RXIPV4GFIM_R {
        RXIPV4GFIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4herfim(&self) -> RXIPV4HERFIM_R {
        RXIPV4HERFIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayfim(&self) -> RXIPV4NOPAYFIM_R {
        RXIPV4NOPAYFIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragfim(&self) -> RXIPV4FRAGFIM_R {
        RXIPV4FRAGFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsblfim(&self) -> RXIPV4UDSBLFIM_R {
        RXIPV4UDSBLFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6gfim(&self) -> RXIPV6GFIM_R {
        RXIPV6GFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6herfim(&self) -> RXIPV6HERFIM_R {
        RXIPV6HERFIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayfim(&self) -> RXIPV6NOPAYFIM_R {
        RXIPV6NOPAYFIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgfim(&self) -> RXUDPGFIM_R {
        RXUDPGFIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperfim(&self) -> RXUDPERFIM_R {
        RXUDPERFIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgfim(&self) -> RXTCPGFIM_R {
        RXTCPGFIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperfim(&self) -> RXTCPERFIM_R {
        RXTCPERFIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgfim(&self) -> RXICMPGFIM_R {
        RXICMPGFIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperfim(&self) -> RXICMPERFIM_R {
        RXICMPERFIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4goim(&self) -> RXIPV4GOIM_R {
        RXIPV4GOIM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4heroim(&self) -> RXIPV4HEROIM_R {
        RXIPV4HEROIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayoim(&self) -> RXIPV4NOPAYOIM_R {
        RXIPV4NOPAYOIM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragoim(&self) -> RXIPV4FRAGOIM_R {
        RXIPV4FRAGOIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsbloim(&self) -> RXIPV4UDSBLOIM_R {
        RXIPV4UDSBLOIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6goim(&self) -> RXIPV6GOIM_R {
        RXIPV6GOIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6heroim(&self) -> RXIPV6HEROIM_R {
        RXIPV6HEROIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayoim(&self) -> RXIPV6NOPAYOIM_R {
        RXIPV6NOPAYOIM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgoim(&self) -> RXUDPGOIM_R {
        RXUDPGOIM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperoim(&self) -> RXUDPEROIM_R {
        RXUDPEROIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgoim(&self) -> RXTCPGOIM_R {
        RXTCPGOIM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperoim(&self) -> RXTCPEROIM_R {
        RXTCPEROIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgoim(&self) -> RXICMPGOIM_R {
        RXICMPGOIM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperoim(&self) -> RXICMPEROIM_R {
        RXICMPEROIM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4gfim(&mut self) -> RXIPV4GFIM_W {
        RXIPV4GFIM_W { w: self }
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4herfim(&mut self) -> RXIPV4HERFIM_W {
        RXIPV4HERFIM_W { w: self }
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayfim(&mut self) -> RXIPV4NOPAYFIM_W {
        RXIPV4NOPAYFIM_W { w: self }
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragfim(&mut self) -> RXIPV4FRAGFIM_W {
        RXIPV4FRAGFIM_W { w: self }
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsblfim(&mut self) -> RXIPV4UDSBLFIM_W {
        RXIPV4UDSBLFIM_W { w: self }
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6gfim(&mut self) -> RXIPV6GFIM_W {
        RXIPV6GFIM_W { w: self }
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6herfim(&mut self) -> RXIPV6HERFIM_W {
        RXIPV6HERFIM_W { w: self }
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayfim(&mut self) -> RXIPV6NOPAYFIM_W {
        RXIPV6NOPAYFIM_W { w: self }
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgfim(&mut self) -> RXUDPGFIM_W {
        RXUDPGFIM_W { w: self }
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperfim(&mut self) -> RXUDPERFIM_W {
        RXUDPERFIM_W { w: self }
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgfim(&mut self) -> RXTCPGFIM_W {
        RXTCPGFIM_W { w: self }
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperfim(&mut self) -> RXTCPERFIM_W {
        RXTCPERFIM_W { w: self }
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgfim(&mut self) -> RXICMPGFIM_W {
        RXICMPGFIM_W { w: self }
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperfim(&mut self) -> RXICMPERFIM_W {
        RXICMPERFIM_W { w: self }
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4goim(&mut self) -> RXIPV4GOIM_W {
        RXIPV4GOIM_W { w: self }
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4heroim(&mut self) -> RXIPV4HEROIM_W {
        RXIPV4HEROIM_W { w: self }
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4nopayoim(&mut self) -> RXIPV4NOPAYOIM_W {
        RXIPV4NOPAYOIM_W { w: self }
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4fragoim(&mut self) -> RXIPV4FRAGOIM_W {
        RXIPV4FRAGOIM_W { w: self }
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv4udsbloim(&mut self) -> RXIPV4UDSBLOIM_W {
        RXIPV4UDSBLOIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6goim(&mut self) -> RXIPV6GOIM_W {
        RXIPV6GOIM_W { w: self }
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6heroim(&mut self) -> RXIPV6HEROIM_W {
        RXIPV6HEROIM_W { w: self }
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxipv6nopayoim(&mut self) -> RXIPV6NOPAYOIM_W {
        RXIPV6NOPAYOIM_W { w: self }
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudpgoim(&mut self) -> RXUDPGOIM_W {
        RXUDPGOIM_W { w: self }
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxudperoim(&mut self) -> RXUDPEROIM_W {
        RXUDPEROIM_W { w: self }
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcpgoim(&mut self) -> RXTCPGOIM_W {
        RXTCPGOIM_W { w: self }
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxtcperoim(&mut self) -> RXTCPEROIM_W {
        RXTCPEROIM_W { w: self }
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmpgoim(&mut self) -> RXICMPGOIM_W {
        RXICMPGOIM_W { w: self }
    }
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxicmperoim(&mut self) -> RXICMPEROIM_W {
        RXICMPEROIM_W { w: self }
    }
}
