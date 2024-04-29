#[doc = "Register `MMFAR` reader"]
pub type R = crate::R<MMFAR_SPEC>;
#[doc = "Register `MMFAR` writer"]
pub type W = crate::W<MMFAR_SPEC>;
#[doc = "Field `ADDRESS` reader - Address causing the fault"]
pub type ADDRESS_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - Address causing the fault"]
pub type ADDRESS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address causing the fault"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address causing the fault"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<MMFAR_SPEC> {
        ADDRESS_W::new(self, 0)
    }
}
#[doc = "MemManage Fault Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmfar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmfar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMFAR_SPEC;
impl crate::RegisterSpec for MMFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmfar::R`](R) reader structure"]
impl crate::Readable for MMFAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmfar::W`](W) writer structure"]
impl crate::Writable for MMFAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMFAR to value 0"]
impl crate::Resettable for MMFAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
