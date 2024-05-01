#[doc = "Register `PMT_CONTROL_STATUS` reader"]
pub type R = crate::R<PMT_CONTROL_STATUS_SPEC>;
#[doc = "Register `PMT_CONTROL_STATUS` writer"]
pub type W = crate::W<PMT_CONTROL_STATUS_SPEC>;
#[doc = "Field `PWRDWN` reader - Power Down"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power Down"]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable"]
pub type MGKPKTEN_R = crate::BitReader;
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable"]
pub type MGKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPKTEN` reader - Wake-Up Frame Enable"]
pub type RWKPKTEN_R = crate::BitReader;
#[doc = "Field `RWKPKTEN` writer - Wake-Up Frame Enable"]
pub type RWKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received"]
pub type MGKPRCVD_R = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - Wake-Up Frame Received"]
pub type RWKPRCVD_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - Global Unicast"]
pub type GLBLUCAST_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` writer - Global Unicast"]
pub type GLBLUCAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKFILTRST` reader - Wake-Up Frame Filter Register Pointer Reset"]
pub type RWKFILTRST_R = crate::BitReader;
#[doc = "Field `RWKFILTRST` writer - Wake-Up Frame Filter Register Pointer Reset"]
pub type RWKFILTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Frame Received"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<PMT_CONTROL_STATUS_SPEC> {
        PWRDWN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<PMT_CONTROL_STATUS_SPEC> {
        MGKPKTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<PMT_CONTROL_STATUS_SPEC> {
        RWKPKTEN_W::new(self, 2)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<PMT_CONTROL_STATUS_SPEC> {
        GLBLUCAST_W::new(self, 9)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<PMT_CONTROL_STATUS_SPEC> {
        RWKFILTRST_W::new(self, 31)
    }
}
#[doc = "PMT Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_control_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmt_control_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMT_CONTROL_STATUS_SPEC;
impl crate::RegisterSpec for PMT_CONTROL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmt_control_status::R`](R) reader structure"]
impl crate::Readable for PMT_CONTROL_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmt_control_status::W`](W) writer structure"]
impl crate::Writable for PMT_CONTROL_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMT_CONTROL_STATUS to value 0"]
impl crate::Resettable for PMT_CONTROL_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
