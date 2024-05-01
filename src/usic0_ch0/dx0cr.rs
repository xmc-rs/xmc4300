#[doc = "Register `DX0CR` reader"]
pub type R = crate::R<Dx0crSpec>;
#[doc = "Register `DX0CR` writer"]
pub type W = crate::W<Dx0crSpec>;
#[doc = "Data Selection for Input Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsel {
    #[doc = "0: The data input DXnA is selected."]
    Value1 = 0,
    #[doc = "1: The data input DXnB is selected."]
    Value2 = 1,
    #[doc = "2: The data input DXnC is selected."]
    Value3 = 2,
    #[doc = "3: The data input DXnD is selected."]
    Value4 = 3,
    #[doc = "4: The data input DXnE is selected."]
    Value5 = 4,
    #[doc = "5: The data input DXnF is selected."]
    Value6 = 5,
    #[doc = "6: The data input DXnG is selected."]
    Value7 = 6,
    #[doc = "7: The data input is always 1."]
    Value8 = 7,
}
impl From<Dsel> for u8 {
    #[inline(always)]
    fn from(variant: Dsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsel {
    type Ux = u8;
}
impl crate::IsEnum for Dsel {}
#[doc = "Field `DSEL` reader - Data Selection for Input Signal"]
pub type DselR = crate::FieldReader<Dsel>;
impl DselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsel {
        match self.bits {
            0 => Dsel::Value1,
            1 => Dsel::Value2,
            2 => Dsel::Value3,
            3 => Dsel::Value4,
            4 => Dsel::Value5,
            5 => Dsel::Value6,
            6 => Dsel::Value7,
            7 => Dsel::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "The data input DXnA is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dsel::Value1
    }
    #[doc = "The data input DXnB is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dsel::Value2
    }
    #[doc = "The data input DXnC is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dsel::Value3
    }
    #[doc = "The data input DXnD is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Dsel::Value4
    }
    #[doc = "The data input DXnE is selected."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Dsel::Value5
    }
    #[doc = "The data input DXnF is selected."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Dsel::Value6
    }
    #[doc = "The data input DXnG is selected."]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Dsel::Value7
    }
    #[doc = "The data input is always 1."]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Dsel::Value8
    }
}
#[doc = "Field `DSEL` writer - Data Selection for Input Signal"]
pub type DselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dsel, crate::Safe>;
impl<'a, REG> DselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data input DXnA is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value1)
    }
    #[doc = "The data input DXnB is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value2)
    }
    #[doc = "The data input DXnC is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value3)
    }
    #[doc = "The data input DXnD is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value4)
    }
    #[doc = "The data input DXnE is selected."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value5)
    }
    #[doc = "The data input DXnF is selected."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value6)
    }
    #[doc = "The data input DXnG is selected."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value7)
    }
    #[doc = "The data input is always 1."]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Dsel::Value8)
    }
}
#[doc = "Input Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insw {
    #[doc = "0: The input of the data shift unit is controlled by the protocol pre-processor."]
    Value1 = 0,
    #[doc = "1: The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    Value2 = 1,
}
impl From<Insw> for bool {
    #[inline(always)]
    fn from(variant: Insw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSW` reader - Input Switch"]
pub type InswR = crate::BitReader<Insw>;
impl InswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insw {
        match self.bits {
            false => Insw::Value1,
            true => Insw::Value2,
        }
    }
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Insw::Value1
    }
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Insw::Value2
    }
}
#[doc = "Field `INSW` writer - Input Switch"]
pub type InswW<'a, REG> = crate::BitWriter<'a, REG, Insw>;
impl<'a, REG> InswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Insw::Value1)
    }
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Insw::Value2)
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfen {
    #[doc = "0: The input signal is not digitally filtered."]
    Value1 = 0,
    #[doc = "1: The input signal is digitally filtered."]
    Value2 = 1,
}
impl From<Dfen> for bool {
    #[inline(always)]
    fn from(variant: Dfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFEN` reader - Digital Filter Enable"]
pub type DfenR = crate::BitReader<Dfen>;
impl DfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfen {
        match self.bits {
            false => Dfen::Value1,
            true => Dfen::Value2,
        }
    }
    #[doc = "The input signal is not digitally filtered."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dfen::Value1
    }
    #[doc = "The input signal is digitally filtered."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dfen::Value2
    }
}
#[doc = "Field `DFEN` writer - Digital Filter Enable"]
pub type DfenW<'a, REG> = crate::BitWriter<'a, REG, Dfen>;
impl<'a, REG> DfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input signal is not digitally filtered."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfen::Value1)
    }
    #[doc = "The input signal is digitally filtered."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dfen::Value2)
    }
}
#[doc = "Data Synchronization Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsen {
    #[doc = "0: The un-synchronized signal can be taken as input for the data shift unit."]
    Value1 = 0,
    #[doc = "1: The synchronized signal can be taken as input for the data shift unit."]
    Value2 = 1,
}
impl From<Dsen> for bool {
    #[inline(always)]
    fn from(variant: Dsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSEN` reader - Data Synchronization Enable"]
pub type DsenR = crate::BitReader<Dsen>;
impl DsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsen {
        match self.bits {
            false => Dsen::Value1,
            true => Dsen::Value2,
        }
    }
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dsen::Value1
    }
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dsen::Value2
    }
}
#[doc = "Field `DSEN` writer - Data Synchronization Enable"]
pub type DsenW<'a, REG> = crate::BitWriter<'a, REG, Dsen>;
impl<'a, REG> DsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsen::Value1)
    }
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dsen::Value2)
    }
}
#[doc = "Data Polarity for DXn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpol {
    #[doc = "0: The input signal is not inverted."]
    Value1 = 0,
    #[doc = "1: The input signal is inverted."]
    Value2 = 1,
}
impl From<Dpol> for bool {
    #[inline(always)]
    fn from(variant: Dpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPOL` reader - Data Polarity for DXn"]
pub type DpolR = crate::BitReader<Dpol>;
impl DpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpol {
        match self.bits {
            false => Dpol::Value1,
            true => Dpol::Value2,
        }
    }
    #[doc = "The input signal is not inverted."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dpol::Value1
    }
    #[doc = "The input signal is inverted."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dpol::Value2
    }
}
#[doc = "Field `DPOL` writer - Data Polarity for DXn"]
pub type DpolW<'a, REG> = crate::BitWriter<'a, REG, Dpol>;
impl<'a, REG> DpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input signal is not inverted."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dpol::Value1)
    }
    #[doc = "The input signal is inverted."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dpol::Value2)
    }
}
#[doc = "Sampling Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sfsel {
    #[doc = "0: The sampling frequency is fPB."]
    Value1 = 0,
    #[doc = "1: The sampling frequency is fFD."]
    Value2 = 1,
}
impl From<Sfsel> for bool {
    #[inline(always)]
    fn from(variant: Sfsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFSEL` reader - Sampling Frequency Selection"]
pub type SfselR = crate::BitReader<Sfsel>;
impl SfselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sfsel {
        match self.bits {
            false => Sfsel::Value1,
            true => Sfsel::Value2,
        }
    }
    #[doc = "The sampling frequency is fPB."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sfsel::Value1
    }
    #[doc = "The sampling frequency is fFD."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sfsel::Value2
    }
}
#[doc = "Field `SFSEL` writer - Sampling Frequency Selection"]
pub type SfselW<'a, REG> = crate::BitWriter<'a, REG, Sfsel>;
impl<'a, REG> SfselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sampling frequency is fPB."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sfsel::Value1)
    }
    #[doc = "The sampling frequency is fFD."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sfsel::Value2)
    }
}
#[doc = "Combination Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm {
    #[doc = "0: The trigger activation is disabled."]
    Value1 = 0,
    #[doc = "1: A rising edge activates DXnT."]
    Value2 = 1,
    #[doc = "2: A falling edge activates DXnT."]
    Value3 = 2,
    #[doc = "3: Both edges activate DXnT."]
    Value4 = 3,
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm {
    type Ux = u8;
}
impl crate::IsEnum for Cm {}
#[doc = "Field `CM` reader - Combination Mode"]
pub type CmR = crate::FieldReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            0 => Cm::Value1,
            1 => Cm::Value2,
            2 => Cm::Value3,
            3 => Cm::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "The trigger activation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cm::Value1
    }
    #[doc = "A rising edge activates DXnT."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cm::Value2
    }
    #[doc = "A falling edge activates DXnT."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cm::Value3
    }
    #[doc = "Both edges activate DXnT."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cm::Value4
    }
}
#[doc = "Field `CM` writer - Combination Mode"]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm, crate::Safe>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The trigger activation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Value1)
    }
    #[doc = "A rising edge activates DXnT."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Value2)
    }
    #[doc = "A falling edge activates DXnT."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Value3)
    }
    #[doc = "Both edges activate DXnT."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Value4)
    }
}
#[doc = "Synchronized Data Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dxs {
    #[doc = "0: The current value of DXnS is 0."]
    Value1 = 0,
    #[doc = "1: The current value of DXnS is 1."]
    Value2 = 1,
}
impl From<Dxs> for bool {
    #[inline(always)]
    fn from(variant: Dxs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DXS` reader - Synchronized Data Value"]
pub type DxsR = crate::BitReader<Dxs>;
impl DxsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dxs {
        match self.bits {
            false => Dxs::Value1,
            true => Dxs::Value2,
        }
    }
    #[doc = "The current value of DXnS is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dxs::Value1
    }
    #[doc = "The current value of DXnS is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dxs::Value2
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline(always)]
    pub fn dsel(&self) -> DselR {
        DselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline(always)]
    pub fn insw(&self) -> InswR {
        InswR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DfenR {
        DfenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline(always)]
    pub fn dsen(&self) -> DsenR {
        DsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline(always)]
    pub fn dpol(&self) -> DpolR {
        DpolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline(always)]
    pub fn sfsel(&self) -> SfselR {
        SfselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 15 - Synchronized Data Value"]
    #[inline(always)]
    pub fn dxs(&self) -> DxsR {
        DxsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dsel(&mut self) -> DselW<Dx0crSpec> {
        DselW::new(self, 0)
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline(always)]
    #[must_use]
    pub fn insw(&mut self) -> InswW<Dx0crSpec> {
        InswW::new(self, 4)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DfenW<Dx0crSpec> {
        DfenW::new(self, 5)
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsen(&mut self) -> DsenW<Dx0crSpec> {
        DsenW::new(self, 6)
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline(always)]
    #[must_use]
    pub fn dpol(&mut self) -> DpolW<Dx0crSpec> {
        DpolW::new(self, 8)
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sfsel(&mut self) -> SfselW<Dx0crSpec> {
        SfselW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CmW<Dx0crSpec> {
        CmW::new(self, 10)
    }
}
#[doc = "Input Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx0cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx0cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dx0crSpec;
impl crate::RegisterSpec for Dx0crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dx0cr::R`](R) reader structure"]
impl crate::Readable for Dx0crSpec {}
#[doc = "`write(|w| ..)` method takes [`dx0cr::W`](W) writer structure"]
impl crate::Writable for Dx0crSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DX0CR to value 0"]
impl crate::Resettable for Dx0crSpec {
    const RESET_VALUE: u32 = 0;
}
