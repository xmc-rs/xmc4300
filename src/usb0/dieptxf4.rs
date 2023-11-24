#[doc = "Register `DIEPTXF4` reader"]
pub type R = crate::R<DIEPTXF4_SPEC>;
#[doc = "Register `DIEPTXF4` writer"]
pub type W = crate::W<DIEPTXF4_SPEC>;
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
    pub fn inepn_tx_fst_addr(&mut self) -> INEPN_TX_FST_ADDR_W<DIEPTXF4_SPEC> {
        INEPN_TX_FST_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn inepn_tx_fdep(&mut self) -> INEPN_TX_FDEP_W<DIEPTXF4_SPEC> {
        INEPN_TX_FDEP_W::new(self, 16)
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
#[doc = "Device IN Endpoint Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF4_SPEC;
impl crate::RegisterSpec for DIEPTXF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf4::R`](R) reader structure"]
impl crate::Readable for DIEPTXF4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf4::W`](W) writer structure"]
impl crate::Writable for DIEPTXF4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF4 to value 0x0100_042a"]
impl crate::Resettable for DIEPTXF4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_042a;
}
