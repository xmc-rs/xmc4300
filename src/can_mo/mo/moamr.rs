#[doc = "Register `MOAMR` reader"]
pub type R = crate::R<MOAMR_SPEC>;
#[doc = "Register `MOAMR` writer"]
pub type W = crate::W<MOAMR_SPEC>;
#[doc = "Field `AM` reader - Acceptance Mask for Message Identifier"]
pub type AM_R = crate::FieldReader<u32>;
#[doc = "Field `AM` writer - Acceptance Mask for Message Identifier"]
pub type AM_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Acceptance Mask Bit for Message IDE Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIDE_A {
    #[doc = "0: Message object n accepts the reception of both, standard and extended frames."]
    VALUE1 = 0,
    #[doc = "1: Message object n receives frames only with matching IDE bit."]
    VALUE2 = 1,
}
impl From<MIDE_A> for bool {
    #[inline(always)]
    fn from(variant: MIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIDE` reader - Acceptance Mask Bit for Message IDE Bit"]
pub type MIDE_R = crate::BitReader<MIDE_A>;
impl MIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MIDE_A {
        match self.bits {
            false => MIDE_A::VALUE1,
            true => MIDE_A::VALUE2,
        }
    }
    #[doc = "Message object n accepts the reception of both, standard and extended frames."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIDE_A::VALUE1
    }
    #[doc = "Message object n receives frames only with matching IDE bit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIDE_A::VALUE2
    }
}
#[doc = "Field `MIDE` writer - Acceptance Mask Bit for Message IDE Bit"]
pub type MIDE_W<'a, REG> = crate::BitWriter<'a, REG, MIDE_A>;
impl<'a, REG> MIDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Message object n accepts the reception of both, standard and extended frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MIDE_A::VALUE1)
    }
    #[doc = "Message object n receives frames only with matching IDE bit."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MIDE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Acceptance Mask for Message Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<MOAMR_SPEC> {
        AM_W::new(self, 0)
    }
    #[doc = "Bit 29 - Acceptance Mask Bit for Message IDE Bit"]
    #[inline(always)]
    #[must_use]
    pub fn mide(&mut self) -> MIDE_W<MOAMR_SPEC> {
        MIDE_W::new(self, 29)
    }
}
#[doc = "Message Object Acceptance Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOAMR_SPEC;
impl crate::RegisterSpec for MOAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moamr::R`](R) reader structure"]
impl crate::Readable for MOAMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moamr::W`](W) writer structure"]
impl crate::Writable for MOAMR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MOAMR to value 0x3fff_ffff"]
impl crate::Resettable for MOAMR_SPEC {
    const RESET_VALUE: u32 = 0x3fff_ffff;
}
