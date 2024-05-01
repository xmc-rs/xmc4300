#[doc = "Register `SM_PDI_CTR` reader"]
pub type R = crate::R<SmPdiCtrSpec>;
#[doc = "Register `SM_PDI_CTR` writer"]
pub type W = crate::W<SmPdiCtrSpec>;
#[doc = "Deactivate SyncManager\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Deact {
    #[doc = "0: Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    Value1 = 0,
    #[doc = "1: Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    Value2 = 1,
}
impl From<Deact> for bool {
    #[inline(always)]
    fn from(variant: Deact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEACT` reader - Deactivate SyncManager"]
pub type DeactR = crate::BitReader<Deact>;
impl DeactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Deact {
        match self.bits {
            false => Deact::Value1,
            true => Deact::Value2,
        }
    }
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Deact::Value1
    }
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Deact::Value2
    }
}
#[doc = "Field `DEACT` writer - Deactivate SyncManager"]
pub type DeactW<'a, REG> = crate::BitWriter<'a, REG, Deact>;
impl<'a, REG> DeactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read 0 for Normal operation, SyncManager activated, write 0 for Activate SyncManager"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Deact::Value1)
    }
    #[doc = "Read 1 for SyncManager deactivated and reset SyncManager locks access to Memory area, write 1 for Request SyncManager deactivation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Deact::Value2)
    }
}
#[doc = "Field `REP_ACK` reader - Repeat Ack"]
pub type RepAckR = crate::BitReader;
#[doc = "Field `REP_ACK` writer - Repeat Ack"]
pub type RepAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    pub fn deact(&self) -> DeactR {
        DeactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    pub fn rep_ack(&self) -> RepAckR {
        RepAckR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Deactivate SyncManager"]
    #[inline(always)]
    #[must_use]
    pub fn deact(&mut self) -> DeactW<SmPdiCtrSpec> {
        DeactW::new(self, 0)
    }
    #[doc = "Bit 1 - Repeat Ack"]
    #[inline(always)]
    #[must_use]
    pub fn rep_ack(&mut self) -> RepAckW<SmPdiCtrSpec> {
        RepAckW::new(self, 1)
    }
}
#[doc = "PDI Control SyncManager 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sm_pdi_ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_pdi_ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmPdiCtrSpec;
impl crate::RegisterSpec for SmPdiCtrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sm_pdi_ctr::R`](R) reader structure"]
impl crate::Readable for SmPdiCtrSpec {}
#[doc = "`write(|w| ..)` method takes [`sm_pdi_ctr::W`](W) writer structure"]
impl crate::Writable for SmPdiCtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SM_PDI_CTR to value 0"]
impl crate::Resettable for SmPdiCtrSpec {
    const RESET_VALUE: u8 = 0;
}
