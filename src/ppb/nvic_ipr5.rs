#[doc = "Register `NVIC_IPR5` reader"]
pub type R = crate::R<NVIC_IPR5_SPEC>;
#[doc = "Register `NVIC_IPR5` writer"]
pub type W = crate::W<NVIC_IPR5_SPEC>;
#[doc = "Field `PRI_0` reader - Priority value 0"]
pub type PRI_0_R = crate::FieldReader;
#[doc = "Field `PRI_0` writer - Priority value 0"]
pub type PRI_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_1` reader - Priority value 1"]
pub type PRI_1_R = crate::FieldReader;
#[doc = "Field `PRI_1` writer - Priority value 1"]
pub type PRI_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_2` reader - Priority value 2"]
pub type PRI_2_R = crate::FieldReader;
#[doc = "Field `PRI_2` writer - Priority value 2"]
pub type PRI_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_3` reader - Priority value 3"]
pub type PRI_3_R = crate::FieldReader;
#[doc = "Field `PRI_3` writer - Priority value 3"]
pub type PRI_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    pub fn pri_0(&self) -> PRI_0_R {
        PRI_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    pub fn pri_1(&self) -> PRI_1_R {
        PRI_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    pub fn pri_2(&self) -> PRI_2_R {
        PRI_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    pub fn pri_3(&self) -> PRI_3_R {
        PRI_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    #[must_use]
    pub fn pri_0(&mut self) -> PRI_0_W<NVIC_IPR5_SPEC> {
        PRI_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    #[must_use]
    pub fn pri_1(&mut self) -> PRI_1_W<NVIC_IPR5_SPEC> {
        PRI_1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    #[must_use]
    pub fn pri_2(&mut self) -> PRI_2_W<NVIC_IPR5_SPEC> {
        PRI_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    #[must_use]
    pub fn pri_3(&mut self) -> PRI_3_W<NVIC_IPR5_SPEC> {
        PRI_3_W::new(self, 24)
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
#[doc = "Interrupt Priority Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_IPR5_SPEC;
impl crate::RegisterSpec for NVIC_IPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr5::R`](R) reader structure"]
impl crate::Readable for NVIC_IPR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr5::W`](W) writer structure"]
impl crate::Writable for NVIC_IPR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR5 to value 0"]
impl crate::Resettable for NVIC_IPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
