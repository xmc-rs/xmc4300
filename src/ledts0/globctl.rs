#[doc = "Register `GLOBCTL` reader"]
pub type R = crate::R<GlobctlSpec>;
#[doc = "Register `GLOBCTL` writer"]
pub type W = crate::W<GlobctlSpec>;
#[doc = "Field `TS_EN` reader - Touch-Sense Function Enable"]
pub type TsEnR = crate::BitReader;
#[doc = "Field `TS_EN` writer - Touch-Sense Function Enable"]
pub type TsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LD_EN` reader - LED Function Enable"]
pub type LdEnR = crate::BitReader;
#[doc = "Field `LD_EN` writer - LED Function Enable"]
pub type LdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Master Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmtr {
    #[doc = "0: Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    Value1 = 0,
    #[doc = "1: LEDTS-counter takes its clock from another master kernel"]
    Value2 = 1,
}
impl From<Cmtr> for bool {
    #[inline(always)]
    fn from(variant: Cmtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTR` reader - Clock Master Disable"]
pub type CmtrR = crate::BitReader<Cmtr>;
impl CmtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmtr {
        match self.bits {
            false => Cmtr::Value1,
            true => Cmtr::Value2,
        }
    }
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cmtr::Value1
    }
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cmtr::Value2
    }
}
#[doc = "Field `CMTR` writer - Clock Master Disable"]
pub type CmtrW<'a, REG> = crate::BitWriter<'a, REG, Cmtr>;
impl<'a, REG> CmtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Kernel generates its own clock for LEDTS-counter based on SFR setting"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmtr::Value1)
    }
    #[doc = "LEDTS-counter takes its clock from another master kernel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cmtr::Value2)
    }
}
#[doc = "Enable Autoscan Time Period Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ensync {
    #[doc = "0: No synchronization"]
    Value1 = 0,
    #[doc = "1: Synchronization enabled on Kernel0 autoscan time period"]
    Value2 = 1,
}
impl From<Ensync> for bool {
    #[inline(always)]
    fn from(variant: Ensync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSYNC` reader - Enable Autoscan Time Period Synchronization"]
pub type EnsyncR = crate::BitReader<Ensync>;
impl EnsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ensync {
        match self.bits {
            false => Ensync::Value1,
            true => Ensync::Value2,
        }
    }
    #[doc = "No synchronization"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ensync::Value1
    }
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ensync::Value2
    }
}
#[doc = "Field `ENSYNC` writer - Enable Autoscan Time Period Synchronization"]
pub type EnsyncW<'a, REG> = crate::BitWriter<'a, REG, Ensync>;
impl<'a, REG> EnsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No synchronization"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ensync::Value1)
    }
    #[doc = "Synchronization enabled on Kernel0 autoscan time period"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ensync::Value2)
    }
}
#[doc = "Suspend Request Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suscfg {
    #[doc = "0: Ignore suspend request"]
    Value1 = 0,
    #[doc = "1: Enable suspend according to request"]
    Value2 = 1,
}
impl From<Suscfg> for bool {
    #[inline(always)]
    fn from(variant: Suscfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSCFG` reader - Suspend Request Configuration"]
pub type SuscfgR = crate::BitReader<Suscfg>;
impl SuscfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suscfg {
        match self.bits {
            false => Suscfg::Value1,
            true => Suscfg::Value2,
        }
    }
    #[doc = "Ignore suspend request"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Suscfg::Value1
    }
    #[doc = "Enable suspend according to request"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Suscfg::Value2
    }
}
#[doc = "Field `SUSCFG` writer - Suspend Request Configuration"]
pub type SuscfgW<'a, REG> = crate::BitWriter<'a, REG, Suscfg>;
impl<'a, REG> SuscfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore suspend request"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value1)
    }
    #[doc = "Enable suspend according to request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Suscfg::Value2)
    }
}
#[doc = "Mask Number of LSB Bits for Event Validation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maskval {
    #[doc = "0: Mask LSB bit"]
    Value1 = 0,
    #[doc = "1: Mask 2 LSB bits"]
    Value2 = 1,
    #[doc = "7: Mask 8 LSB bits"]
    Value3 = 7,
}
impl From<Maskval> for u8 {
    #[inline(always)]
    fn from(variant: Maskval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maskval {
    type Ux = u8;
}
impl crate::IsEnum for Maskval {}
#[doc = "Field `MASKVAL` reader - Mask Number of LSB Bits for Event Validation"]
pub type MaskvalR = crate::FieldReader<Maskval>;
impl MaskvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Maskval> {
        match self.bits {
            0 => Some(Maskval::Value1),
            1 => Some(Maskval::Value2),
            7 => Some(Maskval::Value3),
            _ => None,
        }
    }
    #[doc = "Mask LSB bit"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Maskval::Value1
    }
    #[doc = "Mask 2 LSB bits"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Maskval::Value2
    }
    #[doc = "Mask 8 LSB bits"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Maskval::Value3
    }
}
#[doc = "Field `MASKVAL` writer - Mask Number of LSB Bits for Event Validation"]
pub type MaskvalW<'a, REG> = crate::FieldWriter<'a, REG, 3, Maskval>;
impl<'a, REG> MaskvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mask LSB bit"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Maskval::Value1)
    }
    #[doc = "Mask 2 LSB bits"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Maskval::Value2)
    }
    #[doc = "Mask 8 LSB bits"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Maskval::Value3)
    }
}
#[doc = "Enable (Extended) Time Frame Validation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fenval {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Fenval> for bool {
    #[inline(always)]
    fn from(variant: Fenval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FENVAL` reader - Enable (Extended) Time Frame Validation"]
pub type FenvalR = crate::BitReader<Fenval>;
impl FenvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fenval {
        match self.bits {
            false => Fenval::Value1,
            true => Fenval::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fenval::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fenval::Value2
    }
}
#[doc = "Field `FENVAL` writer - Enable (Extended) Time Frame Validation"]
pub type FenvalW<'a, REG> = crate::BitWriter<'a, REG, Fenval>;
impl<'a, REG> FenvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fenval::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fenval::Value2)
    }
}
#[doc = "Enable Time Slice Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItsEn {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<ItsEn> for bool {
    #[inline(always)]
    fn from(variant: ItsEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITS_EN` reader - Enable Time Slice Interrupt"]
pub type ItsEnR = crate::BitReader<ItsEn>;
impl ItsEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ItsEn {
        match self.bits {
            false => ItsEn::Value1,
            true => ItsEn::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ItsEn::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ItsEn::Value2
    }
}
#[doc = "Field `ITS_EN` writer - Enable Time Slice Interrupt"]
pub type ItsEnW<'a, REG> = crate::BitWriter<'a, REG, ItsEn>;
impl<'a, REG> ItsEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ItsEn::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ItsEn::Value2)
    }
}
#[doc = "Enable (Extended) Time Frame Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItfEn {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<ItfEn> for bool {
    #[inline(always)]
    fn from(variant: ItfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITF_EN` reader - Enable (Extended) Time Frame Interrupt"]
pub type ItfEnR = crate::BitReader<ItfEn>;
impl ItfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ItfEn {
        match self.bits {
            false => ItfEn::Value1,
            true => ItfEn::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ItfEn::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ItfEn::Value2
    }
}
#[doc = "Field `ITF_EN` writer - Enable (Extended) Time Frame Interrupt"]
pub type ItfEnW<'a, REG> = crate::BitWriter<'a, REG, ItfEn>;
impl<'a, REG> ItfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ItfEn::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ItfEn::Value2)
    }
}
#[doc = "Enable Autoscan Time Period Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItpEn {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable (valid only for case of hardware-enabled pad turn control)"]
    Value2 = 1,
}
impl From<ItpEn> for bool {
    #[inline(always)]
    fn from(variant: ItpEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITP_EN` reader - Enable Autoscan Time Period Interrupt"]
pub type ItpEnR = crate::BitReader<ItpEn>;
impl ItpEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ItpEn {
        match self.bits {
            false => ItpEn::Value1,
            true => ItpEn::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ItpEn::Value1
    }
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ItpEn::Value2
    }
}
#[doc = "Field `ITP_EN` writer - Enable Autoscan Time Period Interrupt"]
pub type ItpEnW<'a, REG> = crate::BitWriter<'a, REG, ItpEn>;
impl<'a, REG> ItpEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ItpEn::Value1)
    }
    #[doc = "Enable (valid only for case of hardware-enabled pad turn control)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ItpEn::Value2)
    }
}
#[doc = "Field `CLK_PS` reader - LEDTS-Counter Clock Pre-Scale Factor"]
pub type ClkPsR = crate::FieldReader<u16>;
#[doc = "Field `CLK_PS` writer - LEDTS-Counter Clock Pre-Scale Factor"]
pub type ClkPsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline(always)]
    pub fn ts_en(&self) -> TsEnR {
        TsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline(always)]
    pub fn ld_en(&self) -> LdEnR {
        LdEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline(always)]
    pub fn cmtr(&self) -> CmtrR {
        CmtrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline(always)]
    pub fn ensync(&self) -> EnsyncR {
        EnsyncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SuscfgR {
        SuscfgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline(always)]
    pub fn maskval(&self) -> MaskvalR {
        MaskvalR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline(always)]
    pub fn fenval(&self) -> FenvalR {
        FenvalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline(always)]
    pub fn its_en(&self) -> ItsEnR {
        ItsEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline(always)]
    pub fn itf_en(&self) -> ItfEnR {
        ItfEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline(always)]
    pub fn itp_en(&self) -> ItpEnR {
        ItpEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline(always)]
    pub fn clk_ps(&self) -> ClkPsR {
        ClkPsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Touch-Sense Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts_en(&mut self) -> TsEnW<GlobctlSpec> {
        TsEnW::new(self, 0)
    }
    #[doc = "Bit 1 - LED Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ld_en(&mut self) -> LdEnW<GlobctlSpec> {
        LdEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn cmtr(&mut self) -> CmtrW<GlobctlSpec> {
        CmtrW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Autoscan Time Period Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ensync(&mut self) -> EnsyncW<GlobctlSpec> {
        EnsyncW::new(self, 3)
    }
    #[doc = "Bit 8 - Suspend Request Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn suscfg(&mut self) -> SuscfgW<GlobctlSpec> {
        SuscfgW::new(self, 8)
    }
    #[doc = "Bits 9:11 - Mask Number of LSB Bits for Event Validation"]
    #[inline(always)]
    #[must_use]
    pub fn maskval(&mut self) -> MaskvalW<GlobctlSpec> {
        MaskvalW::new(self, 9)
    }
    #[doc = "Bit 12 - Enable (Extended) Time Frame Validation"]
    #[inline(always)]
    #[must_use]
    pub fn fenval(&mut self) -> FenvalW<GlobctlSpec> {
        FenvalW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Time Slice Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn its_en(&mut self) -> ItsEnW<GlobctlSpec> {
        ItsEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable (Extended) Time Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn itf_en(&mut self) -> ItfEnW<GlobctlSpec> {
        ItfEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Autoscan Time Period Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn itp_en(&mut self) -> ItpEnW<GlobctlSpec> {
        ItpEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - LEDTS-Counter Clock Pre-Scale Factor"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ps(&mut self) -> ClkPsW<GlobctlSpec> {
        ClkPsW::new(self, 16)
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`globctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`globctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlobctlSpec;
impl crate::RegisterSpec for GlobctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`globctl::R`](R) reader structure"]
impl crate::Readable for GlobctlSpec {}
#[doc = "`write(|w| ..)` method takes [`globctl::W`](W) writer structure"]
impl crate::Writable for GlobctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLOBCTL to value 0"]
impl crate::Resettable for GlobctlSpec {
    const RESET_VALUE: u32 = 0;
}
