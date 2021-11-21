#[doc = "Register `MMC_TRANSMIT_INTERRUPT` reader"]
pub struct R(crate::R<MMC_TRANSMIT_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_TRANSMIT_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_TRANSMIT_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_TRANSMIT_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXGBOCTIS` reader - MMC Transmit Good Bad Octet Counter Interrupt Status"]
pub struct TXGBOCTIS_R(crate::FieldReader<bool, bool>);
impl TXGBOCTIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGBOCTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGBOCTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGBFRMIS` reader - MMC Transmit Good Bad Frame Counter Interrupt Status"]
pub struct TXGBFRMIS_R(crate::FieldReader<bool, bool>);
impl TXGBFRMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGBFRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGBFRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBCGFIS` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Status"]
pub struct TXBCGFIS_R(crate::FieldReader<bool, bool>);
impl TXBCGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBCGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBCGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCGFIS` reader - MMC Transmit Multicast Good Frame Counter Interrupt Status"]
pub struct TXMCGFIS_R(crate::FieldReader<bool, bool>);
impl TXMCGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX64OCTGBFIS` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status."]
pub struct TX64OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl TX64OCTGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX64OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX64OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX65T127OCTGBFIS` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub struct TX65T127OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl TX65T127OCTGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX65T127OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX65T127OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX128T255OCTGBFIS` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub struct TX128T255OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl TX128T255OCTGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX128T255OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX128T255OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX256T511OCTGBFIS` reader - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub struct TX256T511OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl TX256T511OCTGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX256T511OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX256T511OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX512T1023OCTGBFIS` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub struct TX512T1023OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl TX512T1023OCTGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX512T1023OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX512T1023OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX1024TMAXOCTGBFIS` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
pub struct TX1024TMAXOCTGBFIS_R(crate::FieldReader<bool, bool>);
impl TX1024TMAXOCTGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX1024TMAXOCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX1024TMAXOCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUCGBFIS` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status"]
pub struct TXUCGBFIS_R(crate::FieldReader<bool, bool>);
impl TXUCGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUCGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUCGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCGBFIS` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status"]
pub struct TXMCGBFIS_R(crate::FieldReader<bool, bool>);
impl TXMCGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBCGBFIS` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status"]
pub struct TXBCGBFIS_R(crate::FieldReader<bool, bool>);
impl TXBCGBFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBCGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBCGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUFLOWERFIS` reader - MMC Transmit Underflow Error Frame Counter Interrupt Status"]
pub struct TXUFLOWERFIS_R(crate::FieldReader<bool, bool>);
impl TXUFLOWERFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUFLOWERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUFLOWERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSCOLGFIS` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
pub struct TXSCOLGFIS_R(crate::FieldReader<bool, bool>);
impl TXSCOLGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSCOLGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSCOLGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMCOLGFIS` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
pub struct TXMCOLGFIS_R(crate::FieldReader<bool, bool>);
impl TXMCOLGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMCOLGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMCOLGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDEFFIS` reader - MMC Transmit Deferred Frame Counter Interrupt Status"]
pub struct TXDEFFIS_R(crate::FieldReader<bool, bool>);
impl TXDEFFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDEFFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDEFFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLATCOLFIS` reader - MMC Transmit Late Collision Frame Counter Interrupt Status"]
pub struct TXLATCOLFIS_R(crate::FieldReader<bool, bool>);
impl TXLATCOLFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXLATCOLFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLATCOLFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEXCOLFIS` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Status"]
pub struct TXEXCOLFIS_R(crate::FieldReader<bool, bool>);
impl TXEXCOLFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEXCOLFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEXCOLFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCARERFIS` reader - MMC Transmit Carrier Error Frame Counter Interrupt Status"]
pub struct TXCARERFIS_R(crate::FieldReader<bool, bool>);
impl TXCARERFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCARERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCARERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGOCTIS` reader - MMC Transmit Good Octet Counter Interrupt Status"]
pub struct TXGOCTIS_R(crate::FieldReader<bool, bool>);
impl TXGOCTIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGOCTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGOCTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXGFRMIS` reader - MMC Transmit Good Frame Counter Interrupt Status"]
pub struct TXGFRMIS_R(crate::FieldReader<bool, bool>);
impl TXGFRMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXGFRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXGFRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEXDEFFIS` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Status"]
pub struct TXEXDEFFIS_R(crate::FieldReader<bool, bool>);
impl TXEXDEFFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEXDEFFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEXDEFFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPAUSFIS` reader - MMC Transmit Pause Frame Counter Interrupt Status"]
pub struct TXPAUSFIS_R(crate::FieldReader<bool, bool>);
impl TXPAUSFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPAUSFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPAUSFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXVLANGFIS` reader - MMC Transmit VLAN Good Frame Counter Interrupt Status"]
pub struct TXVLANGFIS_R(crate::FieldReader<bool, bool>);
impl TXVLANGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXVLANGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXVLANGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOSIZEGFIS` reader - MMC Transmit Oversize Good Frame Counter Interrupt Status"]
pub struct TXOSIZEGFIS_R(crate::FieldReader<bool, bool>);
impl TXOSIZEGFIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOSIZEGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOSIZEGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgboctis(&self) -> TXGBOCTIS_R {
        TXGBOCTIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgbfrmis(&self) -> TXGBFRMIS_R {
        TXGBFRMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgfis(&self) -> TXBCGFIS_R {
        TXBCGFIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgfis(&self) -> TXMCGFIS_R {
        TXMCGFIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn tx64octgbfis(&self) -> TX64OCTGBFIS_R {
        TX64OCTGBFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx65t127octgbfis(&self) -> TX65T127OCTGBFIS_R {
        TX65T127OCTGBFIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx128t255octgbfis(&self) -> TX128T255OCTGBFIS_R {
        TX128T255OCTGBFIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx256t511octgbfis(&self) -> TX256T511OCTGBFIS_R {
        TX256T511OCTGBFIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx512t1023octgbfis(&self) -> TX512T1023OCTGBFIS_R {
        TX512T1023OCTGBFIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfis(&self) -> TX1024TMAXOCTGBFIS_R {
        TX1024TMAXOCTGBFIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txucgbfis(&self) -> TXUCGBFIS_R {
        TXUCGBFIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcgbfis(&self) -> TXMCGBFIS_R {
        TXMCGBFIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txbcgbfis(&self) -> TXBCGBFIS_R {
        TXBCGBFIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txuflowerfis(&self) -> TXUFLOWERFIS_R {
        TXUFLOWERFIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txscolgfis(&self) -> TXSCOLGFIS_R {
        TXSCOLGFIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcolgfis(&self) -> TXMCOLGFIS_R {
        TXMCOLGFIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txdeffis(&self) -> TXDEFFIS_R {
        TXDEFFIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txlatcolfis(&self) -> TXLATCOLFIS_R {
        TXLATCOLFIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexcolfis(&self) -> TXEXCOLFIS_R {
        TXEXCOLFIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txcarerfis(&self) -> TXCARERFIS_R {
        TXCARERFIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgoctis(&self) -> TXGOCTIS_R {
        TXGOCTIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgfrmis(&self) -> TXGFRMIS_R {
        TXGFRMIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txexdeffis(&self) -> TXEXDEFFIS_R {
        TXEXDEFFIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txpausfis(&self) -> TXPAUSFIS_R {
        TXPAUSFIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txvlangfis(&self) -> TXVLANGFIS_R {
        TXVLANGFIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn txosizegfis(&self) -> TXOSIZEGFIS_R {
        TXOSIZEGFIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
#[doc = "MMC Transmit Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_transmit_interrupt](index.html) module"]
pub struct MMC_TRANSMIT_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_TRANSMIT_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_transmit_interrupt::R](R) reader structure"]
impl crate::Readable for MMC_TRANSMIT_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMC_TRANSMIT_INTERRUPT to value 0"]
impl crate::Resettable for MMC_TRANSMIT_INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
