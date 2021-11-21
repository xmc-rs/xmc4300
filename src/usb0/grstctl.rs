#[doc = "Register `GRSTCTL` reader"]
pub struct R(crate::R<GRSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRSTCTL` writer"]
pub struct W(crate::W<GRSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GRSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSftRst` reader - Core Soft Reset"]
pub struct CSFTRST_R(crate::FieldReader<bool, bool>);
impl CSFTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSFTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSFTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSftRst` writer - Core Soft Reset"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FrmCntrRst` reader - Host Frame Counter Reset"]
pub struct FRMCNTRRST_R(crate::FieldReader<bool, bool>);
impl FRMCNTRRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMCNTRRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMCNTRRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FrmCntrRst` writer - Host Frame Counter Reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RxFFlsh` reader - RxFIFO Flush"]
pub struct RXFFLSH_R(crate::FieldReader<bool, bool>);
impl RXFFLSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFFLSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFFLSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RxFFlsh` writer - RxFIFO Flush"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TxFFlsh` reader - TxFIFO Flush"]
pub struct TXFFLSH_R(crate::FieldReader<bool, bool>);
impl TXFFLSH_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFFLSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFFLSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxFFlsh` writer - TxFIFO Flush"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "TxFIFO Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFNUM_A {
    #[doc = "0: Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    VALUE1 = 0,
    #[doc = "1: Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    VALUE2 = 1,
    #[doc = "2: Tx FIFO 2 flush in device mode"]
    VALUE3 = 2,
    #[doc = "15: Tx FIFO 15 flush in device mode"]
    VALUE4 = 15,
    #[doc = "16: Flush all the transmit FIFOs in device or host mode."]
    VALUE5 = 16,
}
impl From<TXFNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub struct TXFNUM_R(crate::FieldReader<u8, TXFNUM_A>);
impl TXFNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFNUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFNUM_A> {
        match self.bits {
            0 => Some(TXFNUM_A::VALUE1),
            1 => Some(TXFNUM_A::VALUE2),
            2 => Some(TXFNUM_A::VALUE3),
            15 => Some(TXFNUM_A::VALUE4),
            16 => Some(TXFNUM_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TXFNUM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TXFNUM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TXFNUM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TXFNUM_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == TXFNUM_A::VALUE5
    }
}
impl core::ops::Deref for TXFNUM_R {
    type Target = crate::FieldReader<u8, TXFNUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
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
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `DMAReq` reader - DMA Request Signal"]
pub struct DMAREQ_R(crate::FieldReader<bool, bool>);
impl DMAREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBIdle` reader - AHB Master Idle"]
pub struct AHBIDLE_R(crate::FieldReader<bool, bool>);
impl AHBIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBIDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstctl](index.html) module"]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grstctl::R](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grstctl::W](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x1000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
