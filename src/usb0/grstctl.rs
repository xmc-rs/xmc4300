#[doc = "Reader of register GRSTCTL"]
pub type R = crate::R<u32, super::GRSTCTL>;
#[doc = "Writer for register GRSTCTL"]
pub type W = crate::W<u32, super::GRSTCTL>;
#[doc = "Register GRSTCTL `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::GRSTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `CSftRst`"]
pub type CSFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSftRst`"]
pub struct CSFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CSFTRST_W<'a> {
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
#[doc = "Reader of field `FrmCntrRst`"]
pub type FRMCNTRRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FrmCntrRst`"]
pub struct FRMCNTRRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCNTRRST_W<'a> {
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
#[doc = "Reader of field `RxFFlsh`"]
pub type RXFFLSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxFFlsh`"]
pub struct RXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFLSH_W<'a> {
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
#[doc = "Reader of field `TxFFlsh`"]
pub type TXFFLSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TxFFlsh`"]
pub struct TXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFFLSH_W<'a> {
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
#[doc = "TxFIFO Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFNUM_A {
    #[doc = "0: Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    VALUE1,
    #[doc = "1: Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    VALUE2,
    #[doc = "2: Tx FIFO 2 flush in device mode"]
    VALUE3,
    #[doc = "15: Tx FIFO 15 flush in device mode"]
    VALUE4,
    #[doc = "16: Flush all the transmit FIFOs in device or host mode."]
    VALUE5,
}
impl From<TXFNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFNUM_A) -> Self {
        match variant {
            TXFNUM_A::VALUE1 => 0,
            TXFNUM_A::VALUE2 => 1,
            TXFNUM_A::VALUE3 => 2,
            TXFNUM_A::VALUE4 => 15,
            TXFNUM_A::VALUE5 => 16,
        }
    }
}
#[doc = "Reader of field `TxFNum`"]
pub type TXFNUM_R = crate::R<u8, TXFNUM_A>;
impl TXFNUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXFNUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXFNUM_A::VALUE1),
            1 => Val(TXFNUM_A::VALUE2),
            2 => Val(TXFNUM_A::VALUE3),
            15 => Val(TXFNUM_A::VALUE4),
            16 => Val(TXFNUM_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXFNUM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TXFNUM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TXFNUM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TXFNUM_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TXFNUM_A::VALUE5
    }
}
#[doc = "Write proxy for field `TxFNum`"]
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFNUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXFNUM_A::VALUE1)
    }
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXFNUM_A::VALUE2)
    }
    #[doc = "Tx FIFO 2 flush in device mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TXFNUM_A::VALUE3)
    }
    #[doc = "Tx FIFO 15 flush in device mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TXFNUM_A::VALUE4)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(TXFNUM_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMAReq`"]
pub type DMAREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `AHBIdle`"]
pub type AHBIDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csft_rst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    pub fn frm_cntr_rst(&self) -> FRMCNTRRST_R {
        FRMCNTRRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rx_fflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn tx_fflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csft_rst(&mut self) -> CSFTRST_W {
        CSFTRST_W { w: self }
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    pub fn frm_cntr_rst(&mut self) -> FRMCNTRRST_W {
        FRMCNTRRST_W { w: self }
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rx_fflsh(&mut self) -> RXFFLSH_W {
        RXFFLSH_W { w: self }
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn tx_fflsh(&mut self) -> TXFFLSH_W {
        TXFFLSH_W { w: self }
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
}
