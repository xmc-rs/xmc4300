#[doc = "Register `FLOW_CONTROL` reader"]
pub type R = crate::R<FLOW_CONTROL_SPEC>;
#[doc = "Register `FLOW_CONTROL` writer"]
pub type W = crate::W<FLOW_CONTROL_SPEC>;
#[doc = "Field `FCA_BPA` reader - Flow Control Busy or Backpressure Activate"]
pub type FCA_BPA_R = crate::BitReader;
#[doc = "Field `FCA_BPA` writer - Flow Control Busy or Backpressure Activate"]
pub type FCA_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable"]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable"]
pub type RFE_R = crate::BitReader;
#[doc = "Field `RFE` writer - Receive Flow Control Enable"]
pub type RFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect"]
pub type UP_R = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect"]
pub type UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause"]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause"]
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause Time"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause Time"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fca_bpa(&self) -> FCA_BPA_R {
        FCA_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fca_bpa(&mut self) -> FCA_BPA_W<FLOW_CONTROL_SPEC> {
        FCA_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<FLOW_CONTROL_SPEC> {
        TFE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W<FLOW_CONTROL_SPEC> {
        RFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W<FLOW_CONTROL_SPEC> {
        UP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<FLOW_CONTROL_SPEC> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W<FLOW_CONTROL_SPEC> {
        DZPQ_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<FLOW_CONTROL_SPEC> {
        PT_W::new(self, 16)
    }
}
#[doc = "Flow Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flow_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLOW_CONTROL_SPEC;
impl crate::RegisterSpec for FLOW_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flow_control::R`](R) reader structure"]
impl crate::Readable for FLOW_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flow_control::W`](W) writer structure"]
impl crate::Writable for FLOW_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLOW_CONTROL to value 0"]
impl crate::Resettable for FLOW_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
