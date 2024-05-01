#[doc = "Register `DIEPEMPMSK` reader"]
pub type R = crate::R<DiepempmskSpec>;
#[doc = "Register `DIEPEMPMSK` writer"]
pub type W = crate::W<DiepempmskSpec>;
#[doc = "Field `InEpTxfEmpMsk` reader - IN EP Tx FIFO Empty Interrupt Mask Bits"]
pub type InEpTxfEmpMskR = crate::FieldReader<u16>;
#[doc = "Field `InEpTxfEmpMsk` writer - IN EP Tx FIFO Empty Interrupt Mask Bits"]
pub type InEpTxfEmpMskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    pub fn in_ep_txf_emp_msk(&self) -> InEpTxfEmpMskR {
        InEpTxfEmpMskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO Empty Interrupt Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn in_ep_txf_emp_msk(&mut self) -> InEpTxfEmpMskW<DiepempmskSpec> {
        InEpTxfEmpMskW::new(self, 0)
    }
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepempmskSpec;
impl crate::RegisterSpec for DiepempmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepempmsk::R`](R) reader structure"]
impl crate::Readable for DiepempmskSpec {}
#[doc = "`write(|w| ..)` method takes [`diepempmsk::W`](W) writer structure"]
impl crate::Writable for DiepempmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPEMPMSK to value 0"]
impl crate::Resettable for DiepempmskSpec {
    const RESET_VALUE: u32 = 0;
}
