#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GrstctlSpec>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GrstctlSpec>;
#[doc = "Field `CSftRst` reader - Core Soft Reset"]
pub type CsftRstR = crate::BitReader;
#[doc = "Field `CSftRst` writer - Core Soft Reset"]
pub type CsftRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FrmCntrRst` reader - Host Frame Counter Reset"]
pub type FrmCntrRstR = crate::BitReader;
#[doc = "Field `FrmCntrRst` writer - Host Frame Counter Reset"]
pub type FrmCntrRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFFlsh` reader - RxFIFO Flush"]
pub type RxFflshR = crate::BitReader;
#[doc = "Field `RxFFlsh` writer - RxFIFO Flush"]
pub type RxFflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFFlsh` reader - TxFIFO Flush"]
pub type TxFflshR = crate::BitReader;
#[doc = "Field `TxFFlsh` writer - TxFIFO Flush"]
pub type TxFflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TxFIFO Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxFnum {
    #[doc = "0: Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    Value1 = 0,
    #[doc = "1: Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    Value2 = 1,
    #[doc = "2: Tx FIFO 2 flush in device mode"]
    Value3 = 2,
    #[doc = "15: Tx FIFO 15 flush in device mode"]
    Value4 = 15,
    #[doc = "16: Flush all the transmit FIFOs in device or host mode."]
    Value5 = 16,
}
impl From<TxFnum> for u8 {
    #[inline(always)]
    fn from(variant: TxFnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxFnum {
    type Ux = u8;
}
impl crate::IsEnum for TxFnum {}
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TxFnumR = crate::FieldReader<TxFnum>;
impl TxFnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TxFnum> {
        match self.bits {
            0 => Some(TxFnum::Value1),
            1 => Some(TxFnum::Value2),
            2 => Some(TxFnum::Value3),
            15 => Some(TxFnum::Value4),
            16 => Some(TxFnum::Value5),
            _ => None,
        }
    }
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxFnum::Value1
    }
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxFnum::Value2
    }
    #[doc = "Tx FIFO 2 flush in device mode"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TxFnum::Value3
    }
    #[doc = "Tx FIFO 15 flush in device mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TxFnum::Value4
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TxFnum::Value5
    }
}
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TxFnumW<'a, REG> = crate::FieldWriter<'a, REG, 5, TxFnum>;
impl<'a, REG> TxFnumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxFnum::Value1)
    }
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxFnum::Value2)
    }
    #[doc = "Tx FIFO 2 flush in device mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TxFnum::Value3)
    }
    #[doc = "Tx FIFO 15 flush in device mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TxFnum::Value4)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(TxFnum::Value5)
    }
}
#[doc = "Field `DMAReq` reader - DMA Request Signal"]
pub type DmareqR = crate::BitReader;
#[doc = "Field `AHBIdle` reader - AHB Master Idle"]
pub type AhbidleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    pub fn csft_rst(&self) -> CsftRstR {
        CsftRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    pub fn frm_cntr_rst(&self) -> FrmCntrRstR {
        FrmCntrRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rx_fflsh(&self) -> RxFflshR {
        RxFflshR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn tx_fflsh(&self) -> TxFflshR {
        TxFflshR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&self) -> TxFnumR {
        TxFnumR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMA Request Signal"]
    #[inline(always)]
    pub fn dmareq(&self) -> DmareqR {
        DmareqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHB Master Idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AhbidleR {
        AhbidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn csft_rst(&mut self) -> CsftRstW<GrstctlSpec> {
        CsftRstW::new(self, 0)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    #[must_use]
    pub fn frm_cntr_rst(&mut self) -> FrmCntrRstW<GrstctlSpec> {
        FrmCntrRstW::new(self, 2)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fflsh(&mut self) -> RxFflshW<GrstctlSpec> {
        RxFflshW::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fflsh(&mut self) -> TxFflshW<GrstctlSpec> {
        TxFflshW::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fnum(&mut self) -> TxFnumW<GrstctlSpec> {
        TxFnumW::new(self, 6)
    }
}
#[doc = "Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrstctlSpec;
impl crate::RegisterSpec for GrstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GrstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GrstctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x1000_0000"]
impl crate::Resettable for GrstctlSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
