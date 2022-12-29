#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub struct R(crate::R<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub struct W(crate::W<INTERRUPT_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_ENABLE_SPEC>;
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
impl From<crate::W<INTERRUPT_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `TSE` reader - Transmit Stopped Enable"]
pub type TSE_R = crate::BitReader<bool>;
#[doc = "Field `TSE` writer - Transmit Stopped Enable"]
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `TUE` reader - Transmit Buffer Unvailable Enable"]
pub type TUE_R = crate::BitReader<bool>;
#[doc = "Field `TUE` writer - Transmit Buffer Unvailable Enable"]
pub type TUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `TJE` reader - Transmit Jabber Timeout Enable"]
pub type TJE_R = crate::BitReader<bool>;
#[doc = "Field `TJE` writer - Transmit Jabber Timeout Enable"]
pub type TJE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `OVE` reader - Overflow Interrupt Enable"]
pub type OVE_R = crate::BitReader<bool>;
#[doc = "Field `OVE` writer - Overflow Interrupt Enable"]
pub type OVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `UNE` reader - Underflow Interrupt Enable"]
pub type UNE_R = crate::BitReader<bool>;
#[doc = "Field `UNE` writer - Underflow Interrupt Enable"]
pub type UNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable"]
pub type RUE_R = crate::BitReader<bool>;
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable"]
pub type RUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub type RSE_R = crate::BitReader<bool>;
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub type RSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable"]
pub type RWE_R = crate::BitReader<bool>;
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable"]
pub type RWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable"]
pub type ETE_R = crate::BitReader<bool>;
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable"]
pub type ETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable"]
pub type FBE_R = crate::BitReader<bool>;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable"]
pub type FBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable"]
pub type ERE_R = crate::BitReader<bool>;
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable"]
pub type ERE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub type AIE_R = crate::BitReader<bool>;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub type AIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub type NIE_R = crate::BitReader<bool>;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub type NIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTERRUPT_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    pub fn tue(&self) -> TUE_R {
        TUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    pub fn tje(&self) -> TJE_R {
        TJE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ove(&self) -> OVE_R {
        OVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn une(&self) -> UNE_R {
        UNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn ete(&self) -> ETE_R {
        ETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn ere(&self) -> ERE_R {
        ERE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<1> {
        TSE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unvailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tue(&mut self) -> TUE_W<2> {
        TUE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tje(&mut self) -> TJE_W<3> {
        TJE_W::new(self)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OVE_W<4> {
        OVE_W::new(self)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn une(&mut self) -> UNE_W<5> {
        UNE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RUE_W<7> {
        RUE_W::new(self)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<8> {
        RSE_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwe(&mut self) -> RWE_W<9> {
        RWE_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ete(&mut self) -> ETE_W<10> {
        ETE_W::new(self)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<13> {
        FBE_W::new(self)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ere(&mut self) -> ERE_W<14> {
        ERE_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<15> {
        AIE_W::new(self)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<16> {
        NIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_enable](index.html) module"]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_enable::R](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_enable::W](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
