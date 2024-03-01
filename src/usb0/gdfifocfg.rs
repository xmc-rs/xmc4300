#[doc = "Register `GDFIFOCFG` reader"]
pub type R = crate::R<GdfifocfgSpec>;
#[doc = "Register `GDFIFOCFG` writer"]
pub type W = crate::W<GdfifocfgSpec>;
#[doc = "Field `GDFIFOCfg` reader - GDFIFOCfg"]
pub type GdfifocfgR = crate::FieldReader<u16>;
#[doc = "Field `GDFIFOCfg` writer - GDFIFOCfg"]
pub type GdfifocfgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EPInfoBaseAddr` reader - EPInfoBaseAddr"]
pub type EpinfoBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `EPInfoBaseAddr` writer - EPInfoBaseAddr"]
pub type EpinfoBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GDFIFOCfg"]
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GdfifocfgR {
        GdfifocfgR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EPInfoBaseAddr"]
    #[inline(always)]
    pub fn epinfo_base_addr(&self) -> EpinfoBaseAddrR {
        EpinfoBaseAddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GDFIFOCfg"]
    #[inline(always)]
    #[must_use]
    pub fn gdfifocfg(&mut self) -> GdfifocfgW<GdfifocfgSpec> {
        GdfifocfgW::new(self, 0)
    }
    #[doc = "Bits 16:31 - EPInfoBaseAddr"]
    #[inline(always)]
    #[must_use]
    pub fn epinfo_base_addr(&mut self) -> EpinfoBaseAddrW<GdfifocfgSpec> {
        EpinfoBaseAddrW::new(self, 16)
    }
}
#[doc = "Global DFIFO Software Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gdfifocfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdfifocfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdfifocfgSpec;
impl crate::RegisterSpec for GdfifocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdfifocfg::R`](R) reader structure"]
impl crate::Readable for GdfifocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gdfifocfg::W`](W) writer structure"]
impl crate::Writable for GdfifocfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDFIFOCFG to value 0x027a_02b2"]
impl crate::Resettable for GdfifocfgSpec {
    const RESET_VALUE: u32 = 0x027a_02b2;
}
