#[doc = "Register `GLOBRES` reader"]
pub type R = crate::R<GlobresSpec>;
#[doc = "Register `GLOBRES` writer"]
pub type W = crate::W<GlobresSpec>;
#[doc = "Field `RESULT` reader - Result of most recent conversion"]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `RESULT` writer - Result of most recent conversion"]
pub type ResultW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GNR` reader - Group Number"]
pub type GnrR = crate::FieldReader;
#[doc = "Field `CHNR` reader - Channel Number"]
pub type ChnrR = crate::FieldReader;
#[doc = "Field `EMUX` reader - External Multiplexer Setting"]
pub type EmuxR = crate::FieldReader;
#[doc = "Field `CRS` reader - Converted Request Source"]
pub type CrsR = crate::FieldReader;
#[doc = "Fast Compare Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcr {
    #[doc = "0: Signal level was below compare value"]
    Value1 = 0,
    #[doc = "1: Signal level was above compare value"]
    Value2 = 1,
}
impl From<Fcr> for bool {
    #[inline(always)]
    fn from(variant: Fcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCR` reader - Fast Compare Result"]
pub type FcrR = crate::BitReader<Fcr>;
impl FcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcr {
        match self.bits {
            false => Fcr::Value1,
            true => Fcr::Value2,
        }
    }
    #[doc = "Signal level was below compare value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fcr::Value1
    }
    #[doc = "Signal level was above compare value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fcr::Value2
    }
}
#[doc = "Valid Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vf {
    #[doc = "0: Read access: No new valid data available Write access: No effect"]
    Value1 = 0,
    #[doc = "1: Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    Value2 = 1,
}
impl From<Vf> for bool {
    #[inline(always)]
    fn from(variant: Vf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VF` reader - Valid Flag"]
pub type VfR = crate::BitReader<Vf>;
impl VfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vf {
        match self.bits {
            false => Vf::Value1,
            true => Vf::Value2,
        }
    }
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vf::Value1
    }
    #[doc = "Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vf::Value2
    }
}
#[doc = "Field `VF` writer - Valid Flag"]
pub type VfW<'a, REG> = crate::BitWriter<'a, REG, Vf>;
impl<'a, REG> VfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read access: No new valid data available Write access: No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vf::Value1)
    }
    #[doc = "Read access: Bitfield RESULT contains valid data and has not yet been read, or bit FCR has been updated Write access: Clear this valid flag and the data reduction counter (overrides a hardware set action)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vf::Value2)
    }
}
impl R {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Group Number"]
    #[inline(always)]
    pub fn gnr(&self) -> GnrR {
        GnrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - Channel Number"]
    #[inline(always)]
    pub fn chnr(&self) -> ChnrR {
        ChnrR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:27 - External Multiplexer Setting"]
    #[inline(always)]
    pub fn emux(&self) -> EmuxR {
        EmuxR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Converted Request Source"]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Fast Compare Result"]
    #[inline(always)]
    pub fn fcr(&self) -> FcrR {
        FcrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    pub fn vf(&self) -> VfR {
        VfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Result of most recent conversion"]
    #[inline(always)]
    #[must_use]
    pub fn result(&mut self) -> ResultW<GlobresSpec> {
        ResultW::new(self, 0)
    }
    #[doc = "Bit 31 - Valid Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vf(&mut self) -> VfW<GlobresSpec> {
        VfW::new(self, 31)
    }
}
#[doc = "Global Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globres::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobresSpec;
impl crate::RegisterSpec for GlobresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globres::R`](R) reader structure"]
impl crate::Readable for GlobresSpec {}
#[doc = "`write(|w| ..)` method takes [`globres::W`](W) writer structure"]
impl crate::Writable for GlobresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBRES to value 0"]
impl crate::Resettable for GlobresSpec {
    const RESET_VALUE: u32 = 0;
}
