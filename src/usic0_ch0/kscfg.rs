#[doc = "Register `KSCFG` reader"]
pub type R = crate::R<KSCFG_SPEC>;
#[doc = "Register `KSCFG` writer"]
pub type W = crate::W<KSCFG_SPEC>;
#[doc = "Field `MODEN` reader - Module Enable"]
pub type MODEN_R = crate::BitReader<MODEN_A>;
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODEN_A {
    #[doc = "0: The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    VALUE1 = 0,
    #[doc = "1: The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    VALUE2 = 1,
}
impl From<MODEN_A> for bool {
    #[inline(always)]
    fn from(variant: MODEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MODEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODEN_A {
        match self.bits {
            false => MODEN_A::VALUE1,
            true => MODEN_A::VALUE2,
        }
    }
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MODEN_A::VALUE1
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MODEN_A::VALUE2
    }
}
#[doc = "Field `MODEN` writer - Module Enable"]
pub type MODEN_W<'a, REG> = crate::BitWriter<'a, REG, MODEN_A>;
impl<'a, REG> MODEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MODEN_A::VALUE1)
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MODEN_A::VALUE2)
    }
}
#[doc = "Bit Protection for MODEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPMODEN_AW {
    #[doc = "0: MODEN is not changed."]
    VALUE1 = 0,
    #[doc = "1: MODEN is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPMODEN_AW> for bool {
    #[inline(always)]
    fn from(variant: BPMODEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPMODEN` writer - Bit Protection for MODEN"]
pub type BPMODEN_W<'a, REG> = crate::BitWriter<'a, REG, BPMODEN_AW>;
impl<'a, REG> BPMODEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODEN is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BPMODEN_AW::VALUE1)
    }
    #[doc = "MODEN is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BPMODEN_AW::VALUE2)
    }
}
#[doc = "Field `NOMCFG` reader - Normal Operation Mode Configuration"]
pub type NOMCFG_R = crate::FieldReader<NOMCFG_A>;
#[doc = "Normal Operation Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NOMCFG_A {
    #[doc = "0: Run mode 0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Run mode 1 is selected."]
    VALUE2 = 1,
    #[doc = "2: Stop mode 0 is selected."]
    VALUE3 = 2,
    #[doc = "3: Stop mode 1 is selected."]
    VALUE4 = 3,
}
impl From<NOMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: NOMCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NOMCFG_A {
    type Ux = u8;
}
impl NOMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOMCFG_A {
        match self.bits {
            0 => NOMCFG_A::VALUE1,
            1 => NOMCFG_A::VALUE2,
            2 => NOMCFG_A::VALUE3,
            3 => NOMCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Run mode 0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NOMCFG_A::VALUE1
    }
    #[doc = "Run mode 1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NOMCFG_A::VALUE2
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == NOMCFG_A::VALUE3
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == NOMCFG_A::VALUE4
    }
}
#[doc = "Field `NOMCFG` writer - Normal Operation Mode Configuration"]
pub type NOMCFG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, NOMCFG_A>;
impl<'a, REG> NOMCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Run mode 0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NOMCFG_A::VALUE1)
    }
    #[doc = "Run mode 1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NOMCFG_A::VALUE2)
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(NOMCFG_A::VALUE3)
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(NOMCFG_A::VALUE4)
    }
}
#[doc = "Bit Protection for NOMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPNOM_AW {
    #[doc = "0: NOMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: NOMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPNOM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPNOM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPNOM` writer - Bit Protection for NOMCFG"]
pub type BPNOM_W<'a, REG> = crate::BitWriter<'a, REG, BPNOM_AW>;
impl<'a, REG> BPNOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BPNOM_AW::VALUE1)
    }
    #[doc = "NOMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BPNOM_AW::VALUE2)
    }
}
#[doc = "Field `SUMCFG` reader - Suspend Mode Configuration"]
pub type SUMCFG_R = crate::FieldReader;
#[doc = "Field `SUMCFG` writer - Suspend Mode Configuration"]
pub type SUMCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Bit Protection for SUMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPSUM_AW {
    #[doc = "0: SUMCFG is not changed."]
    VALUE1 = 0,
    #[doc = "1: SUMCFG is updated with the written value."]
    VALUE2 = 1,
}
impl From<BPSUM_AW> for bool {
    #[inline(always)]
    fn from(variant: BPSUM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPSUM` writer - Bit Protection for SUMCFG"]
pub type BPSUM_W<'a, REG> = crate::BitWriter<'a, REG, BPSUM_AW>;
impl<'a, REG> BPSUM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SUMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BPSUM_AW::VALUE1)
    }
    #[doc = "SUMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BPSUM_AW::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn moden(&self) -> MODEN_R {
        MODEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    pub fn nomcfg(&self) -> NOMCFG_R {
        NOMCFG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn sumcfg(&self) -> SUMCFG_R {
        SUMCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moden(&mut self) -> MODEN_W<KSCFG_SPEC> {
        MODEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit Protection for MODEN"]
    #[inline(always)]
    #[must_use]
    pub fn bpmoden(&mut self) -> BPMODEN_W<KSCFG_SPEC> {
        BPMODEN_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nomcfg(&mut self) -> NOMCFG_W<KSCFG_SPEC> {
        NOMCFG_W::new(self, 4)
    }
    #[doc = "Bit 7 - Bit Protection for NOMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn bpnom(&mut self) -> BPNOM_W<KSCFG_SPEC> {
        BPNOM_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sumcfg(&mut self) -> SUMCFG_W<KSCFG_SPEC> {
        SUMCFG_W::new(self, 8)
    }
    #[doc = "Bit 11 - Bit Protection for SUMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn bpsum(&mut self) -> BPSUM_W<KSCFG_SPEC> {
        BPSUM_W::new(self, 11)
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
#[doc = "Kernel State Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`kscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KSCFG_SPEC;
impl crate::RegisterSpec for KSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kscfg::R`](R) reader structure"]
impl crate::Readable for KSCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`kscfg::W`](W) writer structure"]
impl crate::Writable for KSCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KSCFG to value 0"]
impl crate::Resettable for KSCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
