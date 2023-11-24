#[doc = "Register `PWRMON` reader"]
pub type R = crate::R<PWRMON_SPEC>;
#[doc = "Register `PWRMON` writer"]
pub type W = crate::W<PWRMON_SPEC>;
#[doc = "Field `THRS` reader - Threshold"]
pub type THRS_R = crate::FieldReader;
#[doc = "Field `THRS` writer - Threshold"]
pub type THRS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INTV` reader - Interval"]
pub type INTV_R = crate::FieldReader;
#[doc = "Field `INTV` writer - Interval"]
pub type INTV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENB` reader - Enable"]
pub type ENB_R = crate::BitReader;
#[doc = "Field `ENB` writer - Enable"]
pub type ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    pub fn thrs(&self) -> THRS_R {
        THRS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    pub fn intv(&self) -> INTV_R {
        INTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thrs(&mut self) -> THRS_W<PWRMON_SPEC> {
        THRS_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interval"]
    #[inline(always)]
    #[must_use]
    pub fn intv(&mut self) -> INTV_W<PWRMON_SPEC> {
        INTV_W::new(self, 8)
    }
    #[doc = "Bit 16 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<PWRMON_SPEC> {
        ENB_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Power Monitor Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrmon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrmon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRMON_SPEC;
impl crate::RegisterSpec for PWRMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrmon::R`](R) reader structure"]
impl crate::Readable for PWRMON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrmon::W`](W) writer structure"]
impl crate::Writable for PWRMON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRMON to value 0"]
impl crate::Resettable for PWRMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
