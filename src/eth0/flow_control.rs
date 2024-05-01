#[doc = "Register `FLOW_CONTROL` reader"]
pub type R = crate::R<FlowControlSpec>;
#[doc = "Register `FLOW_CONTROL` writer"]
pub type W = crate::W<FlowControlSpec>;
#[doc = "Field `FCA_BPA` reader - Flow Control Busy or Backpressure Activate"]
pub type FcaBpaR = crate::BitReader;
#[doc = "Field `FCA_BPA` writer - Flow Control Busy or Backpressure Activate"]
pub type FcaBpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable"]
pub type RfeR = crate::BitReader;
#[doc = "Field `RFE` writer - Receive Flow Control Enable"]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect"]
pub type UpR = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect"]
pub type UpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PltR = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause"]
pub type DzpqR = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause"]
pub type DzpqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause Time"]
pub type PtR = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause Time"]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub fn fca_bpa(&self) -> FcaBpaR {
        FcaBpaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PltR {
        PltR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DzpqR {
        DzpqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    #[must_use]
    pub fn fca_bpa(&mut self) -> FcaBpaW<FlowControlSpec> {
        FcaBpaW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<FlowControlSpec> {
        TfeW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<FlowControlSpec> {
        RfeW::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UpW<FlowControlSpec> {
        UpW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PltW<FlowControlSpec> {
        PltW::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DzpqW<FlowControlSpec> {
        DzpqW::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<FlowControlSpec> {
        PtW::new(self, 16)
    }
}
#[doc = "Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flow_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flow_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlowControlSpec;
impl crate::RegisterSpec for FlowControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flow_control::R`](R) reader structure"]
impl crate::Readable for FlowControlSpec {}
#[doc = "`write(|w| ..)` method takes [`flow_control::W`](W) writer structure"]
impl crate::Writable for FlowControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLOW_CONTROL to value 0"]
impl crate::Resettable for FlowControlSpec {
    const RESET_VALUE: u32 = 0;
}
