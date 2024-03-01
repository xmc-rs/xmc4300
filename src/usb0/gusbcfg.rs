#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GusbcfgSpec>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GusbcfgSpec>;
#[doc = "Field `TOutCal` reader - FS Timeout Calibration"]
pub type ToutCalR = crate::FieldReader;
#[doc = "Field `TOutCal` writer - FS Timeout Calibration"]
pub type ToutCalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "USB 1.1 Full-Speed Serial Transceiver Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Physel {
    #[doc = "1: USB 1.1 full-speed serial transceiver"]
    Value2 = 1,
}
impl From<Physel> for bool {
    #[inline(always)]
    fn from(variant: Physel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYSel` reader - USB 1.1 Full-Speed Serial Transceiver Select"]
pub type PhyselR = crate::BitReader<Physel>;
impl PhyselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Physel> {
        match self.bits {
            true => Some(Physel::Value2),
            _ => None,
        }
    }
    #[doc = "USB 1.1 full-speed serial transceiver"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Physel::Value2
    }
}
#[doc = "SRP-Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srpcap {
    #[doc = "0: SRP capability is not enabled."]
    Value1 = 0,
    #[doc = "1: SRP capability is enabled."]
    Value2 = 1,
}
impl From<Srpcap> for bool {
    #[inline(always)]
    fn from(variant: Srpcap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRPCap` reader - SRP-Capable"]
pub type SrpcapR = crate::BitReader<Srpcap>;
impl SrpcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srpcap {
        match self.bits {
            false => Srpcap::Value1,
            true => Srpcap::Value2,
        }
    }
    #[doc = "SRP capability is not enabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srpcap::Value1
    }
    #[doc = "SRP capability is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srpcap::Value2
    }
}
#[doc = "Field `SRPCap` writer - SRP-Capable"]
pub type SrpcapW<'a, REG> = crate::BitWriter<'a, REG, Srpcap>;
impl<'a, REG> SrpcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRP capability is not enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srpcap::Value1)
    }
    #[doc = "SRP capability is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srpcap::Value2)
    }
}
#[doc = "HNP-Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hnpcap {
    #[doc = "0: HNP capability is not enabled."]
    Value1 = 0,
    #[doc = "1: HNP capability is enabled."]
    Value2 = 1,
}
impl From<Hnpcap> for bool {
    #[inline(always)]
    fn from(variant: Hnpcap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HNPCap` reader - HNP-Capable"]
pub type HnpcapR = crate::BitReader<Hnpcap>;
impl HnpcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hnpcap {
        match self.bits {
            false => Hnpcap::Value1,
            true => Hnpcap::Value2,
        }
    }
    #[doc = "HNP capability is not enabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hnpcap::Value1
    }
    #[doc = "HNP capability is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hnpcap::Value2
    }
}
#[doc = "Field `HNPCap` writer - HNP-Capable"]
pub type HnpcapW<'a, REG> = crate::BitWriter<'a, REG, Hnpcap>;
impl<'a, REG> HnpcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HNP capability is not enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hnpcap::Value1)
    }
    #[doc = "HNP capability is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hnpcap::Value2)
    }
}
#[doc = "Field `USBTrdTim` reader - USB Turnaround Time"]
pub type UsbtrdTimR = crate::FieldReader;
#[doc = "Field `USBTrdTim` writer - USB Turnaround Time"]
pub type UsbtrdTimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "UTMIFS Interface Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtgI2csel {
    #[doc = "0: UTMI USB 1.1 Full-Speed interface for OTG signals"]
    Value1 = 0,
}
impl From<OtgI2csel> for bool {
    #[inline(always)]
    fn from(variant: OtgI2csel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OtgI2CSel` reader - UTMIFS Interface Select"]
pub type OtgI2cselR = crate::BitReader<OtgI2csel>;
impl OtgI2cselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OtgI2csel> {
        match self.bits {
            false => Some(OtgI2csel::Value1),
            _ => None,
        }
    }
    #[doc = "UTMI USB 1.1 Full-Speed interface for OTG signals"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OtgI2csel::Value1
    }
}
#[doc = "Field `OtgI2CSel` writer - UTMIFS Interface Select"]
pub type OtgI2cselW<'a, REG> = crate::BitWriter<'a, REG, OtgI2csel>;
impl<'a, REG> OtgI2cselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UTMI USB 1.1 Full-Speed interface for OTG signals"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OtgI2csel::Value1)
    }
}
#[doc = "Tx End Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEndDelay {
    #[doc = "0: Normal mode"]
    Value1 = 0,
    #[doc = "1: Introduce Tx end delay timers"]
    Value2 = 1,
}
impl From<TxEndDelay> for bool {
    #[inline(always)]
    fn from(variant: TxEndDelay) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxEndDelay` reader - Tx End Delay"]
pub type TxEndDelayR = crate::BitReader<TxEndDelay>;
impl TxEndDelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxEndDelay {
        match self.bits {
            false => TxEndDelay::Value1,
            true => TxEndDelay::Value2,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxEndDelay::Value1
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxEndDelay::Value2
    }
}
#[doc = "Field `TxEndDelay` writer - Tx End Delay"]
pub type TxEndDelayW<'a, REG> = crate::BitWriter<'a, REG, TxEndDelay>;
impl<'a, REG> TxEndDelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxEndDelay::Value1)
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxEndDelay::Value2)
    }
}
#[doc = "Force Host Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceHstMode {
    #[doc = "0: Normal Mode"]
    Value1 = 0,
    #[doc = "1: Force Host Mode"]
    Value2 = 1,
}
impl From<ForceHstMode> for bool {
    #[inline(always)]
    fn from(variant: ForceHstMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForceHstMode` reader - Force Host Mode"]
pub type ForceHstModeR = crate::BitReader<ForceHstMode>;
impl ForceHstModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceHstMode {
        match self.bits {
            false => ForceHstMode::Value1,
            true => ForceHstMode::Value2,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ForceHstMode::Value1
    }
    #[doc = "Force Host Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ForceHstMode::Value2
    }
}
#[doc = "Field `ForceHstMode` writer - Force Host Mode"]
pub type ForceHstModeW<'a, REG> = crate::BitWriter<'a, REG, ForceHstMode>;
impl<'a, REG> ForceHstModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceHstMode::Value1)
    }
    #[doc = "Force Host Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ForceHstMode::Value2)
    }
}
#[doc = "Force Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceDevMode {
    #[doc = "0: Normal Mode"]
    Value1 = 0,
    #[doc = "1: Force Device Mode"]
    Value2 = 1,
}
impl From<ForceDevMode> for bool {
    #[inline(always)]
    fn from(variant: ForceDevMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForceDevMode` reader - Force Device Mode"]
pub type ForceDevModeR = crate::BitReader<ForceDevMode>;
impl ForceDevModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceDevMode {
        match self.bits {
            false => ForceDevMode::Value1,
            true => ForceDevMode::Value2,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ForceDevMode::Value1
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ForceDevMode::Value2
    }
}
#[doc = "Field `ForceDevMode` writer - Force Device Mode"]
pub type ForceDevModeW<'a, REG> = crate::BitWriter<'a, REG, ForceDevMode>;
impl<'a, REG> ForceDevModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDevMode::Value1)
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDevMode::Value2)
    }
}
#[doc = "Field `CTP` reader - Corrupt Tx packet"]
pub type CtpR = crate::BitReader;
#[doc = "Field `CTP` writer - Corrupt Tx packet"]
pub type CtpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&self) -> ToutCalR {
        ToutCalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&self) -> PhyselR {
        PhyselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SrpcapR {
        SrpcapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HnpcapR {
        HnpcapR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&self) -> UsbtrdTimR {
        UsbtrdTimR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - UTMIFS Interface Select"]
    #[inline(always)]
    pub fn otg_i2csel(&self) -> OtgI2cselR {
        OtgI2cselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&self) -> TxEndDelayR {
        TxEndDelayR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    pub fn force_hst_mode(&self) -> ForceHstModeR {
        ForceHstModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&self) -> ForceDevModeR {
        ForceDevModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&self) -> CtpR {
        CtpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cal(&mut self) -> ToutCalW<GusbcfgSpec> {
        ToutCalW::new(self, 0)
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SrpcapW<GusbcfgSpec> {
        SrpcapW::new(self, 8)
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HnpcapW<GusbcfgSpec> {
        HnpcapW::new(self, 9)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrd_tim(&mut self) -> UsbtrdTimW<GusbcfgSpec> {
        UsbtrdTimW::new(self, 10)
    }
    #[doc = "Bit 16 - UTMIFS Interface Select"]
    #[inline(always)]
    #[must_use]
    pub fn otg_i2csel(&mut self) -> OtgI2cselW<GusbcfgSpec> {
        OtgI2cselW::new(self, 16)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tx_end_delay(&mut self) -> TxEndDelayW<GusbcfgSpec> {
        TxEndDelayW::new(self, 28)
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    #[must_use]
    pub fn force_hst_mode(&mut self) -> ForceHstModeW<GusbcfgSpec> {
        ForceHstModeW::new(self, 29)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn force_dev_mode(&mut self) -> ForceDevModeW<GusbcfgSpec> {
        ForceDevModeW::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CtpW<GusbcfgSpec> {
        CtpW::new(self, 31)
    }
}
#[doc = "USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GusbcfgSpec;
impl crate::RegisterSpec for GusbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GusbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GusbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GusbcfgSpec {
    const RESET_VALUE: u32 = 0x1440;
}
