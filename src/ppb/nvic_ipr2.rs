#[doc = "Register `NVIC_IPR2` reader"]
pub type R = crate::R<NvicIpr2Spec>;
#[doc = "Register `NVIC_IPR2` writer"]
pub type W = crate::W<NvicIpr2Spec>;
#[doc = "Field `PRI_0` reader - Priority value 0"]
pub type Pri0R = crate::FieldReader;
#[doc = "Field `PRI_0` writer - Priority value 0"]
pub type Pri0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_1` reader - Priority value 1"]
pub type Pri1R = crate::FieldReader;
#[doc = "Field `PRI_1` writer - Priority value 1"]
pub type Pri1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_2` reader - Priority value 2"]
pub type Pri2R = crate::FieldReader;
#[doc = "Field `PRI_2` writer - Priority value 2"]
pub type Pri2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_3` reader - Priority value 3"]
pub type Pri3R = crate::FieldReader;
#[doc = "Field `PRI_3` writer - Priority value 3"]
pub type Pri3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    pub fn pri_0(&self) -> Pri0R {
        Pri0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    pub fn pri_1(&self) -> Pri1R {
        Pri1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    pub fn pri_2(&self) -> Pri2R {
        Pri2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    pub fn pri_3(&self) -> Pri3R {
        Pri3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority value 0"]
    #[inline(always)]
    #[must_use]
    pub fn pri_0(&mut self) -> Pri0W<NvicIpr2Spec> {
        Pri0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Priority value 1"]
    #[inline(always)]
    #[must_use]
    pub fn pri_1(&mut self) -> Pri1W<NvicIpr2Spec> {
        Pri1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Priority value 2"]
    #[inline(always)]
    #[must_use]
    pub fn pri_2(&mut self) -> Pri2W<NvicIpr2Spec> {
        Pri2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Priority value 3"]
    #[inline(always)]
    #[must_use]
    pub fn pri_3(&mut self) -> Pri3W<NvicIpr2Spec> {
        Pri3W::new(self, 24)
    }
}
#[doc = "Interrupt Priority Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr2Spec;
impl crate::RegisterSpec for NvicIpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr2::R`](R) reader structure"]
impl crate::Readable for NvicIpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr2::W`](W) writer structure"]
impl crate::Writable for NvicIpr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR2 to value 0"]
impl crate::Resettable for NvicIpr2Spec {
    const RESET_VALUE: u32 = 0;
}
