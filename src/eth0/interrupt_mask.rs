#[doc = "Register `INTERRUPT_MASK` reader"]
pub type R = crate::R<INTERRUPT_MASK_SPEC>;
#[doc = "Register `INTERRUPT_MASK` writer"]
pub type W = crate::W<INTERRUPT_MASK_SPEC>;
#[doc = "Field `PMTIM` reader - PMT Interrupt Mask"]
pub type PMTIM_R = crate::BitReader;
#[doc = "Field `PMTIM` writer - PMT Interrupt Mask"]
pub type PMTIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask"]
pub type TSIM_R = crate::BitReader;
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask"]
pub type TSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT Interrupt Mask"]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W<INTERRUPT_MASK_SPEC> {
        PMTIM_W::new(self, 3)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TSIM_W<INTERRUPT_MASK_SPEC> {
        TSIM_W::new(self, 9)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_mask::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_mask::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_MASK to value 0"]
impl crate::Resettable for INTERRUPT_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
