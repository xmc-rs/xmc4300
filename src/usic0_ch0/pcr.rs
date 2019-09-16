#[doc = "Reader of register PCR"]
pub type R = crate::R<u32, super::PCR>;
#[doc = "Writer for register PCR"]
pub type W = crate::W<u32, super::PCR>;
#[doc = "Register PCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTR0`"]
pub type CTR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR0`"]
pub struct CTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR0_W<'a> {
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
#[doc = "Reader of field `CTR1`"]
pub type CTR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR1`"]
pub struct CTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR1_W<'a> {
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
#[doc = "Reader of field `CTR2`"]
pub type CTR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR2`"]
pub struct CTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR2_W<'a> {
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
#[doc = "Reader of field `CTR3`"]
pub type CTR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR3`"]
pub struct CTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR3_W<'a> {
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
#[doc = "Reader of field `CTR4`"]
pub type CTR4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR4`"]
pub struct CTR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR4_W<'a> {
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
#[doc = "Reader of field `CTR5`"]
pub type CTR5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR5`"]
pub struct CTR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR5_W<'a> {
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
#[doc = "Reader of field `CTR6`"]
pub type CTR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR6`"]
pub struct CTR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR6_W<'a> {
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
#[doc = "Reader of field `CTR7`"]
pub type CTR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR7`"]
pub struct CTR7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR7_W<'a> {
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
#[doc = "Reader of field `CTR8`"]
pub type CTR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR8`"]
pub struct CTR8_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR8_W<'a> {
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
#[doc = "Reader of field `CTR9`"]
pub type CTR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR9`"]
pub struct CTR9_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR9_W<'a> {
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
#[doc = "Reader of field `CTR10`"]
pub type CTR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR10`"]
pub struct CTR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR10_W<'a> {
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
#[doc = "Reader of field `CTR11`"]
pub type CTR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR11`"]
pub struct CTR11_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR11_W<'a> {
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
#[doc = "Reader of field `CTR12`"]
pub type CTR12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR12`"]
pub struct CTR12_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR12_W<'a> {
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
#[doc = "Reader of field `CTR13`"]
pub type CTR13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR13`"]
pub struct CTR13_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR13_W<'a> {
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
#[doc = "Reader of field `CTR14`"]
pub type CTR14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR14`"]
pub struct CTR14_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CTR15`"]
pub type CTR15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR15`"]
pub struct CTR15_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CTR16`"]
pub type CTR16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR16`"]
pub struct CTR16_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR16_W<'a> {
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
#[doc = "Reader of field `CTR17`"]
pub type CTR17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR17`"]
pub struct CTR17_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR17_W<'a> {
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
#[doc = "Reader of field `CTR18`"]
pub type CTR18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR18`"]
pub struct CTR18_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR18_W<'a> {
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
#[doc = "Reader of field `CTR19`"]
pub type CTR19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR19`"]
pub struct CTR19_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR19_W<'a> {
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
#[doc = "Reader of field `CTR20`"]
pub type CTR20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR20`"]
pub struct CTR20_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR20_W<'a> {
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
#[doc = "Reader of field `CTR21`"]
pub type CTR21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR21`"]
pub struct CTR21_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR21_W<'a> {
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
#[doc = "Reader of field `CTR22`"]
pub type CTR22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR22`"]
pub struct CTR22_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR22_W<'a> {
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
#[doc = "Reader of field `CTR23`"]
pub type CTR23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR23`"]
pub struct CTR23_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR23_W<'a> {
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
#[doc = "Reader of field `CTR24`"]
pub type CTR24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR24`"]
pub struct CTR24_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR24_W<'a> {
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
#[doc = "Reader of field `CTR25`"]
pub type CTR25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR25`"]
pub struct CTR25_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR25_W<'a> {
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
#[doc = "Reader of field `CTR26`"]
pub type CTR26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR26`"]
pub struct CTR26_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR26_W<'a> {
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
#[doc = "Reader of field `CTR27`"]
pub type CTR27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR27`"]
pub struct CTR27_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR27_W<'a> {
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
#[doc = "Reader of field `CTR28`"]
pub type CTR28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR28`"]
pub struct CTR28_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR28_W<'a> {
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
#[doc = "Reader of field `CTR29`"]
pub type CTR29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR29`"]
pub struct CTR29_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR29_W<'a> {
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
#[doc = "Reader of field `CTR30`"]
pub type CTR30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR30`"]
pub struct CTR30_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `CTR31`"]
pub type CTR31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR31`"]
pub struct CTR31_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Protocol Control Bit 0"]
    #[inline(always)]
    pub fn ctr0(&self) -> CTR0_R {
        CTR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protocol Control Bit 1"]
    #[inline(always)]
    pub fn ctr1(&self) -> CTR1_R {
        CTR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol Control Bit 2"]
    #[inline(always)]
    pub fn ctr2(&self) -> CTR2_R {
        CTR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Protocol Control Bit 3"]
    #[inline(always)]
    pub fn ctr3(&self) -> CTR3_R {
        CTR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protocol Control Bit 4"]
    #[inline(always)]
    pub fn ctr4(&self) -> CTR4_R {
        CTR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Protocol Control Bit 5"]
    #[inline(always)]
    pub fn ctr5(&self) -> CTR5_R {
        CTR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Protocol Control Bit 6"]
    #[inline(always)]
    pub fn ctr6(&self) -> CTR6_R {
        CTR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Protocol Control Bit 7"]
    #[inline(always)]
    pub fn ctr7(&self) -> CTR7_R {
        CTR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protocol Control Bit 8"]
    #[inline(always)]
    pub fn ctr8(&self) -> CTR8_R {
        CTR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protocol Control Bit 9"]
    #[inline(always)]
    pub fn ctr9(&self) -> CTR9_R {
        CTR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Protocol Control Bit 10"]
    #[inline(always)]
    pub fn ctr10(&self) -> CTR10_R {
        CTR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protocol Control Bit 11"]
    #[inline(always)]
    pub fn ctr11(&self) -> CTR11_R {
        CTR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protocol Control Bit 12"]
    #[inline(always)]
    pub fn ctr12(&self) -> CTR12_R {
        CTR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Protocol Control Bit 13"]
    #[inline(always)]
    pub fn ctr13(&self) -> CTR13_R {
        CTR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol Control Bit 14"]
    #[inline(always)]
    pub fn ctr14(&self) -> CTR14_R {
        CTR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Protocol Control Bit 15"]
    #[inline(always)]
    pub fn ctr15(&self) -> CTR15_R {
        CTR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protocol Control Bit 16"]
    #[inline(always)]
    pub fn ctr16(&self) -> CTR16_R {
        CTR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Protocol Control Bit 17"]
    #[inline(always)]
    pub fn ctr17(&self) -> CTR17_R {
        CTR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Protocol Control Bit 18"]
    #[inline(always)]
    pub fn ctr18(&self) -> CTR18_R {
        CTR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Protocol Control Bit 19"]
    #[inline(always)]
    pub fn ctr19(&self) -> CTR19_R {
        CTR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Protocol Control Bit 20"]
    #[inline(always)]
    pub fn ctr20(&self) -> CTR20_R {
        CTR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Protocol Control Bit 21"]
    #[inline(always)]
    pub fn ctr21(&self) -> CTR21_R {
        CTR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Protocol Control Bit 22"]
    #[inline(always)]
    pub fn ctr22(&self) -> CTR22_R {
        CTR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Protocol Control Bit 23"]
    #[inline(always)]
    pub fn ctr23(&self) -> CTR23_R {
        CTR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protocol Control Bit 24"]
    #[inline(always)]
    pub fn ctr24(&self) -> CTR24_R {
        CTR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protocol Control Bit 25"]
    #[inline(always)]
    pub fn ctr25(&self) -> CTR25_R {
        CTR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protocol Control Bit 26"]
    #[inline(always)]
    pub fn ctr26(&self) -> CTR26_R {
        CTR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Control Bit 27"]
    #[inline(always)]
    pub fn ctr27(&self) -> CTR27_R {
        CTR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Control Bit 28"]
    #[inline(always)]
    pub fn ctr28(&self) -> CTR28_R {
        CTR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Protocol Control Bit 29"]
    #[inline(always)]
    pub fn ctr29(&self) -> CTR29_R {
        CTR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Protocol Control Bit 30"]
    #[inline(always)]
    pub fn ctr30(&self) -> CTR30_R {
        CTR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Protocol Control Bit 31"]
    #[inline(always)]
    pub fn ctr31(&self) -> CTR31_R {
        CTR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protocol Control Bit 0"]
    #[inline(always)]
    pub fn ctr0(&mut self) -> CTR0_W {
        CTR0_W { w: self }
    }
    #[doc = "Bit 1 - Protocol Control Bit 1"]
    #[inline(always)]
    pub fn ctr1(&mut self) -> CTR1_W {
        CTR1_W { w: self }
    }
    #[doc = "Bit 2 - Protocol Control Bit 2"]
    #[inline(always)]
    pub fn ctr2(&mut self) -> CTR2_W {
        CTR2_W { w: self }
    }
    #[doc = "Bit 3 - Protocol Control Bit 3"]
    #[inline(always)]
    pub fn ctr3(&mut self) -> CTR3_W {
        CTR3_W { w: self }
    }
    #[doc = "Bit 4 - Protocol Control Bit 4"]
    #[inline(always)]
    pub fn ctr4(&mut self) -> CTR4_W {
        CTR4_W { w: self }
    }
    #[doc = "Bit 5 - Protocol Control Bit 5"]
    #[inline(always)]
    pub fn ctr5(&mut self) -> CTR5_W {
        CTR5_W { w: self }
    }
    #[doc = "Bit 6 - Protocol Control Bit 6"]
    #[inline(always)]
    pub fn ctr6(&mut self) -> CTR6_W {
        CTR6_W { w: self }
    }
    #[doc = "Bit 7 - Protocol Control Bit 7"]
    #[inline(always)]
    pub fn ctr7(&mut self) -> CTR7_W {
        CTR7_W { w: self }
    }
    #[doc = "Bit 8 - Protocol Control Bit 8"]
    #[inline(always)]
    pub fn ctr8(&mut self) -> CTR8_W {
        CTR8_W { w: self }
    }
    #[doc = "Bit 9 - Protocol Control Bit 9"]
    #[inline(always)]
    pub fn ctr9(&mut self) -> CTR9_W {
        CTR9_W { w: self }
    }
    #[doc = "Bit 10 - Protocol Control Bit 10"]
    #[inline(always)]
    pub fn ctr10(&mut self) -> CTR10_W {
        CTR10_W { w: self }
    }
    #[doc = "Bit 11 - Protocol Control Bit 11"]
    #[inline(always)]
    pub fn ctr11(&mut self) -> CTR11_W {
        CTR11_W { w: self }
    }
    #[doc = "Bit 12 - Protocol Control Bit 12"]
    #[inline(always)]
    pub fn ctr12(&mut self) -> CTR12_W {
        CTR12_W { w: self }
    }
    #[doc = "Bit 13 - Protocol Control Bit 13"]
    #[inline(always)]
    pub fn ctr13(&mut self) -> CTR13_W {
        CTR13_W { w: self }
    }
    #[doc = "Bit 14 - Protocol Control Bit 14"]
    #[inline(always)]
    pub fn ctr14(&mut self) -> CTR14_W {
        CTR14_W { w: self }
    }
    #[doc = "Bit 15 - Protocol Control Bit 15"]
    #[inline(always)]
    pub fn ctr15(&mut self) -> CTR15_W {
        CTR15_W { w: self }
    }
    #[doc = "Bit 16 - Protocol Control Bit 16"]
    #[inline(always)]
    pub fn ctr16(&mut self) -> CTR16_W {
        CTR16_W { w: self }
    }
    #[doc = "Bit 17 - Protocol Control Bit 17"]
    #[inline(always)]
    pub fn ctr17(&mut self) -> CTR17_W {
        CTR17_W { w: self }
    }
    #[doc = "Bit 18 - Protocol Control Bit 18"]
    #[inline(always)]
    pub fn ctr18(&mut self) -> CTR18_W {
        CTR18_W { w: self }
    }
    #[doc = "Bit 19 - Protocol Control Bit 19"]
    #[inline(always)]
    pub fn ctr19(&mut self) -> CTR19_W {
        CTR19_W { w: self }
    }
    #[doc = "Bit 20 - Protocol Control Bit 20"]
    #[inline(always)]
    pub fn ctr20(&mut self) -> CTR20_W {
        CTR20_W { w: self }
    }
    #[doc = "Bit 21 - Protocol Control Bit 21"]
    #[inline(always)]
    pub fn ctr21(&mut self) -> CTR21_W {
        CTR21_W { w: self }
    }
    #[doc = "Bit 22 - Protocol Control Bit 22"]
    #[inline(always)]
    pub fn ctr22(&mut self) -> CTR22_W {
        CTR22_W { w: self }
    }
    #[doc = "Bit 23 - Protocol Control Bit 23"]
    #[inline(always)]
    pub fn ctr23(&mut self) -> CTR23_W {
        CTR23_W { w: self }
    }
    #[doc = "Bit 24 - Protocol Control Bit 24"]
    #[inline(always)]
    pub fn ctr24(&mut self) -> CTR24_W {
        CTR24_W { w: self }
    }
    #[doc = "Bit 25 - Protocol Control Bit 25"]
    #[inline(always)]
    pub fn ctr25(&mut self) -> CTR25_W {
        CTR25_W { w: self }
    }
    #[doc = "Bit 26 - Protocol Control Bit 26"]
    #[inline(always)]
    pub fn ctr26(&mut self) -> CTR26_W {
        CTR26_W { w: self }
    }
    #[doc = "Bit 27 - Protocol Control Bit 27"]
    #[inline(always)]
    pub fn ctr27(&mut self) -> CTR27_W {
        CTR27_W { w: self }
    }
    #[doc = "Bit 28 - Protocol Control Bit 28"]
    #[inline(always)]
    pub fn ctr28(&mut self) -> CTR28_W {
        CTR28_W { w: self }
    }
    #[doc = "Bit 29 - Protocol Control Bit 29"]
    #[inline(always)]
    pub fn ctr29(&mut self) -> CTR29_W {
        CTR29_W { w: self }
    }
    #[doc = "Bit 30 - Protocol Control Bit 30"]
    #[inline(always)]
    pub fn ctr30(&mut self) -> CTR30_W {
        CTR30_W { w: self }
    }
    #[doc = "Bit 31 - Protocol Control Bit 31"]
    #[inline(always)]
    pub fn ctr31(&mut self) -> CTR31_W {
        CTR31_W { w: self }
    }
}
