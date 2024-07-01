#[doc = "Register `FDR` reader"]
pub type R = crate::R<FDR_SPEC>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FDR_SPEC>;
#[doc = "Field `STEP` reader - Step Value"]
pub type STEP_R = crate::FieldReader<u16>;
#[doc = "Field `STEP` writer - Step Value"]
pub type STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DM` reader - Divider Mode"]
pub type DM_R = crate::FieldReader;
#[doc = "Field `DM` writer - Divider Mode"]
pub type DM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<FDR_SPEC> {
        STEP_W::new(self, 0)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<FDR_SPEC> {
        DM_W::new(self, 14)
    }
}
#[doc = "CAN Fractional Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
