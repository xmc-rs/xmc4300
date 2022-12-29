#[doc = "Register `DEBUG` reader"]
pub struct R(crate::R<DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPESTS` reader - MAC MII Receive Protocol Engine Status"]
pub type RPESTS_R = crate::BitReader<bool>;
#[doc = "Field `RFCFCSTS` reader - MAC Receive Frame Controller FIFO Status"]
pub type RFCFCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWCSTS` reader - MTL Rx FIFO Write Controller Active Status"]
pub type RWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `RRCSTS` reader - MTL Rx FIFO Read Controller State"]
pub type RRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFSTS` reader - MTL Rx FIFO Fill-level Status"]
pub type RXFSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPESTS` reader - MAC MII Transmit Protocol Engine Status"]
pub type TPESTS_R = crate::BitReader<bool>;
#[doc = "Field `TFCSTS` reader - MAC Transmit Frame Controller Status"]
pub type TFCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXPAUSED` reader - MAC transmitter in PAUSE"]
pub type TXPAUSED_R = crate::BitReader<bool>;
#[doc = "Field `TRCSTS` reader - MTL Tx FIFO Read Controller Status"]
pub type TRCSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWCSTS` reader - MTL Tx FIFO Write Controller Active Status"]
pub type TWCSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXFSTS` reader - MTL Tx FIFO Not Empty Status"]
pub type TXFSTS_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSFSTS` reader - MTL TxStatus FIFO Full Status"]
pub type TXSTSFSTS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - MTL Rx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - MTL Rx FIFO Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MTL Rx FIFO Fill-level Status"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RXFSTS_R {
        RXFSTS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - MAC transmitter in PAUSE"]
    #[inline(always)]
    pub fn txpaused(&self) -> TXPAUSED_R {
        TXPAUSED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - MTL Tx FIFO Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - MTL Tx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - MTL Tx FIFO Not Empty Status"]
    #[inline(always)]
    pub fn txfsts(&self) -> TXFSTS_R {
        TXFSTS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTL TxStatus FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug](index.html) module"]
pub struct DEBUG_SPEC;
impl crate::RegisterSpec for DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug::R](R) reader structure"]
impl crate::Readable for DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
