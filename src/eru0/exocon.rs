#[doc = "Register `EXOCON[%s]` reader"]
pub type R = crate::R<ExoconSpec>;
#[doc = "Register `EXOCON[%s]` writer"]
pub type W = crate::W<ExoconSpec>;
#[doc = "Internal Trigger Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iss {
    #[doc = "0: The peripheral trigger function is disabled"]
    Value1 = 0,
    #[doc = "1: Input ERU_OGUy1 is selected"]
    Value2 = 1,
    #[doc = "2: Input ERU_OGUy2 is selected"]
    Value3 = 2,
    #[doc = "3: Input ERU_OGUy3 is selected"]
    Value4 = 3,
}
impl From<Iss> for u8 {
    #[inline(always)]
    fn from(variant: Iss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iss {
    type Ux = u8;
}
impl crate::IsEnum for Iss {}
#[doc = "Field `ISS` reader - Internal Trigger Source Selection"]
pub type IssR = crate::FieldReader<Iss>;
impl IssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iss {
        match self.bits {
            0 => Iss::Value1,
            1 => Iss::Value2,
            2 => Iss::Value3,
            3 => Iss::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "The peripheral trigger function is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Iss::Value1
    }
    #[doc = "Input ERU_OGUy1 is selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Iss::Value2
    }
    #[doc = "Input ERU_OGUy2 is selected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Iss::Value3
    }
    #[doc = "Input ERU_OGUy3 is selected"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Iss::Value4
    }
}
#[doc = "Field `ISS` writer - Internal Trigger Source Selection"]
pub type IssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iss, crate::Safe>;
impl<'a, REG> IssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The peripheral trigger function is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Iss::Value1)
    }
    #[doc = "Input ERU_OGUy1 is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Iss::Value2)
    }
    #[doc = "Input ERU_OGUy2 is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Iss::Value3)
    }
    #[doc = "Input ERU_OGUy3 is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Iss::Value4)
    }
}
#[doc = "Gating Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Geen {
    #[doc = "0: The event detection is disabled"]
    Value1 = 0,
    #[doc = "1: The event detection is enabled"]
    Value2 = 1,
}
impl From<Geen> for bool {
    #[inline(always)]
    fn from(variant: Geen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GEEN` reader - Gating Event Enable"]
pub type GeenR = crate::BitReader<Geen>;
impl GeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Geen {
        match self.bits {
            false => Geen::Value1,
            true => Geen::Value2,
        }
    }
    #[doc = "The event detection is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Geen::Value1
    }
    #[doc = "The event detection is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Geen::Value2
    }
}
#[doc = "Field `GEEN` writer - Gating Event Enable"]
pub type GeenW<'a, REG> = crate::BitWriter<'a, REG, Geen>;
impl<'a, REG> GeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event detection is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Geen::Value1)
    }
    #[doc = "The event detection is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Geen::Value2)
    }
}
#[doc = "Pattern Detection Result Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr {
    #[doc = "0: A pattern miss is detected"]
    Value1 = 0,
    #[doc = "1: A pattern match is detected"]
    Value2 = 1,
}
impl From<Pdr> for bool {
    #[inline(always)]
    fn from(variant: Pdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR` reader - Pattern Detection Result Flag"]
pub type PdrR = crate::BitReader<Pdr>;
impl PdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr {
        match self.bits {
            false => Pdr::Value1,
            true => Pdr::Value2,
        }
    }
    #[doc = "A pattern miss is detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdr::Value1
    }
    #[doc = "A pattern match is detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdr::Value2
    }
}
#[doc = "Gating Selection for Pattern Detection Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gp {
    #[doc = "0: ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    Value1 = 0,
    #[doc = "1: ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    Value2 = 1,
    #[doc = "2: ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    Value3 = 2,
    #[doc = "3: ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    Value4 = 3,
}
impl From<Gp> for u8 {
    #[inline(always)]
    fn from(variant: Gp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gp {
    type Ux = u8;
}
impl crate::IsEnum for Gp {}
#[doc = "Field `GP` reader - Gating Selection for Pattern Detection Result"]
pub type GpR = crate::FieldReader<Gp>;
impl GpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gp {
        match self.bits {
            0 => Gp::Value1,
            1 => Gp::Value2,
            2 => Gp::Value3,
            3 => Gp::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gp::Value1
    }
    #[doc = "ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gp::Value2
    }
    #[doc = "ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Gp::Value3
    }
    #[doc = "ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Gp::Value4
    }
}
#[doc = "Field `GP` writer - Gating Selection for Pattern Detection Result"]
pub type GpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gp, crate::Safe>;
impl<'a, REG> GpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ERU_GOUTy is always disabled and ERU_IOUTy can not be activated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gp::Value1)
    }
    #[doc = "ERU_GOUTy is always enabled and ERU_IOUTy becomes activated with each activation of ERU_TOUTy"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gp::Value2)
    }
    #[doc = "ERU_GOUTy is equal to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is detected (pattern match PDR = 1)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Gp::Value3)
    }
    #[doc = "ERU_GOUTy is inverted to ERU_PDOUTy and ERU_IOUTy becomes activated with an activation of ERU_TOUTy while the desired pattern is not detected (pattern miss PDR = 0)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Gp::Value4)
    }
}
#[doc = "Pattern Detection Enable for ETL0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipen0 {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    Value1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    Value2 = 1,
}
impl From<Ipen0> for bool {
    #[inline(always)]
    fn from(variant: Ipen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPEN0` reader - Pattern Detection Enable for ETL0"]
pub type Ipen0R = crate::BitReader<Ipen0>;
impl Ipen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipen0 {
        match self.bits {
            false => Ipen0::Value1,
            true => Ipen0::Value2,
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ipen0::Value1
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ipen0::Value2
    }
}
#[doc = "Field `IPEN0` writer - Pattern Detection Enable for ETL0"]
pub type Ipen0W<'a, REG> = crate::BitWriter<'a, REG, Ipen0>;
impl<'a, REG> Ipen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen0::Value1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen0::Value2)
    }
}
#[doc = "Pattern Detection Enable for ETL1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipen1 {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    Value1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    Value2 = 1,
}
impl From<Ipen1> for bool {
    #[inline(always)]
    fn from(variant: Ipen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPEN1` reader - Pattern Detection Enable for ETL1"]
pub type Ipen1R = crate::BitReader<Ipen1>;
impl Ipen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipen1 {
        match self.bits {
            false => Ipen1::Value1,
            true => Ipen1::Value2,
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ipen1::Value1
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ipen1::Value2
    }
}
#[doc = "Field `IPEN1` writer - Pattern Detection Enable for ETL1"]
pub type Ipen1W<'a, REG> = crate::BitWriter<'a, REG, Ipen1>;
impl<'a, REG> Ipen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen1::Value1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen1::Value2)
    }
}
#[doc = "Pattern Detection Enable for ETL2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipen2 {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    Value1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    Value2 = 1,
}
impl From<Ipen2> for bool {
    #[inline(always)]
    fn from(variant: Ipen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPEN2` reader - Pattern Detection Enable for ETL2"]
pub type Ipen2R = crate::BitReader<Ipen2>;
impl Ipen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipen2 {
        match self.bits {
            false => Ipen2::Value1,
            true => Ipen2::Value2,
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ipen2::Value1
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ipen2::Value2
    }
}
#[doc = "Field `IPEN2` writer - Pattern Detection Enable for ETL2"]
pub type Ipen2W<'a, REG> = crate::BitWriter<'a, REG, Ipen2>;
impl<'a, REG> Ipen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen2::Value1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen2::Value2)
    }
}
#[doc = "Pattern Detection Enable for ETL3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipen3 {
    #[doc = "0: Flag EXICONx.FL is excluded from the pattern detection"]
    Value1 = 0,
    #[doc = "1: Flag EXICONx.FL is included in the pattern detection"]
    Value2 = 1,
}
impl From<Ipen3> for bool {
    #[inline(always)]
    fn from(variant: Ipen3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPEN3` reader - Pattern Detection Enable for ETL3"]
pub type Ipen3R = crate::BitReader<Ipen3>;
impl Ipen3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipen3 {
        match self.bits {
            false => Ipen3::Value1,
            true => Ipen3::Value2,
        }
    }
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ipen3::Value1
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ipen3::Value2
    }
}
#[doc = "Field `IPEN3` writer - Pattern Detection Enable for ETL3"]
pub type Ipen3W<'a, REG> = crate::BitWriter<'a, REG, Ipen3>;
impl<'a, REG> Ipen3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flag EXICONx.FL is excluded from the pattern detection"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen3::Value1)
    }
    #[doc = "Flag EXICONx.FL is included in the pattern detection"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ipen3::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline(always)]
    pub fn iss(&self) -> IssR {
        IssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline(always)]
    pub fn geen(&self) -> GeenR {
        GeenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pattern Detection Result Flag"]
    #[inline(always)]
    pub fn pdr(&self) -> PdrR {
        PdrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline(always)]
    pub fn gp(&self) -> GpR {
        GpR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline(always)]
    pub fn ipen0(&self) -> Ipen0R {
        Ipen0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline(always)]
    pub fn ipen1(&self) -> Ipen1R {
        Ipen1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline(always)]
    pub fn ipen2(&self) -> Ipen2R {
        Ipen2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline(always)]
    pub fn ipen3(&self) -> Ipen3R {
        Ipen3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Internal Trigger Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn iss(&mut self) -> IssW<ExoconSpec> {
        IssW::new(self, 0)
    }
    #[doc = "Bit 2 - Gating Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn geen(&mut self) -> GeenW<ExoconSpec> {
        GeenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Gating Selection for Pattern Detection Result"]
    #[inline(always)]
    #[must_use]
    pub fn gp(&mut self) -> GpW<ExoconSpec> {
        GpW::new(self, 4)
    }
    #[doc = "Bit 12 - Pattern Detection Enable for ETL0"]
    #[inline(always)]
    #[must_use]
    pub fn ipen0(&mut self) -> Ipen0W<ExoconSpec> {
        Ipen0W::new(self, 12)
    }
    #[doc = "Bit 13 - Pattern Detection Enable for ETL1"]
    #[inline(always)]
    #[must_use]
    pub fn ipen1(&mut self) -> Ipen1W<ExoconSpec> {
        Ipen1W::new(self, 13)
    }
    #[doc = "Bit 14 - Pattern Detection Enable for ETL2"]
    #[inline(always)]
    #[must_use]
    pub fn ipen2(&mut self) -> Ipen2W<ExoconSpec> {
        Ipen2W::new(self, 14)
    }
    #[doc = "Bit 15 - Pattern Detection Enable for ETL3"]
    #[inline(always)]
    #[must_use]
    pub fn ipen3(&mut self) -> Ipen3W<ExoconSpec> {
        Ipen3W::new(self, 15)
    }
}
#[doc = "Event Output Trigger Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exocon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exocon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExoconSpec;
impl crate::RegisterSpec for ExoconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exocon::R`](R) reader structure"]
impl crate::Readable for ExoconSpec {}
#[doc = "`write(|w| ..)` method takes [`exocon::W`](W) writer structure"]
impl crate::Writable for ExoconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXOCON[%s]
to value 0x08"]
impl crate::Resettable for ExoconSpec {
    const RESET_VALUE: u32 = 0x08;
}
