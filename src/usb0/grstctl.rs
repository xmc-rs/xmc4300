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
pub type CSFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `CSftRst` writer - Core Soft Reset"]
pub type CSFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
#[doc = "Field `FrmCntrRst` reader - Host Frame Counter Reset"]
pub type FRM_CNTR_RST_R = crate::BitReader<bool>;
#[doc = "Field `FrmCntrRst` writer - Host Frame Counter Reset"]
pub type FRM_CNTR_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
#[doc = "Field `RxFFlsh` reader - RxFIFO Flush"]
pub type RX_FFLSH_R = crate::BitReader<bool>;
#[doc = "Field `RxFFlsh` writer - RxFIFO Flush"]
pub type RX_FFLSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
#[doc = "Field `TxFFlsh` reader - TxFIFO Flush"]
pub type TX_FFLSH_R = crate::BitReader<bool>;
#[doc = "Field `TxFFlsh` writer - TxFIFO Flush"]
pub type TX_FFLSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GRSTCTL_SPEC, bool, O>;
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TX_FNUM_R = crate::FieldReader<u8, TX_FNUM_A>;
#[doc = "TxFIFO Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_FNUM_A {
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
impl From<TX_FNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_FNUM_A) -> Self {
        variant as _
    }
}
impl TX_FNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_FNUM_A> {
        match self.bits {
            0 => Some(TX_FNUM_A::VALUE1),
            1 => Some(TX_FNUM_A::VALUE2),
            2 => Some(TX_FNUM_A::VALUE3),
            15 => Some(TX_FNUM_A::VALUE4),
            16 => Some(TX_FNUM_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_FNUM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_FNUM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TX_FNUM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TX_FNUM_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TX_FNUM_A::VALUE5
    }
}
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TX_FNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GRSTCTL_SPEC, u8, TX_FNUM_A, 5, O>;
impl<'a, const O: u8> TX_FNUM_W<'a, O> {
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_FNUM_A::VALUE1)
    }
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_FNUM_A::VALUE2)
    }
    #[doc = "Tx FIFO 2 flush in device mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TX_FNUM_A::VALUE3)
    }
    #[doc = "Tx FIFO 15 flush in device mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TX_FNUM_A::VALUE4)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(TX_FNUM_A::VALUE5)
    }
}
#[doc = "Field `DMAReq` reader - DMA Request Signal"]
pub type DMAREQ_R = crate::BitReader<bool>;
#[doc = "Field `AHBIdle` reader - AHB Master Idle"]
pub type AHBIDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csft_rst(&self) -> CSFT_RST_R {
        CSFT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    pub fn frm_cntr_rst(&self) -> FRM_CNTR_RST_R {
        FRM_CNTR_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rx_fflsh(&self) -> RX_FFLSH_R {
        RX_FFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn tx_fflsh(&self) -> TX_FFLSH_R {
        TX_FFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&self) -> TX_FNUM_R {
        TX_FNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn csft_rst(&mut self) -> CSFT_RST_W<0> {
        CSFT_RST_W::new(self)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    #[must_use]
    pub fn frm_cntr_rst(&mut self) -> FRM_CNTR_RST_W<2> {
        FRM_CNTR_RST_W::new(self)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fflsh(&mut self) -> RX_FFLSH_W<4> {
        RX_FFLSH_W::new(self)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fflsh(&mut self) -> TX_FFLSH_W<5> {
        TX_FFLSH_W::new(self)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fnum(&mut self) -> TX_FNUM_W<6> {
        TX_FNUM_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x1000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
