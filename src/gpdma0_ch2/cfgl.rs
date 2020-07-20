#[doc = "Reader of register CFGL"]
pub type R = crate::R<u32, super::CFGL>;
#[doc = "Writer for register CFGL"]
pub type W = crate::W<u32, super::CFGL>;
#[doc = "Register CFGL `reset()`'s with value 0x0e00"]
impl crate::ResetValue for super::CFGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e00
    }
}
#[doc = "Reader of field `MAX_ABRST`"]
pub type MAX_ABRST_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_ABRST`"]
pub struct MAX_ABRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_ABRST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Source Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<SRC_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC_HS_POL`"]
pub type SRC_HS_POL_R = crate::R<bool, SRC_HS_POL_A>;
impl SRC_HS_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_HS_POL_A {
        match self.bits {
            false => SRC_HS_POL_A::VALUE1,
            true => SRC_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRC_HS_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRC_HS_POL_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRC_HS_POL`"]
pub struct SRC_HS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_HS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_HS_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRC_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRC_HS_POL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Destination Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<DST_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DST_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DST_HS_POL`"]
pub type DST_HS_POL_R = crate::R<bool, DST_HS_POL_A>;
impl DST_HS_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_HS_POL_A {
        match self.bits {
            false => DST_HS_POL_A::VALUE1,
            true => DST_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DST_HS_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DST_HS_POL_A::VALUE2
    }
}
#[doc = "Write proxy for field `DST_HS_POL`"]
pub struct DST_HS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_HS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_HS_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DST_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DST_HS_POL_A::VALUE2)
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
#[doc = "Reader of field `LOCK_B`"]
pub type LOCK_B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_B`"]
pub struct LOCK_B_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_B_W<'a> {
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
#[doc = "Reader of field `LOCK_CH`"]
pub type LOCK_CH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_CH`"]
pub struct LOCK_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_CH_W<'a> {
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
#[doc = "Bus Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_B_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_B_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_B_L_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCK_B_L`"]
pub type LOCK_B_L_R = crate::R<u8, LOCK_B_L_A>;
impl LOCK_B_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_B_L_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCK_B_L_A::VALUE1),
            1 => Val(LOCK_B_L_A::VALUE2),
            2 => Val(LOCK_B_L_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_B_L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_B_L_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_B_L_A::VALUE3
    }
}
#[doc = "Write proxy for field `LOCK_B_L`"]
pub struct LOCK_B_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_B_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_B_L_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Channel Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_CH_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_CH_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_CH_L_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOCK_CH_L`"]
pub type LOCK_CH_L_R = crate::R<u8, LOCK_CH_L_A>;
impl LOCK_CH_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_CH_L_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCK_CH_L_A::VALUE1),
            1 => Val(LOCK_CH_L_A::VALUE2),
            2 => Val(LOCK_CH_L_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE3
    }
}
#[doc = "Write proxy for field `LOCK_CH_L`"]
pub struct LOCK_CH_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_CH_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_CH_L_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Source Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_SRC_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HS_SEL_SRC`"]
pub type HS_SEL_SRC_R = crate::R<bool, HS_SEL_SRC_A>;
impl HS_SEL_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_SRC_A {
        match self.bits {
            false => HS_SEL_SRC_A::VALUE1,
            true => HS_SEL_SRC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_SRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_SRC_A::VALUE2
    }
}
#[doc = "Write proxy for field `HS_SEL_SRC`"]
pub struct HS_SEL_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_SEL_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_SEL_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Destination Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_DST_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_DST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_DST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HS_SEL_DST`"]
pub type HS_SEL_DST_R = crate::R<bool, HS_SEL_DST_A>;
impl HS_SEL_DST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_DST_A {
        match self.bits {
            false => HS_SEL_DST_A::VALUE1,
            true => HS_SEL_DST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_DST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_DST_A::VALUE2
    }
}
#[doc = "Write proxy for field `HS_SEL_DST`"]
pub struct HS_SEL_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_SEL_DST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_SEL_DST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_DST_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_DST_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Indicates if there is data left in the channel FIFO\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_EMPTY_A {
    #[doc = "1: Channel FIFO empty"]
    VALUE1 = 1,
    #[doc = "0: Channel FIFO not empty"]
    VALUE2 = 0,
}
impl From<FIFO_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIFO_EMPTY`"]
pub type FIFO_EMPTY_R = crate::R<bool, FIFO_EMPTY_A>;
impl FIFO_EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_EMPTY_A {
        match self.bits {
            true => FIFO_EMPTY_A::VALUE1,
            false => FIFO_EMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_EMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_EMPTY_A::VALUE2
    }
}
#[doc = "Channel Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_SUSP_A {
    #[doc = "0: Not suspended."]
    VALUE1 = 0,
    #[doc = "1: Suspend DMA transfer from the source."]
    VALUE2 = 1,
}
impl From<CH_SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: CH_SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH_SUSP`"]
pub type CH_SUSP_R = crate::R<bool, CH_SUSP_A>;
impl CH_SUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_SUSP_A {
        match self.bits {
            false => CH_SUSP_A::VALUE1,
            true => CH_SUSP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH_SUSP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH_SUSP_A::VALUE2
    }
}
#[doc = "Write proxy for field `CH_SUSP`"]
pub struct CH_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_SUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_SUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not suspended."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH_SUSP_A::VALUE1)
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH_SUSP_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH_PRIOR`"]
pub type CH_PRIOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH_PRIOR`"]
pub struct CH_PRIOR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_PRIOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    pub fn max_abrst(&self) -> MAX_ABRST_R {
        MAX_ABRST_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&self) -> SRC_HS_POL_R {
        SRC_HS_POL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&self) -> DST_HS_POL_R {
        DST_HS_POL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    pub fn lock_b(&self) -> LOCK_B_R {
        LOCK_B_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LOCK_CH_R {
        LOCK_CH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    pub fn lock_b_l(&self) -> LOCK_B_L_R {
        LOCK_B_L_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LOCK_CH_L_R {
        LOCK_CH_L_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HS_SEL_SRC_R {
        HS_SEL_SRC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HS_SEL_DST_R {
        HS_SEL_DST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&self) -> CH_SUSP_R {
        CH_SUSP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    pub fn ch_prior(&self) -> CH_PRIOR_R {
        CH_PRIOR_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    pub fn max_abrst(&mut self) -> MAX_ABRST_W {
        MAX_ABRST_W { w: self }
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&mut self) -> SRC_HS_POL_W {
        SRC_HS_POL_W { w: self }
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&mut self) -> DST_HS_POL_W {
        DST_HS_POL_W { w: self }
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    pub fn lock_b(&mut self) -> LOCK_B_W {
        LOCK_B_W { w: self }
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    pub fn lock_ch(&mut self) -> LOCK_CH_W {
        LOCK_CH_W { w: self }
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    pub fn lock_b_l(&mut self) -> LOCK_B_L_W {
        LOCK_B_L_W { w: self }
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    pub fn lock_ch_l(&mut self) -> LOCK_CH_L_W {
        LOCK_CH_L_W { w: self }
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&mut self) -> HS_SEL_SRC_W {
        HS_SEL_SRC_W { w: self }
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&mut self) -> HS_SEL_DST_W {
        HS_SEL_DST_W { w: self }
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&mut self) -> CH_SUSP_W {
        CH_SUSP_W { w: self }
    }
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    pub fn ch_prior(&mut self) -> CH_PRIOR_W {
        CH_PRIOR_W { w: self }
    }
}
