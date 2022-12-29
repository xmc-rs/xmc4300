#[doc = "Register `INTERRUPT_STATUS` reader"]
pub struct R(crate::R<INTERRUPT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PMTIS` reader - PMT Interrupt Status"]
pub type PMTIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCIS` reader - MMC Interrupt Status"]
pub type MMCIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRXIS` reader - MMC Receive Interrupt Status"]
pub type MMCRXIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCTXIS` reader - MMC Transmit Interrupt Status"]
pub type MMCTXIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRXIPIS` reader - MMC Receive Checksum Offload Interrupt Status"]
pub type MMCRXIPIS_R = crate::BitReader<bool>;
#[doc = "Field `TSIS` reader - Timestamp Interrupt Status"]
pub type TSIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxipis(&self) -> MMCRXIPIS_R {
        MMCRXIPIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_status](index.html) module"]
pub struct INTERRUPT_STATUS_SPEC;
impl crate::RegisterSpec for INTERRUPT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_status::R](R) reader structure"]
impl crate::Readable for INTERRUPT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERRUPT_STATUS to value 0"]
impl crate::Resettable for INTERRUPT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
