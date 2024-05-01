#[doc = "Register `PMT_CONTROL_STATUS` reader"]
pub type R = crate::R<PmtControlStatusSpec>;
#[doc = "Register `PMT_CONTROL_STATUS` writer"]
pub type W = crate::W<PmtControlStatusSpec>;
#[doc = "Field `PWRDWN` reader - Power Down"]
pub type PwrdwnR = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power Down"]
pub type PwrdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable"]
pub type MgkpktenR = crate::BitReader;
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable"]
pub type MgkpktenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPKTEN` reader - Wake-Up Frame Enable"]
pub type RwkpktenR = crate::BitReader;
#[doc = "Field `RWKPKTEN` writer - Wake-Up Frame Enable"]
pub type RwkpktenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received"]
pub type MgkprcvdR = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - Wake-Up Frame Received"]
pub type RwkprcvdR = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - Global Unicast"]
pub type GlblucastR = crate::BitReader;
#[doc = "Field `GLBLUCAST` writer - Global Unicast"]
pub type GlblucastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKFILTRST` reader - Wake-Up Frame Filter Register Pointer Reset"]
pub type RwkfiltrstR = crate::BitReader;
#[doc = "Field `RWKFILTRST` writer - Wake-Up Frame Filter Register Pointer Reset"]
pub type RwkfiltrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PwrdwnR {
        PwrdwnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MgkpktenR {
        MgkpktenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RwkpktenR {
        RwkpktenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MgkprcvdR {
        MgkprcvdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RwkprcvdR {
        RwkprcvdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&self) -> GlblucastR {
        GlblucastR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RwkfiltrstR {
        RwkfiltrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PwrdwnW<PmtControlStatusSpec> {
        PwrdwnW::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MgkpktenW<PmtControlStatusSpec> {
        MgkpktenW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RwkpktenW<PmtControlStatusSpec> {
        RwkpktenW::new(self, 2)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GlblucastW<PmtControlStatusSpec> {
        GlblucastW::new(self, 9)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RwkfiltrstW<PmtControlStatusSpec> {
        RwkfiltrstW::new(self, 31)
    }
}
#[doc = "PMT Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_control_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmt_control_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmtControlStatusSpec;
impl crate::RegisterSpec for PmtControlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmt_control_status::R`](R) reader structure"]
impl crate::Readable for PmtControlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pmt_control_status::W`](W) writer structure"]
impl crate::Writable for PmtControlStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMT_CONTROL_STATUS to value 0"]
impl crate::Resettable for PmtControlStatusSpec {
    const RESET_VALUE: u32 = 0;
}
