#[doc = "Register `GLOBRES` reader"]
pub type R = crate::R<GLOBRES_SPEC>;
#[doc = "Register `GLOBRES` writer"]
pub type W = crate::W<GLOBRES_SPEC>;
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type RESULT_R = crate::FieldReader<u16>;
#[doc = "Field `RESULT` writer - Result of most recent conversion"]
pub type RESULT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GNR` reader - Group Number"]
pub type GNR_R = crate::FieldReader;
#[doc = "Field `CHNR` reader - Channel Number"]
pub type CHNR_R = crate::FieldReader;
#[doc = "Field `EMUX` reader - External Multiplexer Setting"]
pub type EMUX_R = crate::FieldReader;
#[doc = "Field `CRS` reader - Converted Request Source"]
pub type CRS_R = crate::FieldReader;
#[doc = "Fast Compare Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCR_A {
    #[doc = "0: Signal level was below compare value"]
    VALUE1 = 0,
    #[doc = "1: Signal level was above compare value"]
    VALUE2 = 1,
}
impl From<FCR_A> for bool {
    #[inline(always)]
    fn from(variant: FCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCR` reader - Fast Compare Result"]
pub type FCR_R = crate::BitReader<FCR_A>;
impl FCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FCR_A {
        match self.bits {
            false => FCR_A::VALUE1,
            true => FCR_A::VALUE2,
        }
    }
    #[doc = "Signal level was below compare value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCR_A::VALUE1
    }
    #[doc = "Signal level was above compare value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCR_A::VALUE2
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VF_A {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    VALUE1 = 0,
    #[doc = "1: Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    VALUE2 = 1,
}
impl From<VF_A> for bool {
    #[inline(always)]
    fn from(variant: VF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF` reader - Valid Flag"]
pub type VF_R = crate::BitReader<VF_A>;
impl VF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VF_A {
        match self.bits {
            false => VF_A::VALUE1,
            true => VF_A::VALUE2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VF_A::VALUE1
    }
    #[doc = "Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VF_A::VALUE2
    }
}
#[doc = "Field `VF` writer - Valid Flag"]
pub type VF_W<'a, REG> = crate::BitWriter<'a, REG, VF_A>;
impl<'a, REG> VF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VF_A::VALUE1)
    }
    #[doc = "Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VF_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Group Number"]
    #[inline(always)]
    pub fn gnr(&self) -> GNR_R {
        GNR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - Channel Number"]
    #[inline(always)]
    pub fn chnr(&self) -> CHNR_R {
        CHNR_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27 - External Multiplexer Setting"]
    #[inline(always)]
    pub fn emux(&self) -> EMUX_R {
        EMUX_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Converted Request Source"]
    #[inline(always)]
    pub fn crs(&self) -> CRS_R {
        CRS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Fast Compare Result"]
    #[inline(always)]
    pub fn fcr(&self) -> FCR_R {
        FCR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    #[must_use]
    pub fn result(&mut self) -> RESULT_W<GLOBRES_SPEC> {
        RESULT_W::new(self, 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vf(&mut self) -> VF_W<GLOBRES_SPEC> {
        VF_W::new(self, 31)
    }
}
#[doc = "Global Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globres::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBRES_SPEC;
impl crate::RegisterSpec for GLOBRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globres::R`](R) reader structure"]
impl crate::Readable for GLOBRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`globres::W`](W) writer structure"]
impl crate::Writable for GLOBRES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBRES to value 0"]
impl crate::Resettable for GLOBRES_SPEC {
    const RESET_VALUE: u32 = 0;
}
