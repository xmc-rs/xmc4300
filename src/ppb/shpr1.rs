#[doc = "Register `SHPR1` reader"]
pub type R = crate::R<SHPR1_SPEC>;
#[doc = "Register `SHPR1` writer"]
pub type W = crate::W<SHPR1_SPEC>;
#[doc = "Field `PRI_4` reader - Priority of system handler 4, MemManage"]
pub type PRI_4_R = crate::FieldReader;
#[doc = "Field `PRI_4` writer - Priority of system handler 4, MemManage"]
pub type PRI_4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_5` reader - Priority of system handler 5, BusFault"]
pub type PRI_5_R = crate::FieldReader;
#[doc = "Field `PRI_5` writer - Priority of system handler 5, BusFault"]
pub type PRI_5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_6` reader - Priority of system handler 6, UsageFault"]
pub type PRI_6_R = crate::FieldReader;
#[doc = "Field `PRI_6` writer - Priority of system handler 6, UsageFault"]
pub type PRI_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub fn pri_4(&self) -> PRI_4_R {
        PRI_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&self) -> PRI_5_R {
        PRI_5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&self) -> PRI_6_R {
        PRI_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of system handler 4, MemManage"]
    #[inline(always)]
    pub fn pri_4(&mut self) -> PRI_4_W<SHPR1_SPEC> {
        PRI_4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Priority of system handler 5, BusFault"]
    #[inline(always)]
    pub fn pri_5(&mut self) -> PRI_5_W<SHPR1_SPEC> {
        PRI_5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Priority of system handler 6, UsageFault"]
    #[inline(always)]
    pub fn pri_6(&mut self) -> PRI_6_W<SHPR1_SPEC> {
        PRI_6_W::new(self, 16)
    }
}
#[doc = "System Handler Priority Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`shpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHPR1_SPEC;
impl crate::RegisterSpec for SHPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shpr1::R`](R) reader structure"]
impl crate::Readable for SHPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shpr1::W`](W) writer structure"]
impl crate::Writable for SHPR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHPR1 to value 0"]
impl crate::Resettable for SHPR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
