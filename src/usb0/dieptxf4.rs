#[doc = "Register `DIEPTXF4` reader"]
pub type R = crate::R<Dieptxf4Spec>;
#[doc = "Register `DIEPTXF4` writer"]
pub type W = crate::W<Dieptxf4Spec>;
#[doc = "Field `INEPnTxFStAddr` reader - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type InepnTxFstAddrR = crate::FieldReader<u16>;
#[doc = "Field `INEPnTxFStAddr` writer - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type InepnTxFstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPnTxFDep` reader - IN Endpoint TxFIFO Depth"]
pub type InepnTxFdepR = crate::FieldReader<u16>;
#[doc = "Field `INEPnTxFDep` writer - IN Endpoint TxFIFO Depth"]
pub type InepnTxFdepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepn_tx_fst_addr(&self) -> InepnTxFstAddrR {
        InepnTxFstAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepn_tx_fdep(&self) -> InepnTxFdepR {
        InepnTxFdepR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn inepn_tx_fst_addr(&mut self) -> InepnTxFstAddrW<Dieptxf4Spec> {
        InepnTxFstAddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn inepn_tx_fdep(&mut self) -> InepnTxFdepW<Dieptxf4Spec> {
        InepnTxFdepW::new(self, 16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptxf4Spec;
impl crate::RegisterSpec for Dieptxf4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf4::R`](R) reader structure"]
impl crate::Readable for Dieptxf4Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptxf4::W`](W) writer structure"]
impl crate::Writable for Dieptxf4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF4 to value 0x0100_042a"]
impl crate::Resettable for Dieptxf4Spec {
    const RESET_VALUE: u32 = 0x0100_042a;
}
