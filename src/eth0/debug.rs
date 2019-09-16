#[doc = "Reader of register DEBUG"]
pub type R = crate::R<u32, super::DEBUG>;
#[doc = "Reader of field `RPESTS`"]
pub type RPESTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFCFCSTS`"]
pub type RFCFCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RWCSTS`"]
pub type RWCSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RRCSTS`"]
pub type RRCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXFSTS`"]
pub type RXFSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPESTS`"]
pub type TPESTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFCSTS`"]
pub type TFCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXPAUSED`"]
pub type TXPAUSED_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRCSTS`"]
pub type TRCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TWCSTS`"]
pub type TWCSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFSTS`"]
pub type TXFSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTSFSTS`"]
pub type TXSTSFSTS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 4 - MTL Rx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - MTL Rx FIFO Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - MTL Rx FIFO Fill-level Status"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RXFSTS_R {
        RXFSTS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - MAC transmitter in PAUSE"]
    #[inline(always)]
    pub fn txpaused(&self) -> TXPAUSED_R {
        TXPAUSED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - MTL Tx FIFO Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - MTL Tx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MTL Tx FIFO Not Empty Status"]
    #[inline(always)]
    pub fn txfsts(&self) -> TXFSTS_R {
        TXFSTS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MTL TxStatus FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
