#[doc = "Register `NIPR` reader"]
pub type R = crate::R<NiprSpec>;
#[doc = "Register `NIPR` writer"]
pub type W = crate::W<NiprSpec>;
#[doc = "Alert Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Alinp {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    Value1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    Value2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    Value3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    Value4 = 15,
}
impl From<Alinp> for u8 {
    #[inline(always)]
    fn from(variant: Alinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Alinp {
    type Ux = u8;
}
impl crate::IsEnum for Alinp {}
#[doc = "Field `ALINP` reader - Alert Interrupt Node Pointer"]
pub type AlinpR = crate::FieldReader<Alinp>;
impl AlinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Alinp> {
        match self.bits {
            0 => Some(Alinp::Value1),
            1 => Some(Alinp::Value2),
            14 => Some(Alinp::Value3),
            15 => Some(Alinp::Value4),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Alinp::Value1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Alinp::Value2
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Alinp::Value3
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Alinp::Value4
    }
}
#[doc = "Field `ALINP` writer - Alert Interrupt Node Pointer"]
pub type AlinpW<'a, REG> = crate::FieldWriter<'a, REG, 4, Alinp>;
impl<'a, REG> AlinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Alinp::Value1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Alinp::Value2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Alinp::Value3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Alinp::Value4)
    }
}
#[doc = "Last Error Code Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lecinp {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    Value1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    Value2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    Value3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    Value4 = 15,
}
impl From<Lecinp> for u8 {
    #[inline(always)]
    fn from(variant: Lecinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lecinp {
    type Ux = u8;
}
impl crate::IsEnum for Lecinp {}
#[doc = "Field `LECINP` reader - Last Error Code Interrupt Node Pointer"]
pub type LecinpR = crate::FieldReader<Lecinp>;
impl LecinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lecinp> {
        match self.bits {
            0 => Some(Lecinp::Value1),
            1 => Some(Lecinp::Value2),
            14 => Some(Lecinp::Value3),
            15 => Some(Lecinp::Value4),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lecinp::Value1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lecinp::Value2
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lecinp::Value3
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lecinp::Value4
    }
}
#[doc = "Field `LECINP` writer - Last Error Code Interrupt Node Pointer"]
pub type LecinpW<'a, REG> = crate::FieldWriter<'a, REG, 4, Lecinp>;
impl<'a, REG> LecinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lecinp::Value1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lecinp::Value2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Lecinp::Value3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Lecinp::Value4)
    }
}
#[doc = "Transfer OK Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trinp {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    Value1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    Value2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    Value3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    Value4 = 15,
}
impl From<Trinp> for u8 {
    #[inline(always)]
    fn from(variant: Trinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trinp {
    type Ux = u8;
}
impl crate::IsEnum for Trinp {}
#[doc = "Field `TRINP` reader - Transfer OK Interrupt Node Pointer"]
pub type TrinpR = crate::FieldReader<Trinp>;
impl TrinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trinp> {
        match self.bits {
            0 => Some(Trinp::Value1),
            1 => Some(Trinp::Value2),
            14 => Some(Trinp::Value3),
            15 => Some(Trinp::Value4),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trinp::Value1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trinp::Value2
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Trinp::Value3
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Trinp::Value4
    }
}
#[doc = "Field `TRINP` writer - Transfer OK Interrupt Node Pointer"]
pub type TrinpW<'a, REG> = crate::FieldWriter<'a, REG, 4, Trinp>;
impl<'a, REG> TrinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trinp::Value1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trinp::Value2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Trinp::Value3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Trinp::Value4)
    }
}
#[doc = "Frame Counter Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfcinp {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    Value1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    Value2 = 1,
    #[doc = "14: Interrupt output line INT_O14 is selected."]
    Value3 = 14,
    #[doc = "15: Interrupt output line INT_O15 is selected."]
    Value4 = 15,
}
impl From<Cfcinp> for u8 {
    #[inline(always)]
    fn from(variant: Cfcinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfcinp {
    type Ux = u8;
}
impl crate::IsEnum for Cfcinp {}
#[doc = "Field `CFCINP` reader - Frame Counter Interrupt Node Pointer"]
pub type CfcinpR = crate::FieldReader<Cfcinp>;
impl CfcinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfcinp> {
        match self.bits {
            0 => Some(Cfcinp::Value1),
            1 => Some(Cfcinp::Value2),
            14 => Some(Cfcinp::Value3),
            15 => Some(Cfcinp::Value4),
            _ => None,
        }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfcinp::Value1
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfcinp::Value2
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cfcinp::Value3
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cfcinp::Value4
    }
}
#[doc = "Field `CFCINP` writer - Frame Counter Interrupt Node Pointer"]
pub type CfcinpW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cfcinp>;
impl<'a, REG> CfcinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcinp::Value1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcinp::Value2)
    }
    #[doc = "Interrupt output line INT_O14 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcinp::Value3)
    }
    #[doc = "Interrupt output line INT_O15 is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcinp::Value4)
    }
}
impl R {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&self) -> AlinpR {
        AlinpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&self) -> LecinpR {
        LecinpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&self) -> TrinpR {
        TrinpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CfcinpR {
        CfcinpR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn alinp(&mut self) -> AlinpW<NiprSpec> {
        AlinpW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn lecinp(&mut self) -> LecinpW<NiprSpec> {
        LecinpW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn trinp(&mut self) -> TrinpW<NiprSpec> {
        TrinpW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn cfcinp(&mut self) -> CfcinpW<NiprSpec> {
        CfcinpW::new(self, 12)
    }
}
#[doc = "Node Interrupt Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NiprSpec;
impl crate::RegisterSpec for NiprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nipr::R`](R) reader structure"]
impl crate::Readable for NiprSpec {}
#[doc = "`write(|w| ..)` method takes [`nipr::W`](W) writer structure"]
impl crate::Writable for NiprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NIPR to value 0"]
impl crate::Resettable for NiprSpec {
    const RESET_VALUE: u32 = 0;
}
