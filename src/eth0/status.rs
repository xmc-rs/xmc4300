#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub type TI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub type TPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable"]
pub type TU_R = crate::BitReader;
#[doc = "Field `TU` writer - Transmit Buffer Unavailable"]
pub type TU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub type TJT_R = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit Jabber Timeout"]
pub type TJT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Receive Overflow"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Receive Overflow"]
pub type OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNF` reader - Transmit Underflow"]
pub type UNF_R = crate::BitReader;
#[doc = "Field `UNF` writer - Transmit Underflow"]
pub type UNF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt"]
pub type RI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RU` reader - Receive Buffer Unavailable"]
pub type RU_R = crate::BitReader;
#[doc = "Field `RU` writer - Receive Buffer Unavailable"]
pub type RU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub type RPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RWT_R = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub type RWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub type ETI_R = crate::BitReader;
#[doc = "Field `ETI` writer - Early Transmit Interrupt"]
pub type ETI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBI` reader - Fatal Bus Error Interrupt"]
pub type FBI_R = crate::BitReader;
#[doc = "Field `FBI` writer - Fatal Bus Error Interrupt"]
pub type FBI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub type ERI_R = crate::BitReader;
#[doc = "Field `ERI` writer - Early Receive Interrupt"]
pub type ERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Received Process State"]
pub type RS_R = crate::FieldReader;
#[doc = "Field `TS` reader - Transmit Process State"]
pub type TS_R = crate::FieldReader;
#[doc = "Field `EB` reader - Error Bits"]
pub type EB_R = crate::FieldReader;
#[doc = "Field `EMI` reader - ETH MMC Interrupt"]
pub type EMI_R = crate::BitReader;
#[doc = "Field `EPI` reader - ETH PMT Interrupt"]
pub type EPI_R = crate::BitReader;
#[doc = "Field `TTI` reader - Timestamp Trigger Interrupt"]
pub type TTI_R = crate::BitReader;
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
    pub fn ti(&mut self) -> TI_W<STATUS_SPEC> {
        TI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<STATUS_SPEC> {
        TPS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TU_W<STATUS_SPEC> {
        TU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TJT_W<STATUS_SPEC> {
        TJT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<STATUS_SPEC> {
        OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UNF_W<STATUS_SPEC> {
        UNF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<STATUS_SPEC> {
        RI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn ru(&mut self) -> RU_W<STATUS_SPEC> {
        RU_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<STATUS_SPEC> {
        RPS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<STATUS_SPEC> {
        RWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> ETI_W<STATUS_SPEC> {
        ETI_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fbi(&mut self) -> FBI_W<STATUS_SPEC> {
        FBI_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> ERI_W<STATUS_SPEC> {
        ERI_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<STATUS_SPEC> {
        AIS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<STATUS_SPEC> {
        NIS_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
