#[doc = "Register `DAINTMSK` reader"]
pub type R = crate::R<DAINTMSK_SPEC>;
#[doc = "Register `DAINTMSK` writer"]
pub type W = crate::W<DAINTMSK_SPEC>;
#[doc = "Field `InEpMsk` reader - IN EP Interrupt Mask Bits"]
pub type IN_EP_MSK_R = crate::FieldReader<u16>;
#[doc = "Field `InEpMsk` writer - IN EP Interrupt Mask Bits"]
pub type IN_EP_MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OutEpMsk` reader - OUT EP Interrupt Mask Bits"]
pub type OUT_EP_MSK_R = crate::FieldReader<u16>;
#[doc = "Field `OutEpMsk` writer - OUT EP Interrupt Mask Bits"]
pub type OUT_EP_MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_msk(&self) -> IN_EP_MSK_R {
        IN_EP_MSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn out_ep_msk(&self) -> OUT_EP_MSK_R {
        OUT_EP_MSK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_msk(&mut self) -> IN_EP_MSK_W<DAINTMSK_SPEC> {
        IN_EP_MSK_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - OUT EP Interrupt Mask Bits"]
    #[inline(always)]
    pub fn out_ep_msk(&mut self) -> OUT_EP_MSK_W<DAINTMSK_SPEC> {
        OUT_EP_MSK_W::new(self, 16)
    }
}
#[doc = "Device All Endpoints Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daintmsk::R`](R) reader structure"]
impl crate::Readable for DAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daintmsk::W`](W) writer structure"]
impl crate::Writable for DAINTMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
