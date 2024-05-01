#[doc = "Register `GLOBTF` reader"]
pub type R = crate::R<GlobtfSpec>;
#[doc = "Register `GLOBTF` writer"]
pub type W = crate::W<GlobtfSpec>;
#[doc = "Field `CDGR` reader - Converter Diagnostics Group"]
pub type CdgrR = crate::FieldReader;
#[doc = "Field `CDGR` writer - Converter Diagnostics Group"]
pub type CdgrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Converter Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cden {
    #[doc = "0: All diagnostic pull devices are disconnected"]
    Value1 = 0,
    #[doc = "1: Diagnostic pull devices connected as selected by bitfield CDSEL"]
    Value2 = 1,
}
impl From<Cden> for bool {
    #[inline(always)]
    fn from(variant: Cden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDEN` reader - Converter Diagnostics Enable"]
pub type CdenR = crate::BitReader<Cden>;
impl CdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cden {
        match self.bits {
            false => Cden::Value1,
            true => Cden::Value2,
        }
    }
    #[doc = "All diagnostic pull devices are disconnected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cden::Value1
    }
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cden::Value2
    }
}
#[doc = "Field `CDEN` writer - Converter Diagnostics Enable"]
pub type CdenW<'a, REG> = crate::BitWriter<'a, REG, Cden>;
impl<'a, REG> CdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All diagnostic pull devices are disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cden::Value1)
    }
    #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cden::Value2)
    }
}
#[doc = "Converter Diagnostics Pull-Devices Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cdsel {
    #[doc = "0: Connected to VAREF"]
    Value1 = 0,
    #[doc = "1: Connected to VAGND"]
    Value2 = 1,
    #[doc = "2: Connected to 1/3rd VAREF"]
    Value3 = 2,
    #[doc = "3: Connected to 2/3rd VAREF"]
    Value4 = 3,
}
impl From<Cdsel> for u8 {
    #[inline(always)]
    fn from(variant: Cdsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cdsel {
    type Ux = u8;
}
impl crate::IsEnum for Cdsel {}
#[doc = "Field `CDSEL` reader - Converter Diagnostics Pull-Devices Select"]
pub type CdselR = crate::FieldReader<Cdsel>;
impl CdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdsel {
        match self.bits {
            0 => Cdsel::Value1,
            1 => Cdsel::Value2,
            2 => Cdsel::Value3,
            3 => Cdsel::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Connected to VAREF"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cdsel::Value1
    }
    #[doc = "Connected to VAGND"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cdsel::Value2
    }
    #[doc = "Connected to 1/3rd VAREF"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cdsel::Value3
    }
    #[doc = "Connected to 2/3rd VAREF"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cdsel::Value4
    }
}
#[doc = "Field `CDSEL` writer - Converter Diagnostics Pull-Devices Select"]
pub type CdselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cdsel, crate::Safe>;
impl<'a, REG> CdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Connected to VAREF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsel::Value1)
    }
    #[doc = "Connected to VAGND"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsel::Value2)
    }
    #[doc = "Connected to 1/3rd VAREF"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsel::Value3)
    }
    #[doc = "Connected to 2/3rd VAREF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cdsel::Value4)
    }
}
#[doc = "Write Control for Conversion Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdwc {
    #[doc = "0: No write access to parameters"]
    Value1 = 0,
    #[doc = "1: Bitfields CDSEL, CDEN, CDGR can be written"]
    Value2 = 1,
}
impl From<Cdwc> for bool {
    #[inline(always)]
    fn from(variant: Cdwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDWC` writer - Write Control for Conversion Diagnostics"]
pub type CdwcW<'a, REG> = crate::BitWriter<'a, REG, Cdwc>;
impl<'a, REG> CdwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cdwc::Value1)
    }
    #[doc = "Bitfields CDSEL, CDEN, CDGR can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cdwc::Value2)
    }
}
#[doc = "Pull-Down Diagnostics Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdd {
    #[doc = "0: Disconnected"]
    Value1 = 0,
    #[doc = "1: The pull-down diagnostics device is active"]
    Value2 = 1,
}
impl From<Pdd> for bool {
    #[inline(always)]
    fn from(variant: Pdd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDD` reader - Pull-Down Diagnostics Enable"]
pub type PddR = crate::BitReader<Pdd>;
impl PddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdd {
        match self.bits {
            false => Pdd::Value1,
            true => Pdd::Value2,
        }
    }
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdd::Value1
    }
    #[doc = "The pull-down diagnostics device is active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdd::Value2
    }
}
#[doc = "Field `PDD` writer - Pull-Down Diagnostics Enable"]
pub type PddW<'a, REG> = crate::BitWriter<'a, REG, Pdd>;
impl<'a, REG> PddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disconnected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdd::Value1)
    }
    #[doc = "The pull-down diagnostics device is active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdd::Value2)
    }
}
#[doc = "Write Control for Multiplexer Diagnostics\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdwc {
    #[doc = "0: No write access to parameters"]
    Value1 = 0,
    #[doc = "1: Bitfield PDD can be written"]
    Value2 = 1,
}
impl From<Mdwc> for bool {
    #[inline(always)]
    fn from(variant: Mdwc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDWC` writer - Write Control for Multiplexer Diagnostics"]
pub type MdwcW<'a, REG> = crate::BitWriter<'a, REG, Mdwc>;
impl<'a, REG> MdwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to parameters"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mdwc::Value1)
    }
    #[doc = "Bitfield PDD can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mdwc::Value2)
    }
}
impl R {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    pub fn cdgr(&self) -> CdgrR {
        CdgrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    pub fn cden(&self) -> CdenR {
        CdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    pub fn cdsel(&self) -> CdselR {
        CdselR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    pub fn pdd(&self) -> PddR {
        PddR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Converter Diagnostics Group"]
    #[inline(always)]
    #[must_use]
    pub fn cdgr(&mut self) -> CdgrW<GlobtfSpec> {
        CdgrW::new(self, 4)
    }
    #[doc = "Bit 8 - Converter Diagnostics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cden(&mut self) -> CdenW<GlobtfSpec> {
        CdenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Converter Diagnostics Pull-Devices Select"]
    #[inline(always)]
    #[must_use]
    pub fn cdsel(&mut self) -> CdselW<GlobtfSpec> {
        CdselW::new(self, 9)
    }
    #[doc = "Bit 15 - Write Control for Conversion Diagnostics"]
    #[inline(always)]
    #[must_use]
    pub fn cdwc(&mut self) -> CdwcW<GlobtfSpec> {
        CdwcW::new(self, 15)
    }
    #[doc = "Bit 16 - Pull-Down Diagnostics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdd(&mut self) -> PddW<GlobtfSpec> {
        PddW::new(self, 16)
    }
    #[doc = "Bit 23 - Write Control for Multiplexer Diagnostics"]
    #[inline(always)]
    #[must_use]
    pub fn mdwc(&mut self) -> MdwcW<GlobtfSpec> {
        MdwcW::new(self, 23)
    }
}
#[doc = "Global Test Functions Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globtf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globtf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobtfSpec;
impl crate::RegisterSpec for GlobtfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globtf::R`](R) reader structure"]
impl crate::Readable for GlobtfSpec {}
#[doc = "`write(|w| ..)` method takes [`globtf::W`](W) writer structure"]
impl crate::Writable for GlobtfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBTF to value 0"]
impl crate::Resettable for GlobtfSpec {
    const RESET_VALUE: u32 = 0;
}
