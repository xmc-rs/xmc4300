#[doc = "Reader of register GLOBRESD"]
pub type R = crate::R<u32, super::GLOBRESD>;
#[doc = "Writer for register GLOBRESD"]
pub type W = crate::W<u32, super::GLOBRESD>;
#[doc = "Register GLOBRESD `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBRESD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESULT`"]
pub struct RESULT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `GNR`"]
pub type GNR_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHNR`"]
pub type CHNR_R = crate::R<u8, u8>;
#[doc = "Reader of field `EMUX`"]
pub type EMUX_R = crate::R<u8, u8>;
#[doc = "Reader of field `CRS`"]
pub type CRS_R = crate::R<u8, u8>;
#[doc = "Fast Compare Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCR_A {
    #[doc = "0: Signal level was below compare value"]
    VALUE1 = 0,
    #[doc = "1: Signal level was above compare value"]
    VALUE2 = 1,
}
impl From<FCR_A> for bool {
    #[inline(always)]
    fn from(variant: FCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCR`"]
pub type FCR_R = crate::R<bool, FCR_A>;
impl FCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCR_A {
        match self.bits {
            false => FCR_A::VALUE1,
            true => FCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCR_A::VALUE2
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VF_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF_A> for bool {
    #[inline(always)]
    fn from(variant: VF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VF`"]
pub type VF_R = crate::R<bool, VF_A>;
impl VF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VF_A {
        match self.bits {
            false => VF_A::VALUE1,
            true => VF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF_A::VALUE2
    }
}
#[doc = "Write proxy for field `VF`"]
pub struct VF_W<'a> {
    w: &'a mut W,
}
impl<'a> VF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VF_A::VALUE1)
    }
    #[doc = "Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Group Number"]
    #[inline(always)]
    pub fn gnr(&self) -> GNR_R {
        GNR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - Channel Number"]
    #[inline(always)]
    pub fn chnr(&self) -> CHNR_R {
        CHNR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27 - External Multiplexer Setting"]
    #[inline(always)]
    pub fn emux(&self) -> EMUX_R {
        EMUX_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:29 - Converted Request Source"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Fast Compare Result"]
    #[inline(always)]
    pub fn fcr(&self) -> FCR_R {
        FCR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&mut self) -> RESULT_W {
        RESULT_W { w: self }
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&mut self) -> VF_W {
        VF_W { w: self }
    }
}
