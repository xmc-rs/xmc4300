#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HcfgSpec>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HcfgSpec>;
#[doc = "FS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FslspclkSel {
    #[doc = "1: PHY clock is running at 48 MHz"]
    Value1 = 1,
}
impl From<FslspclkSel> for u8 {
    #[inline(always)]
    fn from(variant: FslspclkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FslspclkSel {
    type Ux = u8;
}
#[doc = "Field `FSLSPclkSel` reader - FS PHY Clock Select"]
pub type FslspclkSelR = crate::FieldReader<FslspclkSel>;
impl FslspclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FslspclkSel> {
        match self.bits {
            1 => Some(FslspclkSel::Value1),
            _ => None,
        }
    }
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FslspclkSel::Value1
    }
}
#[doc = "Field `FSLSPclkSel` writer - FS PHY Clock Select"]
pub type FslspclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, FslspclkSel>;
impl<'a, REG> FslspclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FslspclkSel::Value1)
    }
}
#[doc = "FS-Only Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fslssupp {
    #[doc = "0: FS-only, connected device can supports also only FS."]
    Value1 = 0,
    #[doc = "1: FS-only, even if the connected device can support HS"]
    Value2 = 1,
}
impl From<Fslssupp> for bool {
    #[inline(always)]
    fn from(variant: Fslssupp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSLSSupp` reader - FS-Only Support"]
pub type FslssuppR = crate::BitReader<Fslssupp>;
impl FslssuppR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fslssupp {
        match self.bits {
            false => Fslssupp::Value1,
            true => Fslssupp::Value2,
        }
    }
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fslssupp::Value1
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fslssupp::Value2
    }
}
#[doc = "Field `FSLSSupp` writer - FS-Only Support"]
pub type FslssuppW<'a, REG> = crate::BitWriter<'a, REG, Fslssupp>;
impl<'a, REG> FslssuppW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fslssupp::Value1)
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fslssupp::Value2)
    }
}
#[doc = "Field `DescDMA` reader - Enable Scatter/gather DMA in Host mode"]
pub type DescDmaR = crate::BitReader;
#[doc = "Field `DescDMA` writer - Enable Scatter/gather DMA in Host mode"]
pub type DescDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Frame List Entries\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FrListEn {
    #[doc = "0: 8 Entries"]
    Value1 = 0,
    #[doc = "1: 16 Entries"]
    Value2 = 1,
    #[doc = "2: 32 Entries"]
    Value3 = 2,
    #[doc = "3: 64 Entries"]
    Value4 = 3,
}
impl From<FrListEn> for u8 {
    #[inline(always)]
    fn from(variant: FrListEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FrListEn {
    type Ux = u8;
}
#[doc = "Field `FrListEn` reader - Frame List Entries"]
pub type FrListEnR = crate::FieldReader<FrListEn>;
impl FrListEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrListEn {
        match self.bits {
            0 => FrListEn::Value1,
            1 => FrListEn::Value2,
            2 => FrListEn::Value3,
            3 => FrListEn::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "8 Entries"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FrListEn::Value1
    }
    #[doc = "16 Entries"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FrListEn::Value2
    }
    #[doc = "32 Entries"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FrListEn::Value3
    }
    #[doc = "64 Entries"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FrListEn::Value4
    }
}
#[doc = "Field `FrListEn` writer - Frame List Entries"]
pub type FrListEnW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FrListEn>;
impl<'a, REG> FrListEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 Entries"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FrListEn::Value1)
    }
    #[doc = "16 Entries"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FrListEn::Value2)
    }
    #[doc = "32 Entries"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(FrListEn::Value3)
    }
    #[doc = "64 Entries"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(FrListEn::Value4)
    }
}
#[doc = "Field `PerSchedEna` reader - Enable Periodic Scheduling"]
pub type PerSchedEnaR = crate::BitReader;
#[doc = "Field `PerSchedEna` writer - Enable Periodic Scheduling"]
pub type PerSchedEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclk_sel(&self) -> FslspclkSelR {
        FslspclkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FslssuppR {
        FslssuppR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    pub fn desc_dma(&self) -> DescDmaR {
        DescDmaR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    pub fn fr_list_en(&self) -> FrListEnR {
        FrListEnR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    pub fn per_sched_ena(&self) -> PerSchedEnaR {
        PerSchedEnaR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclk_sel(&mut self) -> FslspclkSelW<HcfgSpec> {
        FslspclkSelW::new(self, 0)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    #[must_use]
    pub fn fslssupp(&mut self) -> FslssuppW<HcfgSpec> {
        FslssuppW::new(self, 2)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    #[must_use]
    pub fn desc_dma(&mut self) -> DescDmaW<HcfgSpec> {
        DescDmaW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    #[must_use]
    pub fn fr_list_en(&mut self) -> FrListEnW<HcfgSpec> {
        FrListEnW::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn per_sched_ena(&mut self) -> PerSchedEnaW<HcfgSpec> {
        PerSchedEnaW::new(self, 26)
    }
}
#[doc = "Host Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcfgSpec;
impl crate::RegisterSpec for HcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0x0200"]
impl crate::Resettable for HcfgSpec {
    const RESET_VALUE: u32 = 0x0200;
}
