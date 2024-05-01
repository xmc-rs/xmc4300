#[doc = "Register `KSCFG` reader"]
pub type R = crate::R<KscfgSpec>;
#[doc = "Register `KSCFG` writer"]
pub type W = crate::W<KscfgSpec>;
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moden {
    #[doc = "0: The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    Value1 = 0,
    #[doc = "1: The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    Value2 = 1,
}
impl From<Moden> for bool {
    #[inline(always)]
    fn from(variant: Moden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODEN` reader - Module Enable"]
pub type ModenR = crate::BitReader<Moden>;
impl ModenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moden {
        match self.bits {
            false => Moden::Value1,
            true => Moden::Value2,
        }
    }
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Moden::Value1
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Moden::Value2
    }
}
#[doc = "Field `MODEN` writer - Module Enable"]
pub type ModenW<'a, REG> = crate::BitWriter<'a, REG, Moden>;
impl<'a, REG> ModenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The module is switched off immediately (without respecting a stop condition). It does not react on mode control actions and the module clock is switched off. The module does not react on read accesses and ignores write accesses (except to KSCFG)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Moden::Value1)
    }
    #[doc = "The module is switched on and can operate. After writing 1 to MODEN, it is recommended to read register KSCFG to avoid pipeline effects in the control block before accessing other Service Request Processing registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Moden::Value2)
    }
}
#[doc = "Bit Protection for MODEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpmoden {
    #[doc = "0: MODEN is not changed."]
    Value1 = 0,
    #[doc = "1: MODEN is updated with the written value."]
    Value2 = 1,
}
impl From<Bpmoden> for bool {
    #[inline(always)]
    fn from(variant: Bpmoden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPMODEN` writer - Bit Protection for MODEN"]
pub type BpmodenW<'a, REG> = crate::BitWriter<'a, REG, Bpmoden>;
impl<'a, REG> BpmodenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MODEN is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpmoden::Value1)
    }
    #[doc = "MODEN is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bpmoden::Value2)
    }
}
#[doc = "Normal Operation Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nomcfg {
    #[doc = "0: Run mode 0 is selected."]
    Value1 = 0,
    #[doc = "1: Run mode 1 is selected."]
    Value2 = 1,
    #[doc = "2: Stop mode 0 is selected."]
    Value3 = 2,
    #[doc = "3: Stop mode 1 is selected."]
    Value4 = 3,
}
impl From<Nomcfg> for u8 {
    #[inline(always)]
    fn from(variant: Nomcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nomcfg {
    type Ux = u8;
}
impl crate::IsEnum for Nomcfg {}
#[doc = "Field `NOMCFG` reader - Normal Operation Mode Configuration"]
pub type NomcfgR = crate::FieldReader<Nomcfg>;
impl NomcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nomcfg {
        match self.bits {
            0 => Nomcfg::Value1,
            1 => Nomcfg::Value2,
            2 => Nomcfg::Value3,
            3 => Nomcfg::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Run mode 0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Nomcfg::Value1
    }
    #[doc = "Run mode 1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Nomcfg::Value2
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Nomcfg::Value3
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Nomcfg::Value4
    }
}
#[doc = "Field `NOMCFG` writer - Normal Operation Mode Configuration"]
pub type NomcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nomcfg, crate::Safe>;
impl<'a, REG> NomcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Run mode 0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Nomcfg::Value1)
    }
    #[doc = "Run mode 1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Nomcfg::Value2)
    }
    #[doc = "Stop mode 0 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Nomcfg::Value3)
    }
    #[doc = "Stop mode 1 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Nomcfg::Value4)
    }
}
#[doc = "Bit Protection for NOMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpnom {
    #[doc = "0: NOMCFG is not changed."]
    Value1 = 0,
    #[doc = "1: NOMCFG is updated with the written value."]
    Value2 = 1,
}
impl From<Bpnom> for bool {
    #[inline(always)]
    fn from(variant: Bpnom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPNOM` writer - Bit Protection for NOMCFG"]
pub type BpnomW<'a, REG> = crate::BitWriter<'a, REG, Bpnom>;
impl<'a, REG> BpnomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpnom::Value1)
    }
    #[doc = "NOMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bpnom::Value2)
    }
}
#[doc = "Field `SUMCFG` reader - Suspend Mode Configuration"]
pub type SumcfgR = crate::FieldReader;
#[doc = "Field `SUMCFG` writer - Suspend Mode Configuration"]
pub type SumcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Bit Protection for SUMCFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpsum {
    #[doc = "0: SUMCFG is not changed."]
    Value1 = 0,
    #[doc = "1: SUMCFG is updated with the written value."]
    Value2 = 1,
}
impl From<Bpsum> for bool {
    #[inline(always)]
    fn from(variant: Bpsum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPSUM` writer - Bit Protection for SUMCFG"]
pub type BpsumW<'a, REG> = crate::BitWriter<'a, REG, Bpsum>;
impl<'a, REG> BpsumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SUMCFG is not changed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpsum::Value1)
    }
    #[doc = "SUMCFG is updated with the written value."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bpsum::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn moden(&self) -> ModenR {
        ModenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    pub fn nomcfg(&self) -> NomcfgR {
        NomcfgR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    pub fn sumcfg(&self) -> SumcfgR {
        SumcfgR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moden(&mut self) -> ModenW<KscfgSpec> {
        ModenW::new(self, 0)
    }
    #[doc = "Bit 1 - Bit Protection for MODEN"]
    #[inline(always)]
    #[must_use]
    pub fn bpmoden(&mut self) -> BpmodenW<KscfgSpec> {
        BpmodenW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Normal Operation Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nomcfg(&mut self) -> NomcfgW<KscfgSpec> {
        NomcfgW::new(self, 4)
    }
    #[doc = "Bit 7 - Bit Protection for NOMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn bpnom(&mut self) -> BpnomW<KscfgSpec> {
        BpnomW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Suspend Mode Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sumcfg(&mut self) -> SumcfgW<KscfgSpec> {
        SumcfgW::new(self, 8)
    }
    #[doc = "Bit 11 - Bit Protection for SUMCFG"]
    #[inline(always)]
    #[must_use]
    pub fn bpsum(&mut self) -> BpsumW<KscfgSpec> {
        BpsumW::new(self, 11)
    }
}
#[doc = "Kernel State Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`kscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KscfgSpec;
impl crate::RegisterSpec for KscfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kscfg::R`](R) reader structure"]
impl crate::Readable for KscfgSpec {}
#[doc = "`write(|w| ..)` method takes [`kscfg::W`](W) writer structure"]
impl crate::Writable for KscfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KSCFG to value 0"]
impl crate::Resettable for KscfgSpec {
    const RESET_VALUE: u32 = 0;
}
