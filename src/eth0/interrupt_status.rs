#[doc = "Register `INTERRUPT_STATUS` reader"]
pub type R = crate::R<InterruptStatusSpec>;
#[doc = "Field `PMTIS` reader - PMT Interrupt Status"]
pub type PmtisR = crate::BitReader;
#[doc = "Field `MMCIS` reader - MMC Interrupt Status"]
pub type MmcisR = crate::BitReader;
#[doc = "Field `MMCRXIS` reader - MMC Receive Interrupt Status"]
pub type MmcrxisR = crate::BitReader;
#[doc = "Field `MMCTXIS` reader - MMC Transmit Interrupt Status"]
pub type MmctxisR = crate::BitReader;
#[doc = "Field `MMCRXIPIS` reader - MMC Receive Checksum Offload Interrupt Status"]
pub type MmcrxipisR = crate::BitReader;
#[doc = "Field `TSIS` reader - Timestamp Interrupt Status"]
pub type TsisR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn pmtis(&self) -> PmtisR {
        PmtisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn mmcis(&self) -> MmcisR {
        MmcisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MmcrxisR {
        MmcrxisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MmctxisR {
        MmctxisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxipis(&self) -> MmcrxipisR {
        MmcrxipisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn tsis(&self) -> TsisR {
        TsisR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptStatusSpec;
impl crate::RegisterSpec for InterruptStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_status::R`](R) reader structure"]
impl crate::Readable for InterruptStatusSpec {}
#[doc = "`reset()` method sets INTERRUPT_STATUS to value 0"]
impl crate::Resettable for InterruptStatusSpec {
    const RESET_VALUE: u32 = 0;
}
