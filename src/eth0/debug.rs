#[doc = "Register `DEBUG` reader"]
pub type R = crate::R<DebugSpec>;
#[doc = "Field `RPESTS` reader - MAC MII Receive Protocol Engine Status"]
pub type RpestsR = crate::BitReader;
#[doc = "Field `RFCFCSTS` reader - MAC Receive Frame Controller FIFO Status"]
pub type RfcfcstsR = crate::FieldReader;
#[doc = "Field `RWCSTS` reader - MTL Rx FIFO Write Controller Active Status"]
pub type RwcstsR = crate::BitReader;
#[doc = "Field `RRCSTS` reader - MTL Rx FIFO Read Controller State"]
pub type RrcstsR = crate::FieldReader;
#[doc = "Field `RXFSTS` reader - MTL Rx FIFO Fill-level Status"]
pub type RxfstsR = crate::FieldReader;
#[doc = "Field `TPESTS` reader - MAC MII Transmit Protocol Engine Status"]
pub type TpestsR = crate::BitReader;
#[doc = "Field `TFCSTS` reader - MAC Transmit Frame Controller Status"]
pub type TfcstsR = crate::FieldReader;
#[doc = "Field `TXPAUSED` reader - MAC transmitter in PAUSE"]
pub type TxpausedR = crate::BitReader;
#[doc = "Field `TRCSTS` reader - MTL Tx FIFO Read Controller Status"]
pub type TrcstsR = crate::FieldReader;
#[doc = "Field `TWCSTS` reader - MTL Tx FIFO Write Controller Active Status"]
pub type TwcstsR = crate::BitReader;
#[doc = "Field `TXFSTS` reader - MTL Tx FIFO Not Empty Status"]
pub type TxfstsR = crate::BitReader;
#[doc = "Field `TXSTSFSTS` reader - MTL TxStatus FIFO Full Status"]
pub type TxstsfstsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RpestsR {
        RpestsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RfcfcstsR {
        RfcfcstsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - MTL Rx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RwcstsR {
        RwcstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - MTL Rx FIFO Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RrcstsR {
        RrcstsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MTL Rx FIFO Fill-level Status"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RxfstsR {
        RxfstsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TpestsR {
        TpestsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TfcstsR {
        TfcstsR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - MAC transmitter in PAUSE"]
    #[inline(always)]
    pub fn txpaused(&self) -> TxpausedR {
        TxpausedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - MTL Tx FIFO Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TrcstsR {
        TrcstsR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - MTL Tx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TwcstsR {
        TwcstsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - MTL Tx FIFO Not Empty Status"]
    #[inline(always)]
    pub fn txfsts(&self) -> TxfstsR {
        TxfstsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTL TxStatus FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TxstsfstsR {
        TxstsfstsR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSpec;
impl crate::RegisterSpec for DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DebugSpec {}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DebugSpec {
    const RESET_VALUE: u32 = 0;
}
