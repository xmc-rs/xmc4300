#[doc = "Reader of register CTLL"]
pub type R = crate::R<u32, super::CTLL>;
#[doc = "Writer for register CTLL"]
pub type W = crate::W<u32, super::CTLL>;
#[doc = "Register CTLL `reset()`'s with value 0x0030_4801"]
impl crate::ResetValue for super::CTLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0030_4801
    }
}
#[doc = "Reader of field `LLP_SRC_EN`"]
pub type LLP_SRC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LLP_SRC_EN`"]
pub struct LLP_SRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LLP_SRC_EN_W<'a> {
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
#[doc = "Reader of field `LLP_DST_EN`"]
pub type LLP_DST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LLP_DST_EN`"]
pub struct LLP_DST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LLP_DST_EN_W<'a> {
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
#[doc = "Reader of field `TT_FC`"]
pub type TT_FC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TT_FC`"]
pub struct TT_FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TT_FC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Destination scatter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_SCATTER_EN_A {
    #[doc = "0: Scatter disabled"]
    VALUE1 = 0,
    #[doc = "1: Scatter enabled"]
    VALUE2 = 1,
}
impl From<DST_SCATTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DST_SCATTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DST_SCATTER_EN`"]
pub type DST_SCATTER_EN_R = crate::R<bool, DST_SCATTER_EN_A>;
impl DST_SCATTER_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_SCATTER_EN_A {
        match self.bits {
            false => DST_SCATTER_EN_A::VALUE1,
            true => DST_SCATTER_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DST_SCATTER_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DST_SCATTER_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `DST_SCATTER_EN`"]
pub struct DST_SCATTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_SCATTER_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_SCATTER_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Scatter disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DST_SCATTER_EN_A::VALUE1)
    }
    #[doc = "Scatter enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DST_SCATTER_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Source gather enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_GATHER_EN_A {
    #[doc = "0: Gather disabled"]
    VALUE1 = 0,
    #[doc = "1: Gather enabled"]
    VALUE2 = 1,
}
impl From<SRC_GATHER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_GATHER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC_GATHER_EN`"]
pub type SRC_GATHER_EN_R = crate::R<bool, SRC_GATHER_EN_A>;
impl SRC_GATHER_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_GATHER_EN_A {
        match self.bits {
            false => SRC_GATHER_EN_A::VALUE1,
            true => SRC_GATHER_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRC_GATHER_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRC_GATHER_EN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRC_GATHER_EN`"]
pub struct SRC_GATHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_GATHER_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_GATHER_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Gather disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRC_GATHER_EN_A::VALUE1)
    }
    #[doc = "Gather enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRC_GATHER_EN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SRC_MSIZE`"]
pub type SRC_MSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRC_MSIZE`"]
pub struct SRC_MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `DEST_MSIZE`"]
pub type DEST_MSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEST_MSIZE`"]
pub struct DEST_MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_MSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Source Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<SINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SINC`"]
pub type SINC_R = crate::R<u8, SINC_A>;
impl SINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SINC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SINC_A::VALUE1),
            1 => Val(SINC_A::VALUE2),
            2 => Val(SINC_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SINC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SINC_A::VALUE3
    }
}
#[doc = "Write proxy for field `SINC`"]
pub struct SINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SINC_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Destination Address Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DINC_A {
    #[doc = "0: Increment"]
    VALUE1 = 0,
    #[doc = "1: Decrement"]
    VALUE2 = 1,
    #[doc = "2: No change"]
    VALUE3 = 2,
}
impl From<DINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DINC`"]
pub type DINC_R = crate::R<u8, DINC_A>;
impl DINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DINC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DINC_A::VALUE1),
            1 => Val(DINC_A::VALUE2),
            2 => Val(DINC_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DINC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DINC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DINC_A::VALUE3
    }
}
#[doc = "Write proxy for field `DINC`"]
pub struct DINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Increment"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DINC_A::VALUE1)
    }
    #[doc = "Decrement"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DINC_A::VALUE2)
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(DINC_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `SRC_TR_WIDTH`"]
pub type SRC_TR_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRC_TR_WIDTH`"]
pub struct SRC_TR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_TR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DST_TR_WIDTH`"]
pub type DST_TR_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DST_TR_WIDTH`"]
pub struct DST_TR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_TR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `INT_EN`"]
pub type INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_EN`"]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
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
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline(always)]
    pub fn llp_src_en(&self) -> LLP_SRC_EN_R {
        LLP_SRC_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline(always)]
    pub fn llp_dst_en(&self) -> LLP_DST_EN_R {
        LLP_DST_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TT_FC_R {
        TT_FC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline(always)]
    pub fn dst_scatter_en(&self) -> DST_SCATTER_EN_R {
        DST_SCATTER_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline(always)]
    pub fn src_gather_en(&self) -> SRC_GATHER_EN_R {
        SRC_GATHER_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SRC_MSIZE_R {
        SRC_MSIZE_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&self) -> DEST_MSIZE_R {
        DEST_MSIZE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SRC_TR_WIDTH_R {
        SRC_TR_WIDTH_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DST_TR_WIDTH_R {
        DST_TR_WIDTH_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Linked List Pointer for Source Enable"]
    #[inline(always)]
    pub fn llp_src_en(&mut self) -> LLP_SRC_EN_W {
        LLP_SRC_EN_W { w: self }
    }
    #[doc = "Bit 27 - Linked List Pointer for Destination Enable"]
    #[inline(always)]
    pub fn llp_dst_en(&mut self) -> LLP_DST_EN_W {
        LLP_DST_EN_W { w: self }
    }
    #[doc = "Bits 20:22 - Transfer Type and Flow Control"]
    #[inline(always)]
    pub fn tt_fc(&mut self) -> TT_FC_W {
        TT_FC_W { w: self }
    }
    #[doc = "Bit 18 - Destination scatter enable"]
    #[inline(always)]
    pub fn dst_scatter_en(&mut self) -> DST_SCATTER_EN_W {
        DST_SCATTER_EN_W { w: self }
    }
    #[doc = "Bit 17 - Source gather enable"]
    #[inline(always)]
    pub fn src_gather_en(&mut self) -> SRC_GATHER_EN_W {
        SRC_GATHER_EN_W { w: self }
    }
    #[doc = "Bits 14:16 - Source Burst Transaction Length"]
    #[inline(always)]
    pub fn src_msize(&mut self) -> SRC_MSIZE_W {
        SRC_MSIZE_W { w: self }
    }
    #[doc = "Bits 11:13 - Destination Burst Transaction Length"]
    #[inline(always)]
    pub fn dest_msize(&mut self) -> DEST_MSIZE_W {
        DEST_MSIZE_W { w: self }
    }
    #[doc = "Bits 9:10 - Source Address Increment"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W {
        SINC_W { w: self }
    }
    #[doc = "Bits 7:8 - Destination Address Increment"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W {
        DINC_W { w: self }
    }
    #[doc = "Bits 4:6 - Source Transfer Width"]
    #[inline(always)]
    pub fn src_tr_width(&mut self) -> SRC_TR_WIDTH_W {
        SRC_TR_WIDTH_W { w: self }
    }
    #[doc = "Bits 1:3 - Destination Transfer Width"]
    #[inline(always)]
    pub fn dst_tr_width(&mut self) -> DST_TR_WIDTH_W {
        DST_TR_WIDTH_W { w: self }
    }
    #[doc = "Bit 0 - Interrupt Enable Bit"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
}
