#[doc = "Register `HCDMA_SCATGATHER` reader"]
pub type R = crate::R<HCDMA_SCATGATHER_SPEC>;
#[doc = "Register `HCDMA_SCATGATHER` writer"]
pub type W = crate::W<HCDMA_SCATGATHER_SPEC>;
#[doc = "Current Transfer Desc:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTD_A {
    #[doc = "0: 1 descriptor"]
    VALUE1 = 0,
    #[doc = "63: 64 descriptors"]
    VALUE2 = 63,
}
impl From<CTD_A> for u8 {
    #[inline(always)]
    fn from(variant: CTD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTD_A {
    type Ux = u8;
}
impl crate::IsEnum for CTD_A {}
#[doc = "Field `CTD` reader - Current Transfer Desc:"]
pub type CTD_R = crate::FieldReader<CTD_A>;
impl CTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTD_A> {
        match self.bits {
            0 => Some(CTD_A::VALUE1),
            63 => Some(CTD_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "1 descriptor"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CTD_A::VALUE1
    }
    #[doc = "64 descriptors"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CTD_A::VALUE2
    }
}
#[doc = "Field `CTD` writer - Current Transfer Desc:"]
pub type CTD_W<'a, REG> = crate::FieldWriter<'a, REG, 6, CTD_A>;
impl<'a, REG> CTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 descriptor"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CTD_A::VALUE1)
    }
    #[doc = "64 descriptors"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CTD_A::VALUE2)
    }
}
#[doc = "Field `DMAAddr` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMAAddr` writer - DMA Address"]
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    pub fn ctd(&self) -> CTD_R {
        CTD_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 3:8 - Current Transfer Desc:"]
    #[inline(always)]
    #[must_use]
    pub fn ctd(&mut self) -> CTD_W<HCDMA_SCATGATHER_SPEC> {
        CTD_W::new(self, 3)
    }
    #[doc = "Bits 9:31 - DMA Address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<HCDMA_SCATGATHER_SPEC> {
        DMAADDR_W::new(self, 9)
    }
}
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma_scatgather::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma_scatgather::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMA_SCATGATHER_SPEC;
impl crate::RegisterSpec for HCDMA_SCATGATHER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma_scatgather::R`](R) reader structure"]
impl crate::Readable for HCDMA_SCATGATHER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcdma_scatgather::W`](W) writer structure"]
impl crate::Writable for HCDMA_SCATGATHER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA_SCATGATHER to value 0"]
impl crate::Resettable for HCDMA_SCATGATHER_SPEC {
    const RESET_VALUE: u32 = 0;
}
