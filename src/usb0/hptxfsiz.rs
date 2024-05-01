#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HptxfsizSpec>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HptxfsizSpec>;
#[doc = "Field `PTxFStAddr` reader - Host Periodic TxFIFO Start Address"]
pub type PtxFstAddrR = crate::FieldReader<u16>;
#[doc = "Field `PTxFStAddr` writer - Host Periodic TxFIFO Start Address"]
pub type PtxFstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTxFSize` reader - Host Periodic TxFIFO Depth"]
pub type PtxFsizeR = crate::FieldReader<u16>;
#[doc = "Field `PTxFSize` writer - Host Periodic TxFIFO Depth"]
pub type PtxFsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptx_fst_addr(&self) -> PtxFstAddrR {
        PtxFstAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptx_fsize(&self) -> PtxFsizeR {
        PtxFsizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fst_addr(&mut self) -> PtxFstAddrW<HptxfsizSpec> {
        PtxFstAddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fsize(&mut self) -> PtxFsizeW<HptxfsizSpec> {
        PtxFsizeW::new(self, 16)
    }
}
#[doc = "Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxfsizSpec;
impl crate::RegisterSpec for HptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HptxfsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0100_012a"]
impl crate::Resettable for HptxfsizSpec {
    const RESET_VALUE: u32 = 0x0100_012a;
}
