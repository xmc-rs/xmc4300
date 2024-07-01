#[doc = "Register `GDFIFOCFG` reader"]
pub type R = crate::R<GDFIFOCFG_SPEC>;
#[doc = "Register `GDFIFOCFG` writer"]
pub type W = crate::W<GDFIFOCFG_SPEC>;
#[doc = "Field `GDFIFOCfg` reader - GDFIFOCfg"]
pub type GDFIFOCFG_R = crate::FieldReader<u16>;
#[doc = "Field `GDFIFOCfg` writer - GDFIFOCfg"]
pub type GDFIFOCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EPInfoBaseAddr` reader - EPInfoBaseAddr"]
pub type EPINFO_BASE_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `EPInfoBaseAddr` writer - EPInfoBaseAddr"]
pub type EPINFO_BASE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GDFIFOCfg"]
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GDFIFOCFG_R {
        GDFIFOCFG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EPInfoBaseAddr"]
    #[inline(always)]
    pub fn epinfo_base_addr(&self) -> EPINFO_BASE_ADDR_R {
        EPINFO_BASE_ADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GDFIFOCfg"]
    #[inline(always)]
    #[must_use]
    pub fn gdfifocfg(&mut self) -> GDFIFOCFG_W<GDFIFOCFG_SPEC> {
        GDFIFOCFG_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - EPInfoBaseAddr"]
    #[inline(always)]
    #[must_use]
    pub fn epinfo_base_addr(&mut self) -> EPINFO_BASE_ADDR_W<GDFIFOCFG_SPEC> {
        EPINFO_BASE_ADDR_W::new(self, 16)
    }
}
#[doc = "Global DFIFO Software Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdfifocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdfifocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDFIFOCFG_SPEC;
impl crate::RegisterSpec for GDFIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdfifocfg::R`](R) reader structure"]
impl crate::Readable for GDFIFOCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdfifocfg::W`](W) writer structure"]
impl crate::Writable for GDFIFOCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GDFIFOCFG to value 0x027a_02b2"]
impl crate::Resettable for GDFIFOCFG_SPEC {
    const RESET_VALUE: u32 = 0x027a_02b2;
}
