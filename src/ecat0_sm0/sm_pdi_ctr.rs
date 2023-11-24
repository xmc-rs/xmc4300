#[doc = "Register `SM_PDI_CTR` reader"]
pub type R = crate::R<SM_PDI_CTR_SPEC>;
#[doc = "Register `SM_PDI_CTR` writer"]
pub type W = crate::W<SM_PDI_CTR_SPEC>;
#[doc = "Field `DEACT` reader - Deactivate SyncManager"]
pub type DEACT_R = crate::BitReader<DEACT_A>;
#[doc = "Deactivate SyncManager\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEACT_A {
    #[doc = "0: Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    VALUE1 = 0,
    #[doc = "1: Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    VALUE2 = 1,
}
impl From<DEACT_A> for bool {
    #[inline(always)]
    fn from(variant: DEACT_A) -> Self {
        variant as u8 != 0
    }
}
impl DEACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEACT_A {
        match self.bits {
            false => DEACT_A::VALUE1,
            true => DEACT_A::VALUE2,
        }
    }
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEACT_A::VALUE1
    }
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DEACT_A::VALUE2
    }
}
#[doc = "Field `DEACT` writer - Deactivate SyncManager"]
pub type DEACT_W<'a, REG> = crate::BitWriter<'a, REG, DEACT_A>;
impl<'a, REG> DEACT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DEACT_A::VALUE1)
    }
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DEACT_A::VALUE2)
    }
}
#[doc = "Field `REP_ACK` reader - Repeat Ack"]
pub type REP_ACK_R = crate::BitReader;
#[doc = "Field `REP_ACK` writer - Repeat Ack"]
pub type REP_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    pub fn rep_ack(&self) -> REP_ACK_R {
        REP_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    #[must_use]
    pub fn deact(&mut self) -> DEACT_W<SM_PDI_CTR_SPEC> {
        DEACT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    #[must_use]
    pub fn rep_ack(&mut self) -> REP_ACK_W<SM_PDI_CTR_SPEC> {
        REP_ACK_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PDI Control SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_pdi_ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_pdi_ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_PDI_CTR_SPEC;
impl crate::RegisterSpec for SM_PDI_CTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sm_pdi_ctr::R`](R) reader structure"]
impl crate::Readable for SM_PDI_CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sm_pdi_ctr::W`](W) writer structure"]
impl crate::Writable for SM_PDI_CTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SM_PDI_CTR to value 0"]
impl crate::Resettable for SM_PDI_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
