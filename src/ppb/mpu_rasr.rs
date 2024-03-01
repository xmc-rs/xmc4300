#[doc = "Register `MPU_RASR` reader"]
pub type R = crate::R<MpuRasrSpec>;
#[doc = "Register `MPU_RASR` writer"]
pub type W = crate::W<MpuRasrSpec>;
#[doc = "Field `ENABLE` reader - Region enable bit."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Region enable bit."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE` reader - MPU protection region size"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - MPU protection region size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Subregion disable bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srd {
    #[doc = "0: corresponding sub-region is enabled"]
    Value1 = 0,
    #[doc = "1: corresponding sub-region is disabled"]
    Value2 = 1,
}
impl From<Srd> for u8 {
    #[inline(always)]
    fn from(variant: Srd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srd {
    type Ux = u8;
}
#[doc = "Field `SRD` reader - Subregion disable bits"]
pub type SrdR = crate::FieldReader<Srd>;
impl SrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srd> {
        match self.bits {
            0 => Some(Srd::Value1),
            1 => Some(Srd::Value2),
            _ => None,
        }
    }
    #[doc = "corresponding sub-region is enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srd::Value1
    }
    #[doc = "corresponding sub-region is disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srd::Value2
    }
}
#[doc = "Field `SRD` writer - Subregion disable bits"]
pub type SrdW<'a, REG> = crate::FieldWriter<'a, REG, 8, Srd>;
impl<'a, REG> SrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "corresponding sub-region is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srd::Value1)
    }
    #[doc = "corresponding sub-region is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srd::Value2)
    }
}
#[doc = "Field `B` reader - Memory access attribute"]
pub type BR = crate::BitReader;
#[doc = "Field `B` writer - Memory access attribute"]
pub type BW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C` reader - Memory access attribute"]
pub type CR = crate::BitReader;
#[doc = "Field `C` writer - Memory access attribute"]
pub type CW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - Shareable bit"]
pub type SR = crate::BitReader;
#[doc = "Field `S` writer - Shareable bit"]
pub type SW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEX` reader - Memory access attribute"]
pub type TexR = crate::FieldReader;
#[doc = "Field `TEX` writer - Memory access attribute"]
pub type TexW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AP` reader - Access permission field"]
pub type ApR = crate::FieldReader;
#[doc = "Field `AP` writer - Access permission field"]
pub type ApW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Instruction access disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xn {
    #[doc = "0: instruction fetches enabled"]
    Value1 = 0,
    #[doc = "1: instruction fetches disabled."]
    Value2 = 1,
}
impl From<Xn> for bool {
    #[inline(always)]
    fn from(variant: Xn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XN` reader - Instruction access disable bit"]
pub type XnR = crate::BitReader<Xn>;
impl XnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xn {
        match self.bits {
            false => Xn::Value1,
            true => Xn::Value2,
        }
    }
    #[doc = "instruction fetches enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Xn::Value1
    }
    #[doc = "instruction fetches disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Xn::Value2
    }
}
#[doc = "Field `XN` writer - Instruction access disable bit"]
pub type XnW<'a, REG> = crate::BitWriter<'a, REG, Xn>;
impl<'a, REG> XnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "instruction fetches enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Xn::Value1)
    }
    #[doc = "instruction fetches disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Xn::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    pub fn srd(&self) -> SrdR {
        SrdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    pub fn tex(&self) -> TexR {
        TexR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    pub fn xn(&self) -> XnR {
        XnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<MpuRasrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:5 - MPU protection region size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<MpuRasrSpec> {
        SizeW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Subregion disable bits"]
    #[inline(always)]
    #[must_use]
    pub fn srd(&mut self) -> SrdW<MpuRasrSpec> {
        SrdW::new(self, 8)
    }
    #[doc = "Bit 16 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> BW<MpuRasrSpec> {
        BW::new(self, 16)
    }
    #[doc = "Bit 17 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> CW<MpuRasrSpec> {
        CW::new(self, 17)
    }
    #[doc = "Bit 18 - Shareable bit"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> SW<MpuRasrSpec> {
        SW::new(self, 18)
    }
    #[doc = "Bits 19:21 - Memory access attribute"]
    #[inline(always)]
    #[must_use]
    pub fn tex(&mut self) -> TexW<MpuRasrSpec> {
        TexW::new(self, 19)
    }
    #[doc = "Bits 24:26 - Access permission field"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> ApW<MpuRasrSpec> {
        ApW::new(self, 24)
    }
    #[doc = "Bit 28 - Instruction access disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XnW<MpuRasrSpec> {
        XnW::new(self, 28)
    }
}
#[doc = "MPU Region Attribute and Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRasrSpec;
impl crate::RegisterSpec for MpuRasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rasr::R`](R) reader structure"]
impl crate::Readable for MpuRasrSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rasr::W`](W) writer structure"]
impl crate::Writable for MpuRasrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RASR to value 0"]
impl crate::Resettable for MpuRasrSpec {
    const RESET_VALUE: u32 = 0;
}
