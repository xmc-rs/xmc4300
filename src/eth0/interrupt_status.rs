#[doc = "Reader of register INTERRUPT_STATUS"]
pub type R = crate::R<u32, super::INTERRUPT_STATUS>;
#[doc = "Reader of field `PMTIS`"]
pub type PMTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCIS`"]
pub type MMCIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCRXIS`"]
pub type MMCRXIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCTXIS`"]
pub type MMCTXIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCRXIPIS`"]
pub type MMCRXIPIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSIS`"]
pub type TSIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxipis(&self) -> MMCRXIPIS_R {
        MMCRXIPIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
