#[doc = "Register `LENGTH` reader"]
pub type R = crate::R<LENGTH_SPEC>;
#[doc = "Register `LENGTH` writer"]
pub type W = crate::W<LENGTH_SPEC>;
#[doc = "Field `LENGTH` reader - Message Length Register"]
pub type LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `LENGTH` writer - Message Length Register"]
pub type LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Message Length Register"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Message Length Register"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<LENGTH_SPEC> {
        LENGTH_W::new(self, 0)
    }
}
#[doc = "CRC Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LENGTH_SPEC;
impl crate::RegisterSpec for LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`length::R`](R) reader structure"]
impl crate::Readable for LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`length::W`](W) writer structure"]
impl crate::Writable for LENGTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LENGTH to value 0"]
impl crate::Resettable for LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
