#[doc = "Register `DIEPTXF1` reader"]
pub type R = crate::R<DIEPTXF1_SPEC>;
#[doc = "Register `DIEPTXF1` writer"]
pub type W = crate::W<DIEPTXF1_SPEC>;
#[doc = "Field `INEPnTxFStAddr` reader - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type INEPN_TX_FST_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `INEPnTxFStAddr` writer - IN Endpoint FIFOn Transmit RAM Start Address"]
pub type INEPN_TX_FST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPnTxFDep` reader - IN Endpoint TxFIFO Depth"]
pub type INEPN_TX_FDEP_R = crate::FieldReader<u16>;
#[doc = "Field `INEPnTxFDep` writer - IN Endpoint TxFIFO Depth"]
pub type INEPN_TX_FDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepn_tx_fst_addr(&self) -> INEPN_TX_FST_ADDR_R {
        INEPN_TX_FST_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepn_tx_fdep(&self) -> INEPN_TX_FDEP_R {
        INEPN_TX_FDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn inepn_tx_fst_addr(&mut self) -> INEPN_TX_FST_ADDR_W<DIEPTXF1_SPEC> {
        INEPN_TX_FST_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn inepn_tx_fdep(&mut self) -> INEPN_TX_FDEP_W<DIEPTXF1_SPEC> {
        INEPN_TX_FDEP_W::new(self, 16)
    }
}
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF1_SPEC;
impl crate::RegisterSpec for DIEPTXF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf1::R`](R) reader structure"]
impl crate::Readable for DIEPTXF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf1::W`](W) writer structure"]
impl crate::Writable for DIEPTXF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF1 to value 0x0100_012a"]
impl crate::Resettable for DIEPTXF1_SPEC {
    const RESET_VALUE: u32 = 0x0100_012a;
}
