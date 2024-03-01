#[doc = "Register `GNPTXFSIZ_HOSTMODE` reader"]
pub type R = crate::R<GnptxfsizHostmodeSpec>;
#[doc = "Register `GNPTXFSIZ_HOSTMODE` writer"]
pub type W = crate::W<GnptxfsizHostmodeSpec>;
#[doc = "Field `NPTxFStAddr` reader - Non-periodic Transmit RAM Start Address"]
pub type NptxFstAddrR = crate::FieldReader<u16>;
#[doc = "Field `NPTxFStAddr` writer - Non-periodic Transmit RAM Start Address"]
pub type NptxFstAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTxFDep` reader - Non-periodic TxFIFO Depth"]
pub type NptxFdepR = crate::FieldReader<u16>;
#[doc = "Field `NPTxFDep` writer - Non-periodic TxFIFO Depth"]
pub type NptxFdepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptx_fst_addr(&self) -> NptxFstAddrR {
        NptxFstAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptx_fdep(&self) -> NptxFdepR {
        NptxFdepR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn nptx_fst_addr(&mut self) -> NptxFstAddrW<GnptxfsizHostmodeSpec> {
        NptxFstAddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptx_fdep(&mut self) -> NptxFdepW<GnptxfsizHostmodeSpec> {
        NptxFdepW::new(self, 16)
    }
}
#[doc = "Non-Periodic Transmit FIFO Size Register \\[HOSTMODE\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_hostmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_hostmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GnptxfsizHostmodeSpec;
impl crate::RegisterSpec for GnptxfsizHostmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz_hostmode::R`](R) reader structure"]
impl crate::Readable for GnptxfsizHostmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz_hostmode::W`](W) writer structure"]
impl crate::Writable for GnptxfsizHostmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ_HOSTMODE to value 0x0010_011a"]
impl crate::Resettable for GnptxfsizHostmodeSpec {
    const RESET_VALUE: u32 = 0x0010_011a;
}
