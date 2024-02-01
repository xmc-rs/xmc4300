#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HPTXFSIZ_SPEC>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HPTXFSIZ_SPEC>;
#[doc = "Field `PTxFStAddr` reader - Host Periodic TxFIFO Start Address"]
pub type PTX_FST_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PTxFStAddr` writer - Host Periodic TxFIFO Start Address"]
pub type PTX_FST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTxFSize` reader - Host Periodic TxFIFO Depth"]
pub type PTX_FSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PTxFSize` writer - Host Periodic TxFIFO Depth"]
pub type PTX_FSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptx_fst_addr(&self) -> PTX_FST_ADDR_R {
        PTX_FST_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptx_fsize(&self) -> PTX_FSIZE_R {
        PTX_FSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fst_addr(&mut self) -> PTX_FST_ADDR_W<HPTXFSIZ_SPEC> {
        PTX_FST_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fsize(&mut self) -> PTX_FSIZE_W<HPTXFSIZ_SPEC> {
        PTX_FSIZE_W::new(self, 16)
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
#[doc = "Host Periodic Transmit FIFO Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXFSIZ_SPEC;
impl crate::RegisterSpec for HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HPTXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0100_012a"]
impl crate::Resettable for HPTXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x0100_012a;
}
