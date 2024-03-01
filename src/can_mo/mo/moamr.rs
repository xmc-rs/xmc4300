#[doc = "Register `MOAMR` reader"]
pub type R = crate::R<MoamrSpec>;
#[doc = "Register `MOAMR` writer"]
pub type W = crate::W<MoamrSpec>;
#[doc = "Field `AM` reader - Acceptance Mask for Message Identifier"]
pub type AmR = crate::FieldReader<u32>;
#[doc = "Field `AM` writer - Acceptance Mask for Message Identifier"]
pub type AmW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Acceptance Mask Bit for Message IDE Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mide {
    #[doc = "0: Message object n accepts the reception of both, standard and extended frames."]
    Value1 = 0,
    #[doc = "1: Message object n receives frames only with matching IDE bit."]
    Value2 = 1,
}
impl From<Mide> for bool {
    #[inline(always)]
    fn from(variant: Mide) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIDE` reader - Acceptance Mask Bit for Message IDE Bit"]
pub type MideR = crate::BitReader<Mide>;
impl MideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mide {
        match self.bits {
            false => Mide::Value1,
            true => Mide::Value2,
        }
    }
    #[doc = "Message object n accepts the reception of both, standard and extended frames."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mide::Value1
    }
    #[doc = "Message object n receives frames only with matching IDE bit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mide::Value2
    }
}
#[doc = "Field `MIDE` writer - Acceptance Mask Bit for Message IDE Bit"]
pub type MideW<'a, REG> = crate::BitWriter<'a, REG, Mide>;
impl<'a, REG> MideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message object n accepts the reception of both, standard and extended frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mide::Value1)
    }
    #[doc = "Message object n receives frames only with matching IDE bit."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mide::Value2)
    }
}
impl R {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    pub fn am(&self) -> AmR {
        AmR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    pub fn mide(&self) -> MideR {
        MideR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AmW<MoamrSpec> {
        AmW::new(self, 0)
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mide(&mut self) -> MideW<MoamrSpec> {
        MideW::new(self, 29)
    }
}
#[doc = "Message Object Acceptance Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MoamrSpec;
impl crate::RegisterSpec for MoamrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moamr::R`](R) reader structure"]
impl crate::Readable for MoamrSpec {}
#[doc = "`write(|w| ..)` method takes [`moamr::W`](W) writer structure"]
impl crate::Writable for MoamrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOAMR to value 0x3fff_ffff"]
impl crate::Resettable for MoamrSpec {
    const RESET_VALUE: u32 = 0x3fff_ffff;
}
