#[doc = "Register `HCFG` reader"]
pub type R = crate::R<HCFG_SPEC>;
#[doc = "Register `HCFG` writer"]
pub type W = crate::W<HCFG_SPEC>;
#[doc = "Field `FSLSPclkSel` reader - FS PHY Clock Select"]
pub type FSLSPCLK_SEL_R = crate::FieldReader<FSLSPCLK_SEL_A>;
#[doc = "FS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSLSPCLK_SEL_A {
    #[doc = "1: PHY clock is running at 48 MHz"]
    VALUE1 = 1,
}
impl From<FSLSPCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLSPCLK_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FSLSPCLK_SEL_A {
    type Ux = u8;
}
impl FSLSPCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FSLSPCLK_SEL_A> {
        match self.bits {
            1 => Some(FSLSPCLK_SEL_A::VALUE1),
            _ => None,
        }
    }
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FSLSPCLK_SEL_A::VALUE1
    }
}
#[doc = "Field `FSLSPclkSel` writer - FS PHY Clock Select"]
pub type FSLSPCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FSLSPCLK_SEL_A>;
impl<'a, REG> FSLSPCLK_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FSLSPCLK_SEL_A::VALUE1)
    }
}
#[doc = "Field `FSLSSupp` reader - FS-Only Support"]
pub type FSLSSUPP_R = crate::BitReader<FSLSSUPP_A>;
#[doc = "FS-Only Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSLSSUPP_A {
    #[doc = "0: FS-only, connected device can supports also only FS."]
    VALUE1 = 0,
    #[doc = "1: FS-only, even if the connected device can support HS"]
    VALUE2 = 1,
}
impl From<FSLSSUPP_A> for bool {
    #[inline(always)]
    fn from(variant: FSLSSUPP_A) -> Self {
        variant as u8 != 0
    }
}
impl FSLSSUPP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSLSSUPP_A {
        match self.bits {
            false => FSLSSUPP_A::VALUE1,
            true => FSLSSUPP_A::VALUE2,
        }
    }
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FSLSSUPP_A::VALUE1
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FSLSSUPP_A::VALUE2
    }
}
#[doc = "Field `FSLSSupp` writer - FS-Only Support"]
pub type FSLSSUPP_W<'a, REG> = crate::BitWriter<'a, REG, FSLSSUPP_A>;
impl<'a, REG> FSLSSUPP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FSLSSUPP_A::VALUE1)
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FSLSSUPP_A::VALUE2)
    }
}
#[doc = "Field `DescDMA` reader - Enable Scatter/gather DMA in Host mode"]
pub type DESC_DMA_R = crate::BitReader;
#[doc = "Field `DescDMA` writer - Enable Scatter/gather DMA in Host mode"]
pub type DESC_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FrListEn` reader - Frame List Entries"]
pub type FR_LIST_EN_R = crate::FieldReader<FR_LIST_EN_A>;
#[doc = "Frame List Entries\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FR_LIST_EN_A {
    #[doc = "0: 8 Entries"]
    VALUE1 = 0,
    #[doc = "1: 16 Entries"]
    VALUE2 = 1,
    #[doc = "2: 32 Entries"]
    VALUE3 = 2,
    #[doc = "3: 64 Entries"]
    VALUE4 = 3,
}
impl From<FR_LIST_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: FR_LIST_EN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FR_LIST_EN_A {
    type Ux = u8;
}
impl FR_LIST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FR_LIST_EN_A {
        match self.bits {
            0 => FR_LIST_EN_A::VALUE1,
            1 => FR_LIST_EN_A::VALUE2,
            2 => FR_LIST_EN_A::VALUE3,
            3 => FR_LIST_EN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "8 Entries"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE1
    }
    #[doc = "16 Entries"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE2
    }
    #[doc = "32 Entries"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE3
    }
    #[doc = "64 Entries"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FR_LIST_EN_A::VALUE4
    }
}
#[doc = "Field `FrListEn` writer - Frame List Entries"]
pub type FR_LIST_EN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, FR_LIST_EN_A>;
impl<'a, REG> FR_LIST_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 Entries"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FR_LIST_EN_A::VALUE1)
    }
    #[doc = "16 Entries"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FR_LIST_EN_A::VALUE2)
    }
    #[doc = "32 Entries"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(FR_LIST_EN_A::VALUE3)
    }
    #[doc = "64 Entries"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(FR_LIST_EN_A::VALUE4)
    }
}
#[doc = "Field `PerSchedEna` reader - Enable Periodic Scheduling"]
pub type PER_SCHED_ENA_R = crate::BitReader;
#[doc = "Field `PerSchedEna` writer - Enable Periodic Scheduling"]
pub type PER_SCHED_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclk_sel(&self) -> FSLSPCLK_SEL_R {
        FSLSPCLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESC_DMA_R {
        DESC_DMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    pub fn fr_list_en(&self) -> FR_LIST_EN_R {
        FR_LIST_EN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    pub fn per_sched_ena(&self) -> PER_SCHED_ENA_R {
        PER_SCHED_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspclk_sel(&mut self) -> FSLSPCLK_SEL_W<HCFG_SPEC> {
        FSLSPCLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    #[must_use]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W<HCFG_SPEC> {
        FSLSSUPP_W::new(self, 2)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    #[must_use]
    pub fn desc_dma(&mut self) -> DESC_DMA_W<HCFG_SPEC> {
        DESC_DMA_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    #[must_use]
    pub fn fr_list_en(&mut self) -> FR_LIST_EN_W<HCFG_SPEC> {
        FR_LIST_EN_W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    #[must_use]
    pub fn per_sched_ena(&mut self) -> PER_SCHED_ENA_W<HCFG_SPEC> {
        PER_SCHED_ENA_W::new(self, 26)
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
#[doc = "Host Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCFG_SPEC;
impl crate::RegisterSpec for HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcfg::R`](R) reader structure"]
impl crate::Readable for HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcfg::W`](W) writer structure"]
impl crate::Writable for HCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCFG to value 0x0200"]
impl crate::Resettable for HCFG_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
