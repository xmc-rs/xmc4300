#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOutCal` reader - FS Timeout Calibration"]
pub type TOUT_CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOutCal` writer - FS Timeout Calibration"]
pub type TOUT_CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PHYSel` reader - USB 1.1 Full-Speed Serial Transceiver Select"]
pub type PHYSEL_R = crate::BitReader<PHYSEL_A>;
#[doc = "USB 1.1 Full-Speed Serial Transceiver Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYSEL_A {
    #[doc = "1: USB 1.1 full-speed serial transceiver"]
    VALUE2 = 1,
}
impl From<PHYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PHYSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PHYSEL_A> {
        match self.bits {
            true => Some(PHYSEL_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHYSEL_A::VALUE2
    }
}
#[doc = "Field `SRPCap` reader - SRP-Capable"]
pub type SRPCAP_R = crate::BitReader<SRPCAP_A>;
#[doc = "SRP-Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRPCAP_A {
    #[doc = "0: SRP capability is not enabled."]
    VALUE1 = 0,
    #[doc = "1: SRP capability is enabled."]
    VALUE2 = 1,
}
impl From<SRPCAP_A> for bool {
    #[inline(always)]
    fn from(variant: SRPCAP_A) -> Self {
        variant as u8 != 0
    }
}
impl SRPCAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRPCAP_A {
        match self.bits {
            false => SRPCAP_A::VALUE1,
            true => SRPCAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRPCAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRPCAP_A::VALUE2
    }
}
#[doc = "Field `SRPCap` writer - SRP-Capable"]
pub type SRPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, SRPCAP_A, O>;
impl<'a, const O: u8> SRPCAP_W<'a, O> {
    #[doc = "SRP capability is not enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRPCAP_A::VALUE1)
    }
    #[doc = "SRP capability is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRPCAP_A::VALUE2)
    }
}
#[doc = "Field `HNPCap` reader - HNP-Capable"]
pub type HNPCAP_R = crate::BitReader<HNPCAP_A>;
#[doc = "HNP-Capable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HNPCAP_A {
    #[doc = "0: HNP capability is not enabled."]
    VALUE1 = 0,
    #[doc = "1: HNP capability is enabled."]
    VALUE2 = 1,
}
impl From<HNPCAP_A> for bool {
    #[inline(always)]
    fn from(variant: HNPCAP_A) -> Self {
        variant as u8 != 0
    }
}
impl HNPCAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNPCAP_A {
        match self.bits {
            false => HNPCAP_A::VALUE1,
            true => HNPCAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HNPCAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HNPCAP_A::VALUE2
    }
}
#[doc = "Field `HNPCap` writer - HNP-Capable"]
pub type HNPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, HNPCAP_A, O>;
impl<'a, const O: u8> HNPCAP_W<'a, O> {
    #[doc = "HNP capability is not enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HNPCAP_A::VALUE1)
    }
    #[doc = "HNP capability is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HNPCAP_A::VALUE2)
    }
}
#[doc = "Field `USBTrdTim` reader - USB Turnaround Time"]
pub type USBTRD_TIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBTrdTim` writer - USB Turnaround Time"]
pub type USBTRD_TIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `OtgI2CSel` reader - UTMIFS Interface Select"]
pub type OTG_I2CSEL_R = crate::BitReader<OTG_I2CSEL_A>;
#[doc = "UTMIFS Interface Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTG_I2CSEL_A {
    #[doc = "0: UTMI USB 1.1 Full-Speed interface for OTG signals"]
    VALUE1 = 0,
}
impl From<OTG_I2CSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OTG_I2CSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl OTG_I2CSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OTG_I2CSEL_A> {
        match self.bits {
            false => Some(OTG_I2CSEL_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OTG_I2CSEL_A::VALUE1
    }
}
#[doc = "Field `OtgI2CSel` writer - UTMIFS Interface Select"]
pub type OTG_I2CSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, OTG_I2CSEL_A, O>;
impl<'a, const O: u8> OTG_I2CSEL_W<'a, O> {
    #[doc = "UTMI USB 1.1 Full-Speed interface for OTG signals"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OTG_I2CSEL_A::VALUE1)
    }
}
#[doc = "Field `TxEndDelay` reader - Tx End Delay"]
pub type TX_END_DELAY_R = crate::BitReader<TX_END_DELAY_A>;
#[doc = "Tx End Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_END_DELAY_A {
    #[doc = "0: Normal mode"]
    VALUE1 = 0,
    #[doc = "1: Introduce Tx end delay timers"]
    VALUE2 = 1,
}
impl From<TX_END_DELAY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_END_DELAY_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_END_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_END_DELAY_A {
        match self.bits {
            false => TX_END_DELAY_A::VALUE1,
            true => TX_END_DELAY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_END_DELAY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_END_DELAY_A::VALUE2
    }
}
#[doc = "Field `TxEndDelay` writer - Tx End Delay"]
pub type TX_END_DELAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, TX_END_DELAY_A, O>;
impl<'a, const O: u8> TX_END_DELAY_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TX_END_DELAY_A::VALUE1)
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TX_END_DELAY_A::VALUE2)
    }
}
#[doc = "Field `ForceHstMode` reader - Force Host Mode"]
pub type FORCE_HST_MODE_R = crate::BitReader<FORCE_HST_MODE_A>;
#[doc = "Force Host Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_HST_MODE_A {
    #[doc = "0: Normal Mode"]
    VALUE1 = 0,
    #[doc = "1: Force Host Mode"]
    VALUE2 = 1,
}
impl From<FORCE_HST_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_HST_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_HST_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_HST_MODE_A {
        match self.bits {
            false => FORCE_HST_MODE_A::VALUE1,
            true => FORCE_HST_MODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_HST_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_HST_MODE_A::VALUE2
    }
}
#[doc = "Field `ForceHstMode` writer - Force Host Mode"]
pub type FORCE_HST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, FORCE_HST_MODE_A, O>;
impl<'a, const O: u8> FORCE_HST_MODE_W<'a, O> {
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCE_HST_MODE_A::VALUE1)
    }
    #[doc = "Force Host Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCE_HST_MODE_A::VALUE2)
    }
}
#[doc = "Field `ForceDevMode` reader - Force Device Mode"]
pub type FORCE_DEV_MODE_R = crate::BitReader<FORCE_DEV_MODE_A>;
#[doc = "Force Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_DEV_MODE_A {
    #[doc = "0: Normal Mode"]
    VALUE1 = 0,
    #[doc = "1: Force Device Mode"]
    VALUE2 = 1,
}
impl From<FORCE_DEV_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_DEV_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_DEV_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_DEV_MODE_A {
        match self.bits {
            false => FORCE_DEV_MODE_A::VALUE1,
            true => FORCE_DEV_MODE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_DEV_MODE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_DEV_MODE_A::VALUE2
    }
}
#[doc = "Field `ForceDevMode` writer - Force Device Mode"]
pub type FORCE_DEV_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, FORCE_DEV_MODE_A, O>;
impl<'a, const O: u8> FORCE_DEV_MODE_W<'a, O> {
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FORCE_DEV_MODE_A::VALUE1)
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FORCE_DEV_MODE_A::VALUE2)
    }
}
#[doc = "Field `CTP` reader - Corrupt Tx packet"]
pub type CTP_R = crate::BitReader<bool>;
#[doc = "Field `CTP` writer - Corrupt Tx packet"]
pub type CTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&self) -> TOUT_CAL_R {
        TOUT_CAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&self) -> USBTRD_TIM_R {
        USBTRD_TIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - UTMIFS Interface Select"]
    #[inline(always)]
    pub fn otg_i2csel(&self) -> OTG_I2CSEL_R {
        OTG_I2CSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&self) -> TX_END_DELAY_R {
        TX_END_DELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    pub fn force_hst_mode(&self) -> FORCE_HST_MODE_R {
        FORCE_HST_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&self) -> FORCE_DEV_MODE_R {
        FORCE_DEV_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cal(&mut self) -> TOUT_CAL_W<0> {
        TOUT_CAL_W::new(self)
    }
    #[doc = "Bit 8 - SRP-Capable"]
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<8> {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 9 - HNP-Capable"]
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<9> {
        HNPCAP_W::new(self)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrd_tim(&mut self) -> USBTRD_TIM_W<10> {
        USBTRD_TIM_W::new(self)
    }
    #[doc = "Bit 16 - UTMIFS Interface Select"]
    #[inline(always)]
    #[must_use]
    pub fn otg_i2csel(&mut self) -> OTG_I2CSEL_W<16> {
        OTG_I2CSEL_W::new(self)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tx_end_delay(&mut self) -> TX_END_DELAY_W<28> {
        TX_END_DELAY_W::new(self)
    }
    #[doc = "Bit 29 - Force Host Mode"]
    #[inline(always)]
    #[must_use]
    pub fn force_hst_mode(&mut self) -> FORCE_HST_MODE_W<29> {
        FORCE_HST_MODE_W::new(self)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn force_dev_mode(&mut self) -> FORCE_DEV_MODE_W<30> {
        FORCE_DEV_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CTP_W<31> {
        CTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1440;
}
