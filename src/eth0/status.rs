#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TI_R = crate::BitReader<bool>;
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub type TI_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TPS_R = crate::BitReader<bool>;
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub type TPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable"]
pub type TU_R = crate::BitReader<bool>;
#[doc = "Field `TU` writer - Transmit Buffer Unavailable"]
pub type TU_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub type TJT_R = crate::BitReader<bool>;
#[doc = "Field `TJT` writer - Transmit Jabber Timeout"]
pub type TJT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `OVF` reader - Receive Overflow"]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Receive Overflow"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `UNF` reader - Transmit Underflow"]
pub type UNF_R = crate::BitReader<bool>;
#[doc = "Field `UNF` writer - Transmit Underflow"]
pub type UNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - Receive Interrupt"]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RU` reader - Receive Buffer Unavailable"]
pub type RU_R = crate::BitReader<bool>;
#[doc = "Field `RU` writer - Receive Buffer Unavailable"]
pub type RU_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RPS_R = crate::BitReader<bool>;
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub type RPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RWT_R = crate::BitReader<bool>;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub type RWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub type ETI_R = crate::BitReader<bool>;
#[doc = "Field `ETI` writer - Early Transmit Interrupt"]
pub type ETI_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FBI` reader - Fatal Bus Error Interrupt"]
pub type FBI_R = crate::BitReader<bool>;
#[doc = "Field `FBI` writer - Fatal Bus Error Interrupt"]
pub type FBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub type ERI_R = crate::BitReader<bool>;
#[doc = "Field `ERI` writer - Early Receive Interrupt"]
pub type ERI_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RS` reader - Received Process State"]
pub type RS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS` reader - Transmit Process State"]
pub type TS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EB` reader - Error Bits"]
pub type EB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMI` reader - ETH MMC Interrupt"]
pub type EMI_R = crate::BitReader<bool>;
#[doc = "Field `EPI` reader - ETH PMT Interrupt"]
pub type EPI_R = crate::BitReader<bool>;
#[doc = "Field `TTI` reader - Timestamp Trigger Interrupt"]
pub type TTI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fbi(&self) -> FBI_R {
        FBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Received Process State"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - ETH MMC Interrupt"]
    #[inline(always)]
    pub fn emi(&self) -> EMI_R {
        EMI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ETH PMT Interrupt"]
    #[inline(always)]
    pub fn epi(&self) -> EPI_R {
        EPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TTI_R {
        TTI_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<0> {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<1> {
        TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TU_W<2> {
        TU_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TJT_W<3> {
        TJT_W::new(self)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<4> {
        OVF_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UNF_W<5> {
        UNF_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn ru(&mut self) -> RU_W<7> {
        RU_W::new(self)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<8> {
        RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<9> {
        RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> ETI_W<10> {
        ETI_W::new(self)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fbi(&mut self) -> FBI_W<13> {
        FBI_W::new(self)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> ERI_W<14> {
        ERI_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<15> {
        AIS_W::new(self)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<16> {
        NIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
