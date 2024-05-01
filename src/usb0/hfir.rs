#[doc = "Register `HFIR` reader"]
pub type R = crate::R<HFIR_SPEC>;
#[doc = "Register `HFIR` writer"]
pub type W = crate::W<HFIR_SPEC>;
#[doc = "Field `FrInt` reader - Frame Interval"]
pub type FR_INT_R = crate::FieldReader<u16>;
#[doc = "Field `FrInt` writer - Frame Interval"]
pub type FR_INT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Reload Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFIRRLD_CTRL_A {
    #[doc = "0: HFIR cannot be reloaded dynamically"]
    VALUE1 = 0,
    #[doc = "1: HFIR can be dynamically reloaded during runtime"]
    VALUE2 = 1,
}
impl From<HFIRRLD_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: HFIRRLD_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFIRRldCtrl` reader - Reload Control"]
pub type HFIRRLD_CTRL_R = crate::BitReader<HFIRRLD_CTRL_A>;
impl HFIRRLD_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFIRRLD_CTRL_A {
        match self.bits {
            false => HFIRRLD_CTRL_A::VALUE1,
            true => HFIRRLD_CTRL_A::VALUE2,
        }
    }
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HFIRRLD_CTRL_A::VALUE1
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HFIRRLD_CTRL_A::VALUE2
    }
}
#[doc = "Field `HFIRRldCtrl` writer - Reload Control"]
pub type HFIRRLD_CTRL_W<'a, REG> = crate::BitWriter<'a, REG, HFIRRLD_CTRL_A>;
impl<'a, REG> HFIRRLD_CTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFIR cannot be reloaded dynamically"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HFIRRLD_CTRL_A::VALUE1)
    }
    #[doc = "HFIR can be dynamically reloaded during runtime"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HFIRRLD_CTRL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn fr_int(&self) -> FR_INT_R {
        FR_INT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrld_ctrl(&self) -> HFIRRLD_CTRL_R {
        HFIRRLD_CTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    #[must_use]
    pub fn fr_int(&mut self) -> FR_INT_W<HFIR_SPEC> {
        FR_INT_W::new(self, 0)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    #[must_use]
    pub fn hfirrld_ctrl(&mut self) -> HFIRRLD_CTRL_W<HFIR_SPEC> {
        HFIRRLD_CTRL_W::new(self, 16)
    }
}
#[doc = "Host Frame Interval Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFIR_SPEC;
impl crate::RegisterSpec for HFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfir::R`](R) reader structure"]
impl crate::Readable for HFIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfir::W`](W) writer structure"]
impl crate::Writable for HFIR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFIR to value 0xea60"]
impl crate::Resettable for HFIR_SPEC {
    const RESET_VALUE: u32 = 0xea60;
}
