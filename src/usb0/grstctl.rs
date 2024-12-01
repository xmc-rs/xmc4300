#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GRSTCTL_SPEC>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GRSTCTL_SPEC>;
#[doc = "Field `CSftRst` reader - Core Soft Reset"]
pub type CSFT_RST_R = crate::BitReader;
#[doc = "Field `CSftRst` writer - Core Soft Reset"]
pub type CSFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FrmCntrRst` reader - Host Frame Counter Reset"]
pub type FRM_CNTR_RST_R = crate::BitReader;
#[doc = "Field `FrmCntrRst` writer - Host Frame Counter Reset"]
pub type FRM_CNTR_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFFlsh` reader - RxFIFO Flush"]
pub type RX_FFLSH_R = crate::BitReader;
#[doc = "Field `RxFFlsh` writer - RxFIFO Flush"]
pub type RX_FFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFFlsh` reader - TxFIFO Flush"]
pub type TX_FFLSH_R = crate::BitReader;
#[doc = "Field `TxFFlsh` writer - TxFIFO Flush"]
pub type TX_FFLSH_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl crate::FieldSpec for TX_FNUM_A {
    type Ux = u8;
}
impl crate::IsEnum for TX_FNUM_A {}
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TX_FNUM_R = crate::FieldReader<TX_FNUM_A>;
impl TX_FNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TX_FNUM_A> {
        match self.bits {
            0 => Some(TX_FNUM_A::VALUE1),
            1 => Some(TX_FNUM_A::VALUE2),
            2 => Some(TX_FNUM_A::VALUE3),
            15 => Some(TX_FNUM_A::VALUE4),
            16 => Some(TX_FNUM_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_FNUM_A::VALUE1
    }
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_FNUM_A::VALUE2
    }
    #[doc = "Tx FIFO 2 flush in device mode"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TX_FNUM_A::VALUE3
    }
    #[doc = "Tx FIFO 15 flush in device mode"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TX_FNUM_A::VALUE4
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TX_FNUM_A::VALUE5
    }
}
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TX_FNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 5, TX_FNUM_A>;
impl<'a, REG> TX_FNUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-periodic TxFIFO flush in Host mode or Tx FIFO 0 flush in device mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FNUM_A::VALUE1)
    }
    #[doc = "Periodic TxFIFO flush in Host mode or Tx FIFO 1 flush in device mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FNUM_A::VALUE2)
    }
    #[doc = "Tx FIFO 2 flush in device mode"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FNUM_A::VALUE3)
    }
    #[doc = "Tx FIFO 15 flush in device mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FNUM_A::VALUE4)
    }
    #[doc = "Flush all the transmit FIFOs in device or host mode."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(TX_FNUM_A::VALUE5)
    }
}
#[doc = "Field `DMAReq` reader - DMA Request Signal"]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIdle` reader - AHB Master Idle"]
pub type AHBIDLE_R = crate::BitReader;
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
    pub fn csft_rst(&mut self) -> CSFT_RST_W<GRSTCTL_SPEC> {
        CSFT_RST_W::new(self, 0)
    }
    #[doc = "Bit 2 - Host Frame Counter Reset"]
    #[inline(always)]
    pub fn frm_cntr_rst(&mut self) -> FRM_CNTR_RST_W<GRSTCTL_SPEC> {
        FRM_CNTR_RST_W::new(self, 2)
    }
    #[doc = "Bit 4 - RxFIFO Flush"]
    #[inline(always)]
    pub fn rx_fflsh(&mut self) -> RX_FFLSH_W<GRSTCTL_SPEC> {
        RX_FFLSH_W::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO Flush"]
    #[inline(always)]
    pub fn tx_fflsh(&mut self) -> TX_FFLSH_W<GRSTCTL_SPEC> {
        TX_FFLSH_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&mut self) -> TX_FNUM_W<GRSTCTL_SPEC> {
        TX_FNUM_W::new(self, 6)
    }
}
#[doc = "Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x1000_0000"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: u32 = 0x1000_0000;
}
