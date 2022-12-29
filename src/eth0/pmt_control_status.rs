#[doc = "Register `PMT_CONTROL_STATUS` reader"]
pub struct R(crate::R<PMT_CONTROL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMT_CONTROL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMT_CONTROL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMT_CONTROL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMT_CONTROL_STATUS` writer"]
pub struct W(crate::W<PMT_CONTROL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMT_CONTROL_STATUS_SPEC>;
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
impl From<crate::W<PMT_CONTROL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMT_CONTROL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power Down"]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `PWRDWN` writer - Power Down"]
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CONTROL_STATUS_SPEC, bool, O>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable"]
pub type MGKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable"]
pub type MGKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CONTROL_STATUS_SPEC, bool, O>;
#[doc = "Field `RWKPKTEN` reader - Wake-Up Frame Enable"]
pub type RWKPKTEN_R = crate::BitReader<bool>;
#[doc = "Field `RWKPKTEN` writer - Wake-Up Frame Enable"]
pub type RWKPKTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CONTROL_STATUS_SPEC, bool, O>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received"]
pub type MGKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `RWKPRCVD` reader - Wake-Up Frame Received"]
pub type RWKPRCVD_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` reader - Global Unicast"]
pub type GLBLUCAST_R = crate::BitReader<bool>;
#[doc = "Field `GLBLUCAST` writer - Global Unicast"]
pub type GLBLUCAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CONTROL_STATUS_SPEC, bool, O>;
#[doc = "Field `RWKFILTRST` reader - Wake-Up Frame Filter Register Pointer Reset"]
pub type RWKFILTRST_R = crate::BitReader<bool>;
#[doc = "Field `RWKFILTRST` writer - Wake-Up Frame Filter Register Pointer Reset"]
pub type RWKFILTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMT_CONTROL_STATUS_SPEC, bool, O>;
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
    pub fn pwrdwn(&mut self) -> PWRDWN_W<0> {
        PWRDWN_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<1> {
        MGKPKTEN_W::new(self)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<2> {
        RWKPKTEN_W::new(self)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<9> {
        GLBLUCAST_W::new(self)
    }
    #[doc = "Bit 31 - Wake-Up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<31> {
        RWKFILTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMT Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmt_control_status](index.html) module"]
pub struct PMT_CONTROL_STATUS_SPEC;
impl crate::RegisterSpec for PMT_CONTROL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmt_control_status::R](R) reader structure"]
impl crate::Readable for PMT_CONTROL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmt_control_status::W](W) writer structure"]
impl crate::Writable for PMT_CONTROL_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMT_CONTROL_STATUS to value 0"]
impl crate::Resettable for PMT_CONTROL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
