#[doc = "Register `GNPTXFSIZ_HOSTMODE` reader"]
pub type R = crate::R<GNPTXFSIZ_HOSTMODE_SPEC>;
#[doc = "Register `GNPTXFSIZ_HOSTMODE` writer"]
pub type W = crate::W<GNPTXFSIZ_HOSTMODE_SPEC>;
#[doc = "Field `NPTxFStAddr` reader - Non-periodic Transmit RAM Start Address"]
pub type NPTX_FST_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `NPTxFStAddr` writer - Non-periodic Transmit RAM Start Address"]
pub type NPTX_FST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTxFDep` reader - Non-periodic TxFIFO Depth"]
pub type NPTX_FDEP_R = crate::FieldReader<u16>;
#[doc = "Field `NPTxFDep` writer - Non-periodic TxFIFO Depth"]
pub type NPTX_FDEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptx_fst_addr(&self) -> NPTX_FST_ADDR_R {
        NPTX_FST_ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptx_fdep(&self) -> NPTX_FDEP_R {
        NPTX_FDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptx_fst_addr(&mut self) -> NPTX_FST_ADDR_W<GNPTXFSIZ_HOSTMODE_SPEC> {
        NPTX_FST_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptx_fdep(&mut self) -> NPTX_FDEP_W<GNPTXFSIZ_HOSTMODE_SPEC> {
        NPTX_FDEP_W::new(self, 16)
    }
}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz_hostmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz_hostmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_HOSTMODE_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_HOSTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz_hostmode::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_HOSTMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz_hostmode::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_HOSTMODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ_HOSTMODE to value 0x0010_011a"]
impl crate::Resettable for GNPTXFSIZ_HOSTMODE_SPEC {
    const RESET_VALUE: u32 = 0x0010_011a;
}
