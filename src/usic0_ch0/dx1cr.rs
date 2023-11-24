#[doc = "Register `DX1CR` reader"]
pub type R = crate::R<DX1CR_SPEC>;
#[doc = "Register `DX1CR` writer"]
pub type W = crate::W<DX1CR_SPEC>;
#[doc = "Field `DSEL` reader - Data Selection for Input Signal"]
pub type DSEL_R = crate::FieldReader<DSEL_A>;
#[doc = "Data Selection for Input Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSEL_A {
    #[doc = "0: The data input DX1A is selected."]
    VALUE1 = 0,
    #[doc = "1: The data input DX1B is selected."]
    VALUE2 = 1,
    #[doc = "2: The data input DX1C is selected."]
    VALUE3 = 2,
    #[doc = "3: The data input DX1D is selected."]
    VALUE4 = 3,
    #[doc = "4: The data input DX1E is selected."]
    VALUE5 = 4,
    #[doc = "5: The data input DX1F is selected."]
    VALUE6 = 5,
    #[doc = "6: The data input DX1G is selected."]
    VALUE7 = 6,
    #[doc = "7: The data input is always 1."]
    VALUE8 = 7,
}
impl From<DSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSEL_A {
    type Ux = u8;
}
impl DSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSEL_A {
        match self.bits {
            0 => DSEL_A::VALUE1,
            1 => DSEL_A::VALUE2,
            2 => DSEL_A::VALUE3,
            3 => DSEL_A::VALUE4,
            4 => DSEL_A::VALUE5,
            5 => DSEL_A::VALUE6,
            6 => DSEL_A::VALUE7,
            7 => DSEL_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "The data input DX1A is selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSEL_A::VALUE1
    }
    #[doc = "The data input DX1B is selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSEL_A::VALUE2
    }
    #[doc = "The data input DX1C is selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DSEL_A::VALUE3
    }
    #[doc = "The data input DX1D is selected."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DSEL_A::VALUE4
    }
    #[doc = "The data input DX1E is selected."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == DSEL_A::VALUE5
    }
    #[doc = "The data input DX1F is selected."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == DSEL_A::VALUE6
    }
    #[doc = "The data input DX1G is selected."]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == DSEL_A::VALUE7
    }
    #[doc = "The data input is always 1."]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == DSEL_A::VALUE8
    }
}
#[doc = "Field `DSEL` writer - Data Selection for Input Signal"]
pub type DSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DSEL_A>;
impl<'a, REG> DSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data input DX1A is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE1)
    }
    #[doc = "The data input DX1B is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE2)
    }
    #[doc = "The data input DX1C is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE3)
    }
    #[doc = "The data input DX1D is selected."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE4)
    }
    #[doc = "The data input DX1E is selected."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE5)
    }
    #[doc = "The data input DX1F is selected."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE6)
    }
    #[doc = "The data input DX1G is selected."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE7)
    }
    #[doc = "The data input is always 1."]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(DSEL_A::VALUE8)
    }
}
#[doc = "Field `DCEN` reader - Delay Compensation Enable"]
pub type DCEN_R = crate::BitReader<DCEN_A>;
#[doc = "Delay Compensation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN_A {
    #[doc = "0: The receive shift clock is dependent on INSW selection."]
    VALUE1 = 0,
    #[doc = "1: The receive shift clock is connected to the selected data input line. This setting is used if delay compensation is required in SSC and IIS protocols, else DCEN should always be 0."]
    VALUE2 = 1,
}
impl From<DCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCEN_A {
        match self.bits {
            false => DCEN_A::VALUE1,
            true => DCEN_A::VALUE2,
        }
    }
    #[doc = "The receive shift clock is dependent on INSW selection."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCEN_A::VALUE1
    }
    #[doc = "The receive shift clock is connected to the selected data input line. This setting is used if delay compensation is required in SSC and IIS protocols, else DCEN should always be 0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCEN_A::VALUE2
    }
}
#[doc = "Field `DCEN` writer - Delay Compensation Enable"]
pub type DCEN_W<'a, REG> = crate::BitWriter<'a, REG, DCEN_A>;
impl<'a, REG> DCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receive shift clock is dependent on INSW selection."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN_A::VALUE1)
    }
    #[doc = "The receive shift clock is connected to the selected data input line. This setting is used if delay compensation is required in SSC and IIS protocols, else DCEN should always be 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DCEN_A::VALUE2)
    }
}
#[doc = "Field `INSW` reader - Input Switch"]
pub type INSW_R = crate::BitReader<INSW_A>;
#[doc = "Input Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INSW_A {
    #[doc = "0: The input of the data shift unit is controlled by the protocol pre-processor."]
    VALUE1 = 0,
    #[doc = "1: The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    VALUE2 = 1,
}
impl From<INSW_A> for bool {
    #[inline(always)]
    fn from(variant: INSW_A) -> Self {
        variant as u8 != 0
    }
}
impl INSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INSW_A {
        match self.bits {
            false => INSW_A::VALUE1,
            true => INSW_A::VALUE2,
        }
    }
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == INSW_A::VALUE1
    }
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == INSW_A::VALUE2
    }
}
#[doc = "Field `INSW` writer - Input Switch"]
pub type INSW_W<'a, REG> = crate::BitWriter<'a, REG, INSW_A>;
impl<'a, REG> INSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input of the data shift unit is controlled by the protocol pre-processor."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(INSW_A::VALUE1)
    }
    #[doc = "The input of the data shift unit is connected to the selected data input line. This setting is used if the signals are directly derived from an input pin without treatment by the protocol pre-processor."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(INSW_A::VALUE2)
    }
}
#[doc = "Field `DFEN` reader - Digital Filter Enable"]
pub type DFEN_R = crate::BitReader<DFEN_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFEN_A {
    #[doc = "0: The input signal is not digitally filtered."]
    VALUE1 = 0,
    #[doc = "1: The input signal is digitally filtered."]
    VALUE2 = 1,
}
impl From<DFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DFEN_A {
        match self.bits {
            false => DFEN_A::VALUE1,
            true => DFEN_A::VALUE2,
        }
    }
    #[doc = "The input signal is not digitally filtered."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DFEN_A::VALUE1
    }
    #[doc = "The input signal is digitally filtered."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DFEN_A::VALUE2
    }
}
#[doc = "Field `DFEN` writer - Digital Filter Enable"]
pub type DFEN_W<'a, REG> = crate::BitWriter<'a, REG, DFEN_A>;
impl<'a, REG> DFEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input signal is not digitally filtered."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DFEN_A::VALUE1)
    }
    #[doc = "The input signal is digitally filtered."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DFEN_A::VALUE2)
    }
}
#[doc = "Field `DSEN` reader - Data Synchronization Enable"]
pub type DSEN_R = crate::BitReader<DSEN_A>;
#[doc = "Data Synchronization Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSEN_A {
    #[doc = "0: The un-synchronized signal can be taken as input for the data shift unit."]
    VALUE1 = 0,
    #[doc = "1: The synchronized signal can be taken as input for the data shift unit."]
    VALUE2 = 1,
}
impl From<DSEN_A> for bool {
    #[inline(always)]
    fn from(variant: DSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSEN_A {
        match self.bits {
            false => DSEN_A::VALUE1,
            true => DSEN_A::VALUE2,
        }
    }
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSEN_A::VALUE1
    }
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSEN_A::VALUE2
    }
}
#[doc = "Field `DSEN` writer - Data Synchronization Enable"]
pub type DSEN_W<'a, REG> = crate::BitWriter<'a, REG, DSEN_A>;
impl<'a, REG> DSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The un-synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DSEN_A::VALUE1)
    }
    #[doc = "The synchronized signal can be taken as input for the data shift unit."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DSEN_A::VALUE2)
    }
}
#[doc = "Field `DPOL` reader - Data Polarity for DXn"]
pub type DPOL_R = crate::BitReader<DPOL_A>;
#[doc = "Data Polarity for DXn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPOL_A {
    #[doc = "0: The input signal is not inverted."]
    VALUE1 = 0,
    #[doc = "1: The input signal is inverted."]
    VALUE2 = 1,
}
impl From<DPOL_A> for bool {
    #[inline(always)]
    fn from(variant: DPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl DPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DPOL_A {
        match self.bits {
            false => DPOL_A::VALUE1,
            true => DPOL_A::VALUE2,
        }
    }
    #[doc = "The input signal is not inverted."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPOL_A::VALUE1
    }
    #[doc = "The input signal is inverted."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPOL_A::VALUE2
    }
}
#[doc = "Field `DPOL` writer - Data Polarity for DXn"]
pub type DPOL_W<'a, REG> = crate::BitWriter<'a, REG, DPOL_A>;
impl<'a, REG> DPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input signal is not inverted."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DPOL_A::VALUE1)
    }
    #[doc = "The input signal is inverted."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DPOL_A::VALUE2)
    }
}
#[doc = "Field `SFSEL` reader - Sampling Frequency Selection"]
pub type SFSEL_R = crate::BitReader<SFSEL_A>;
#[doc = "Sampling Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFSEL_A {
    #[doc = "0: The sampling frequency is fPB."]
    VALUE1 = 0,
    #[doc = "1: The sampling frequency is fFD."]
    VALUE2 = 1,
}
impl From<SFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SFSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SFSEL_A {
        match self.bits {
            false => SFSEL_A::VALUE1,
            true => SFSEL_A::VALUE2,
        }
    }
    #[doc = "The sampling frequency is fPB."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SFSEL_A::VALUE1
    }
    #[doc = "The sampling frequency is fFD."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SFSEL_A::VALUE2
    }
}
#[doc = "Field `SFSEL` writer - Sampling Frequency Selection"]
pub type SFSEL_W<'a, REG> = crate::BitWriter<'a, REG, SFSEL_A>;
impl<'a, REG> SFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sampling frequency is fPB."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SFSEL_A::VALUE1)
    }
    #[doc = "The sampling frequency is fFD."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SFSEL_A::VALUE2)
    }
}
#[doc = "Field `CM` reader - Combination Mode"]
pub type CM_R = crate::FieldReader<CM_A>;
#[doc = "Combination Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: The trigger activation is disabled."]
    VALUE1 = 0,
    #[doc = "1: A rising edge activates DX1T."]
    VALUE2 = 1,
    #[doc = "2: A falling edge activates DX1T."]
    VALUE3 = 2,
    #[doc = "3: Both edges activate DX1T."]
    VALUE4 = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CM_A {
    type Ux = u8;
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::VALUE1,
            1 => CM_A::VALUE2,
            2 => CM_A::VALUE3,
            3 => CM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "The trigger activation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CM_A::VALUE1
    }
    #[doc = "A rising edge activates DX1T."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CM_A::VALUE2
    }
    #[doc = "A falling edge activates DX1T."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CM_A::VALUE3
    }
    #[doc = "Both edges activate DX1T."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CM_A::VALUE4
    }
}
#[doc = "Field `CM` writer - Combination Mode"]
pub type CM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CM_A>;
impl<'a, REG> CM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The trigger activation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::VALUE1)
    }
    #[doc = "A rising edge activates DX1T."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::VALUE2)
    }
    #[doc = "A falling edge activates DX1T."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::VALUE3)
    }
    #[doc = "Both edges activate DX1T."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CM_A::VALUE4)
    }
}
#[doc = "Field `DXS` reader - Synchronized Data Value"]
pub type DXS_R = crate::BitReader<DXS_A>;
#[doc = "Synchronized Data Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DXS_A {
    #[doc = "0: The current value of DX1S is 0."]
    VALUE1 = 0,
    #[doc = "1: The current value of DX1S is 1."]
    VALUE2 = 1,
}
impl From<DXS_A> for bool {
    #[inline(always)]
    fn from(variant: DXS_A) -> Self {
        variant as u8 != 0
    }
}
impl DXS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DXS_A {
        match self.bits {
            false => DXS_A::VALUE1,
            true => DXS_A::VALUE2,
        }
    }
    #[doc = "The current value of DX1S is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DXS_A::VALUE1
    }
    #[doc = "The current value of DX1S is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DXS_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline(always)]
    pub fn dsel(&self) -> DSEL_R {
        DSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Delay Compensation Enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline(always)]
    pub fn insw(&self) -> INSW_R {
        INSW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline(always)]
    pub fn dpol(&self) -> DPOL_R {
        DPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline(always)]
    pub fn sfsel(&self) -> SFSEL_R {
        SFSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 15 - Synchronized Data Value"]
    #[inline(always)]
    pub fn dxs(&self) -> DXS_R {
        DXS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Selection for Input Signal"]
    #[inline(always)]
    #[must_use]
    pub fn dsel(&mut self) -> DSEL_W<DX1CR_SPEC> {
        DSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Delay Compensation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DCEN_W<DX1CR_SPEC> {
        DCEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Input Switch"]
    #[inline(always)]
    #[must_use]
    pub fn insw(&mut self) -> INSW_W<DX1CR_SPEC> {
        INSW_W::new(self, 4)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfen(&mut self) -> DFEN_W<DX1CR_SPEC> {
        DFEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Data Synchronization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsen(&mut self) -> DSEN_W<DX1CR_SPEC> {
        DSEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - Data Polarity for DXn"]
    #[inline(always)]
    #[must_use]
    pub fn dpol(&mut self) -> DPOL_W<DX1CR_SPEC> {
        DPOL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Sampling Frequency Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sfsel(&mut self) -> SFSEL_W<DX1CR_SPEC> {
        SFSEL_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Combination Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<DX1CR_SPEC> {
        CM_W::new(self, 10)
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
#[doc = "Input Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dx1cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dx1cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DX1CR_SPEC;
impl crate::RegisterSpec for DX1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dx1cr::R`](R) reader structure"]
impl crate::Readable for DX1CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dx1cr::W`](W) writer structure"]
impl crate::Writable for DX1CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DX1CR to value 0"]
impl crate::Resettable for DX1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
