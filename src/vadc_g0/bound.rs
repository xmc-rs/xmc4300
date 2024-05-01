#[doc = "Register `BOUND` reader"]
pub type R = crate::R<BoundSpec>;
#[doc = "Register `BOUND` writer"]
pub type W = crate::W<BoundSpec>;
#[doc = "Field `BOUNDARY0` reader - Boundary Value 0 for Limit Checking"]
pub type Boundary0R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY0` writer - Boundary Value 0 for Limit Checking"]
pub type Boundary0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BOUNDARY1` reader - Boundary Value 1 for Limit Checking"]
pub type Boundary1R = crate::FieldReader<u16>;
#[doc = "Field `BOUNDARY1` writer - Boundary Value 1 for Limit Checking"]
pub type Boundary1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    pub fn boundary0(&self) -> Boundary0R {
        Boundary0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    pub fn boundary1(&self) -> Boundary1R {
        Boundary1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Boundary Value 0 for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundary0(&mut self) -> Boundary0W<BoundSpec> {
        Boundary0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Boundary Value 1 for Limit Checking"]
    #[inline(always)]
    #[must_use]
    pub fn boundary1(&mut self) -> Boundary1W<BoundSpec> {
        Boundary1W::new(self, 16)
    }
}
#[doc = "Boundary Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bound::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bound::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BoundSpec;
impl crate::RegisterSpec for BoundSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bound::R`](R) reader structure"]
impl crate::Readable for BoundSpec {}
#[doc = "`write(|w| ..)` method takes [`bound::W`](W) writer structure"]
impl crate::Writable for BoundSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOUND to value 0"]
impl crate::Resettable for BoundSpec {
    const RESET_VALUE: u32 = 0;
}
