#[doc = "Register `PCR_IISMode` reader"]
pub type R = crate::R<PcrIismodeSpec>;
#[doc = "Register `PCR_IISMode` writer"]
pub type W = crate::W<PcrIismodeSpec>;
#[doc = "WA Generation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wagen {
    #[doc = "0: The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    Value1 = 0,
    #[doc = "1: The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    Value2 = 1,
}
impl From<Wagen> for bool {
    #[inline(always)]
    fn from(variant: Wagen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAGEN` reader - WA Generation Enable"]
pub type WagenR = crate::BitReader<Wagen>;
impl WagenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wagen {
        match self.bits {
            false => Wagen::Value1,
            true => Wagen::Value2,
        }
    }
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wagen::Value1
    }
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wagen::Value2
    }
}
#[doc = "Field `WAGEN` writer - WA Generation Enable"]
pub type WagenW<'a, REG> = crate::BitWriter<'a, REG, Wagen>;
impl<'a, REG> WagenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wagen::Value1)
    }
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wagen::Value2)
    }
}
#[doc = "Data Transfers Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dten {
    #[doc = "0: The changes of the WA input signal are ignored and no transfers take place."]
    Value1 = 0,
    #[doc = "1: Transfers are enabled."]
    Value2 = 1,
}
impl From<Dten> for bool {
    #[inline(always)]
    fn from(variant: Dten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN` reader - Data Transfers Enable"]
pub type DtenR = crate::BitReader<Dten>;
impl DtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dten {
        match self.bits {
            false => Dten::Value1,
            true => Dten::Value2,
        }
    }
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dten::Value1
    }
    #[doc = "Transfers are enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dten::Value2
    }
}
#[doc = "Field `DTEN` writer - Data Transfers Enable"]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG, Dten>;
impl<'a, REG> DtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dten::Value1)
    }
    #[doc = "Transfers are enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dten::Value2)
    }
}
#[doc = "Select Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selinv {
    #[doc = "0: The SELOx outputs have the same polarity as the WA signal."]
    Value1 = 0,
    #[doc = "1: The SELOx outputs have the inverted polarity to the WA signal."]
    Value2 = 1,
}
impl From<Selinv> for bool {
    #[inline(always)]
    fn from(variant: Selinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELINV` reader - Select Inversion"]
pub type SelinvR = crate::BitReader<Selinv>;
impl SelinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selinv {
        match self.bits {
            false => Selinv::Value1,
            true => Selinv::Value2,
        }
    }
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selinv::Value1
    }
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Selinv::Value2
    }
}
#[doc = "Field `SELINV` writer - Select Inversion"]
pub type SelinvW<'a, REG> = crate::BitWriter<'a, REG, Selinv>;
impl<'a, REG> SelinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selinv::Value1)
    }
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selinv::Value2)
    }
}
#[doc = "WA Falling Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wafeien {
    #[doc = "0: A protocol interrupt is not activated if a falling edge of WA is generated."]
    Value1 = 0,
    #[doc = "1: A protocol interrupt is activated if a falling edge of WA is generated."]
    Value2 = 1,
}
impl From<Wafeien> for bool {
    #[inline(always)]
    fn from(variant: Wafeien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAFEIEN` reader - WA Falling Edge Interrupt Enable"]
pub type WafeienR = crate::BitReader<Wafeien>;
impl WafeienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wafeien {
        match self.bits {
            false => Wafeien::Value1,
            true => Wafeien::Value2,
        }
    }
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wafeien::Value1
    }
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wafeien::Value2
    }
}
#[doc = "Field `WAFEIEN` writer - WA Falling Edge Interrupt Enable"]
pub type WafeienW<'a, REG> = crate::BitWriter<'a, REG, Wafeien>;
impl<'a, REG> WafeienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wafeien::Value1)
    }
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wafeien::Value2)
    }
}
#[doc = "WA Rising Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wareien {
    #[doc = "0: A protocol interrupt is not activated if a rising edge of WA is generated."]
    Value1 = 0,
    #[doc = "1: A protocol interrupt is activated if a rising edge of WA is generated."]
    Value2 = 1,
}
impl From<Wareien> for bool {
    #[inline(always)]
    fn from(variant: Wareien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAREIEN` reader - WA Rising Edge Interrupt Enable"]
pub type WareienR = crate::BitReader<Wareien>;
impl WareienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wareien {
        match self.bits {
            false => Wareien::Value1,
            true => Wareien::Value2,
        }
    }
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wareien::Value1
    }
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wareien::Value2
    }
}
#[doc = "Field `WAREIEN` writer - WA Rising Edge Interrupt Enable"]
pub type WareienW<'a, REG> = crate::BitWriter<'a, REG, Wareien>;
impl<'a, REG> WareienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wareien::Value1)
    }
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wareien::Value2)
    }
}
#[doc = "END Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endien {
    #[doc = "0: A protocol interrupt is not activated."]
    Value1 = 0,
    #[doc = "1: A protocol interrupt is activated."]
    Value2 = 1,
}
impl From<Endien> for bool {
    #[inline(always)]
    fn from(variant: Endien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDIEN` reader - END Interrupt Enable"]
pub type EndienR = crate::BitReader<Endien>;
impl EndienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endien {
        match self.bits {
            false => Endien::Value1,
            true => Endien::Value2,
        }
    }
    #[doc = "A protocol interrupt is not activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Endien::Value1
    }
    #[doc = "A protocol interrupt is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Endien::Value2
    }
}
#[doc = "Field `ENDIEN` writer - END Interrupt Enable"]
pub type EndienW<'a, REG> = crate::BitWriter<'a, REG, Endien>;
impl<'a, REG> EndienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Endien::Value1)
    }
    #[doc = "A protocol interrupt is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Endien::Value2)
    }
}
#[doc = "DX2T Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dx2tien {
    #[doc = "0: A protocol interrupt is not generated if DX2T is active."]
    Value1 = 0,
    #[doc = "1: A protocol interrupt is generated if DX2T is active."]
    Value2 = 1,
}
impl From<Dx2tien> for bool {
    #[inline(always)]
    fn from(variant: Dx2tien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DX2TIEN` reader - DX2T Interrupt Enable"]
pub type Dx2tienR = crate::BitReader<Dx2tien>;
impl Dx2tienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dx2tien {
        match self.bits {
            false => Dx2tien::Value1,
            true => Dx2tien::Value2,
        }
    }
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dx2tien::Value1
    }
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dx2tien::Value2
    }
}
#[doc = "Field `DX2TIEN` writer - DX2T Interrupt Enable"]
pub type Dx2tienW<'a, REG> = crate::BitWriter<'a, REG, Dx2tien>;
impl<'a, REG> Dx2tienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2tien::Value1)
    }
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2tien::Value2)
    }
}
#[doc = "Field `TDEL` reader - Transfer Delay"]
pub type TdelR = crate::FieldReader;
#[doc = "Field `TDEL` writer - Transfer Delay"]
pub type TdelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclk {
    #[doc = "0: The MCLK generation is disabled and MCLK is 0."]
    Value1 = 0,
    #[doc = "1: The MCLK generation is enabled."]
    Value2 = 1,
}
impl From<Mclk> for bool {
    #[inline(always)]
    fn from(variant: Mclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub type MclkR = crate::BitReader<Mclk>;
impl MclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclk {
        match self.bits {
            false => Mclk::Value1,
            true => Mclk::Value2,
        }
    }
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mclk::Value1
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mclk::Value2
    }
}
#[doc = "Field `MCLK` writer - Master Clock Enable"]
pub type MclkW<'a, REG> = crate::BitWriter<'a, REG, Mclk>;
impl<'a, REG> MclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mclk::Value1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mclk::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline(always)]
    pub fn wagen(&self) -> WagenR {
        WagenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&self) -> SelinvR {
        SelinvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wafeien(&self) -> WafeienR {
        WafeienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wareien(&self) -> WareienR {
        WareienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline(always)]
    pub fn endien(&self) -> EndienR {
        EndienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&self) -> Dx2tienR {
        Dx2tienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline(always)]
    pub fn tdel(&self) -> TdelR {
        TdelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MclkR {
        MclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wagen(&mut self) -> WagenW<PcrIismodeSpec> {
        WagenW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DtenW<PcrIismodeSpec> {
        DtenW::new(self, 1)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn selinv(&mut self) -> SelinvW<PcrIismodeSpec> {
        SelinvW::new(self, 2)
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wafeien(&mut self) -> WafeienW<PcrIismodeSpec> {
        WafeienW::new(self, 4)
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wareien(&mut self) -> WareienW<PcrIismodeSpec> {
        WareienW::new(self, 5)
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endien(&mut self) -> EndienW<PcrIismodeSpec> {
        EndienW::new(self, 6)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dx2tien(&mut self) -> Dx2tienW<PcrIismodeSpec> {
        Dx2tienW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tdel(&mut self) -> TdelW<PcrIismodeSpec> {
        TdelW::new(self, 16)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MclkW<PcrIismodeSpec> {
        MclkW::new(self, 31)
    }
}
#[doc = "Protocol Control Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iismode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iismode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrIismodeSpec;
impl crate::RegisterSpec for PcrIismodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr_iismode::R`](R) reader structure"]
impl crate::Readable for PcrIismodeSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr_iismode::W`](W) writer structure"]
impl crate::Writable for PcrIismodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR_IISMode to value 0"]
impl crate::Resettable for PcrIismodeSpec {
    const RESET_VALUE: u32 = 0;
}
