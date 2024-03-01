#[doc = "Register `PCR_SSCMode` reader"]
pub type R = crate::R<PcrSscmodeSpec>;
#[doc = "Register `PCR_SSCMode` writer"]
pub type W = crate::W<PcrSscmodeSpec>;
#[doc = "MSLS Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mslsen {
    #[doc = "0: The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    Value1 = 0,
    #[doc = "1: The MSLS generation is enabled. This is the setting for SSC master mode."]
    Value2 = 1,
}
impl From<Mslsen> for bool {
    #[inline(always)]
    fn from(variant: Mslsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSLSEN` reader - MSLS Enable"]
pub type MslsenR = crate::BitReader<Mslsen>;
impl MslsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mslsen {
        match self.bits {
            false => Mslsen::Value1,
            true => Mslsen::Value2,
        }
    }
    #[doc = "The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mslsen::Value1
    }
    #[doc = "The MSLS generation is enabled. This is the setting for SSC master mode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mslsen::Value2
    }
}
#[doc = "Field `MSLSEN` writer - MSLS Enable"]
pub type MslsenW<'a, REG> = crate::BitWriter<'a, REG, Mslsen>;
impl<'a, REG> MslsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MSLS generation is disabled (MSLS = 0). This is the setting for SSC slave mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mslsen::Value1)
    }
    #[doc = "The MSLS generation is enabled. This is the setting for SSC master mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mslsen::Value2)
    }
}
#[doc = "Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selctr {
    #[doc = "0: The coded select mode is enabled."]
    Value1 = 0,
    #[doc = "1: The direct select mode is enabled."]
    Value2 = 1,
}
impl From<Selctr> for bool {
    #[inline(always)]
    fn from(variant: Selctr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELCTR` reader - Select Control"]
pub type SelctrR = crate::BitReader<Selctr>;
impl SelctrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selctr {
        match self.bits {
            false => Selctr::Value1,
            true => Selctr::Value2,
        }
    }
    #[doc = "The coded select mode is enabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selctr::Value1
    }
    #[doc = "The direct select mode is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Selctr::Value2
    }
}
#[doc = "Field `SELCTR` writer - Select Control"]
pub type SelctrW<'a, REG> = crate::BitWriter<'a, REG, Selctr>;
impl<'a, REG> SelctrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The coded select mode is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selctr::Value1)
    }
    #[doc = "The direct select mode is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selctr::Value2)
    }
}
#[doc = "Select Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selinv {
    #[doc = "0: The SELO outputs have the same polarity as the MSLS signal (active high)."]
    Value1 = 0,
    #[doc = "1: The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
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
    #[doc = "The SELO outputs have the same polarity as the MSLS signal (active high)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selinv::Value1
    }
    #[doc = "The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
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
    #[doc = "The SELO outputs have the same polarity as the MSLS signal (active high)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selinv::Value1)
    }
    #[doc = "The SELO outputs have the inverted polarity to the MSLS signal (active low)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selinv::Value2)
    }
}
#[doc = "Frame End Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fem {
    #[doc = "0: The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    Value1 = 0,
    #[doc = "1: The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    Value2 = 1,
}
impl From<Fem> for bool {
    #[inline(always)]
    fn from(variant: Fem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEM` reader - Frame End Mode"]
pub type FemR = crate::BitReader<Fem>;
impl FemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fem {
        match self.bits {
            false => Fem::Value1,
            true => Fem::Value2,
        }
    }
    #[doc = "The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fem::Value1
    }
    #[doc = "The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fem::Value2
    }
}
#[doc = "Field `FEM` writer - Frame End Mode"]
pub type FemW<'a, REG> = crate::BitWriter<'a, REG, Fem>;
impl<'a, REG> FemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The current data frame is considered as finished when the last bit of a data word has been sent out and the transmit buffer TBUF does not contain new data (TDV = 0)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fem::Value1)
    }
    #[doc = "The MSLS signal is kept active also while no new data is available and no other end of frame condition is reached. In this case, the software can accept delays in delivering the data without automatic deactivation of MSLS in multi-word data frames."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fem::Value2)
    }
}
#[doc = "Input Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctqsel1 {
    #[doc = "0: fCTQIN = fPDIV"]
    Value1 = 0,
    #[doc = "1: fCTQIN = fPPP"]
    Value2 = 1,
    #[doc = "2: fCTQIN = fSCLK"]
    Value3 = 2,
    #[doc = "3: fCTQIN = fMCLK"]
    Value4 = 3,
}
impl From<Ctqsel1> for u8 {
    #[inline(always)]
    fn from(variant: Ctqsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctqsel1 {
    type Ux = u8;
}
#[doc = "Field `CTQSEL1` reader - Input Frequency Selection"]
pub type Ctqsel1R = crate::FieldReader<Ctqsel1>;
impl Ctqsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctqsel1 {
        match self.bits {
            0 => Ctqsel1::Value1,
            1 => Ctqsel1::Value2,
            2 => Ctqsel1::Value3,
            3 => Ctqsel1::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ctqsel1::Value1
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ctqsel1::Value2
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ctqsel1::Value3
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Ctqsel1::Value4
    }
}
#[doc = "Field `CTQSEL1` writer - Input Frequency Selection"]
pub type Ctqsel1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ctqsel1>;
impl<'a, REG> Ctqsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fCTQIN = fPDIV"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel1::Value1)
    }
    #[doc = "fCTQIN = fPPP"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel1::Value2)
    }
    #[doc = "fCTQIN = fSCLK"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel1::Value3)
    }
    #[doc = "fCTQIN = fMCLK"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctqsel1::Value4)
    }
}
#[doc = "Field `PCTQ1` reader - Divider Factor PCTQ1 for Tiw and Tnf"]
pub type Pctq1R = crate::FieldReader;
#[doc = "Field `PCTQ1` writer - Divider Factor PCTQ1 for Tiw and Tnf"]
pub type Pctq1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCTQ1` reader - Divider Factor DCTQ1 for Tiw and Tnf"]
pub type Dctq1R = crate::FieldReader;
#[doc = "Field `DCTQ1` writer - Divider Factor DCTQ1 for Tiw and Tnf"]
pub type Dctq1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parien {
    #[doc = "0: A protocol interrupt is not generated with the detection of a parity error."]
    Value1 = 0,
    #[doc = "1: A protocol interrupt is generated with the detection of a parity error."]
    Value2 = 1,
}
impl From<Parien> for bool {
    #[inline(always)]
    fn from(variant: Parien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARIEN` reader - Parity Error Interrupt Enable"]
pub type ParienR = crate::BitReader<Parien>;
impl ParienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Parien {
        match self.bits {
            false => Parien::Value1,
            true => Parien::Value2,
        }
    }
    #[doc = "A protocol interrupt is not generated with the detection of a parity error."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Parien::Value1
    }
    #[doc = "A protocol interrupt is generated with the detection of a parity error."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Parien::Value2
    }
}
#[doc = "Field `PARIEN` writer - Parity Error Interrupt Enable"]
pub type ParienW<'a, REG> = crate::BitWriter<'a, REG, Parien>;
impl<'a, REG> ParienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not generated with the detection of a parity error."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Parien::Value1)
    }
    #[doc = "A protocol interrupt is generated with the detection of a parity error."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Parien::Value2)
    }
}
#[doc = "MSLS Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mslsien {
    #[doc = "0: A protocol interrupt is not generated if a change of signal MSLS is detected."]
    Value1 = 0,
    #[doc = "1: A protocol interrupt is generated if a change of signal MSLS is detected."]
    Value2 = 1,
}
impl From<Mslsien> for bool {
    #[inline(always)]
    fn from(variant: Mslsien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSLSIEN` reader - MSLS Interrupt Enable"]
pub type MslsienR = crate::BitReader<Mslsien>;
impl MslsienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mslsien {
        match self.bits {
            false => Mslsien::Value1,
            true => Mslsien::Value2,
        }
    }
    #[doc = "A protocol interrupt is not generated if a change of signal MSLS is detected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mslsien::Value1
    }
    #[doc = "A protocol interrupt is generated if a change of signal MSLS is detected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mslsien::Value2
    }
}
#[doc = "Field `MSLSIEN` writer - MSLS Interrupt Enable"]
pub type MslsienW<'a, REG> = crate::BitWriter<'a, REG, Mslsien>;
impl<'a, REG> MslsienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not generated if a change of signal MSLS is detected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mslsien::Value1)
    }
    #[doc = "A protocol interrupt is generated if a change of signal MSLS is detected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mslsien::Value2)
    }
}
#[doc = "DX2T Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dx2tien {
    #[doc = "0: A protocol interrupt is not generated if DX2T is activated."]
    Value1 = 0,
    #[doc = "1: A protocol interrupt is generated if DX2T is activated."]
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
    #[doc = "A protocol interrupt is not generated if DX2T is activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dx2tien::Value1
    }
    #[doc = "A protocol interrupt is generated if DX2T is activated."]
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
    #[doc = "A protocol interrupt is not generated if DX2T is activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2tien::Value1)
    }
    #[doc = "A protocol interrupt is generated if DX2T is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dx2tien::Value2)
    }
}
#[doc = "Select Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selo {
    #[doc = "0: The corresponding SELOx line cannot be activated."]
    Value1 = 0,
    #[doc = "1: The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    Value2 = 1,
}
impl From<Selo> for u8 {
    #[inline(always)]
    fn from(variant: Selo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selo {
    type Ux = u8;
}
#[doc = "Field `SELO` reader - Select Output"]
pub type SeloR = crate::FieldReader<Selo>;
impl SeloR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selo> {
        match self.bits {
            0 => Some(Selo::Value1),
            1 => Some(Selo::Value2),
            _ => None,
        }
    }
    #[doc = "The corresponding SELOx line cannot be activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selo::Value1
    }
    #[doc = "The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Selo::Value2
    }
}
#[doc = "Field `SELO` writer - Select Output"]
pub type SeloW<'a, REG> = crate::FieldWriter<'a, REG, 8, Selo>;
impl<'a, REG> SeloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The corresponding SELOx line cannot be activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selo::Value1)
    }
    #[doc = "The corresponding SELOx line can be activated (according to the mode selected by SELCTR)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selo::Value2)
    }
}
#[doc = "Enable Inter-Word Delay Tiw\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tiwen {
    #[doc = "0: No delay between data words of the same frame."]
    Value1 = 0,
    #[doc = "1: The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    Value2 = 1,
}
impl From<Tiwen> for bool {
    #[inline(always)]
    fn from(variant: Tiwen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIWEN` reader - Enable Inter-Word Delay Tiw"]
pub type TiwenR = crate::BitReader<Tiwen>;
impl TiwenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tiwen {
        match self.bits {
            false => Tiwen::Value1,
            true => Tiwen::Value2,
        }
    }
    #[doc = "No delay between data words of the same frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tiwen::Value1
    }
    #[doc = "The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tiwen::Value2
    }
}
#[doc = "Field `TIWEN` writer - Enable Inter-Word Delay Tiw"]
pub type TiwenW<'a, REG> = crate::BitWriter<'a, REG, Tiwen>;
impl<'a, REG> TiwenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No delay between data words of the same frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tiwen::Value1)
    }
    #[doc = "The inter-word delay Tiw is enabled and introduced between data words of the same frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tiwen::Value2)
    }
}
#[doc = "Slave Mode Clock Phase Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slphsel {
    #[doc = "0: Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    Value1 = 0,
    #[doc = "1: The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    Value2 = 1,
}
impl From<Slphsel> for bool {
    #[inline(always)]
    fn from(variant: Slphsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPHSEL` reader - Slave Mode Clock Phase Select"]
pub type SlphselR = crate::BitReader<Slphsel>;
impl SlphselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slphsel {
        match self.bits {
            false => Slphsel::Value1,
            true => Slphsel::Value2,
        }
    }
    #[doc = "Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Slphsel::Value1
    }
    #[doc = "The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Slphsel::Value2
    }
}
#[doc = "Field `SLPHSEL` writer - Slave Mode Clock Phase Select"]
pub type SlphselW<'a, REG> = crate::BitWriter<'a, REG, Slphsel>;
impl<'a, REG> SlphselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data bits are shifted out with the leading edge of the shift clock signal and latched in with the trailing edge."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Slphsel::Value1)
    }
    #[doc = "The first data bit is shifted out when the data shift unit receives a low to high transition from the DX2 stage. Subsequent bits are shifted out with the trailing edge of the shift clock signal. Data bits are always latched in with the leading edge."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Slphsel::Value2)
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclk {
    #[doc = "0: The MCLK generation is disabled and output MCLK = 0."]
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
    #[doc = "The MCLK generation is disabled and output MCLK = 0."]
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
    #[doc = "The MCLK generation is disabled and output MCLK = 0."]
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
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline(always)]
    pub fn mslsen(&self) -> MslsenR {
        MslsenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline(always)]
    pub fn selctr(&self) -> SelctrR {
        SelctrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&self) -> SelinvR {
        SelinvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline(always)]
    pub fn fem(&self) -> FemR {
        FemR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline(always)]
    pub fn ctqsel1(&self) -> Ctqsel1R {
        Ctqsel1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn pctq1(&self) -> Pctq1R {
        Pctq1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    pub fn dctq1(&self) -> Dctq1R {
        Dctq1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn parien(&self) -> ParienR {
        ParienR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline(always)]
    pub fn mslsien(&self) -> MslsienR {
        MslsienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&self) -> Dx2tienR {
        Dx2tienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline(always)]
    pub fn selo(&self) -> SeloR {
        SeloR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline(always)]
    pub fn tiwen(&self) -> TiwenR {
        TiwenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline(always)]
    pub fn slphsel(&self) -> SlphselR {
        SlphselR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MclkR {
        MclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSLS Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mslsen(&mut self) -> MslsenW<PcrSscmodeSpec> {
        MslsenW::new(self, 0)
    }
    #[doc = "Bit 1 - Select Control"]
    #[inline(always)]
    #[must_use]
    pub fn selctr(&mut self) -> SelctrW<PcrSscmodeSpec> {
        SelctrW::new(self, 1)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn selinv(&mut self) -> SelinvW<PcrSscmodeSpec> {
        SelinvW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame End Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fem(&mut self) -> FemW<PcrSscmodeSpec> {
        FemW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Input Frequency Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ctqsel1(&mut self) -> Ctqsel1W<PcrSscmodeSpec> {
        Ctqsel1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Divider Factor PCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    #[must_use]
    pub fn pctq1(&mut self) -> Pctq1W<PcrSscmodeSpec> {
        Pctq1W::new(self, 6)
    }
    #[doc = "Bits 8:12 - Divider Factor DCTQ1 for Tiw and Tnf"]
    #[inline(always)]
    #[must_use]
    pub fn dctq1(&mut self) -> Dctq1W<PcrSscmodeSpec> {
        Dctq1W::new(self, 8)
    }
    #[doc = "Bit 13 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn parien(&mut self) -> ParienW<PcrSscmodeSpec> {
        ParienW::new(self, 13)
    }
    #[doc = "Bit 14 - MSLS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mslsien(&mut self) -> MslsienW<PcrSscmodeSpec> {
        MslsienW::new(self, 14)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dx2tien(&mut self) -> Dx2tienW<PcrSscmodeSpec> {
        Dx2tienW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Select Output"]
    #[inline(always)]
    #[must_use]
    pub fn selo(&mut self) -> SeloW<PcrSscmodeSpec> {
        SeloW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable Inter-Word Delay Tiw"]
    #[inline(always)]
    #[must_use]
    pub fn tiwen(&mut self) -> TiwenW<PcrSscmodeSpec> {
        TiwenW::new(self, 24)
    }
    #[doc = "Bit 25 - Slave Mode Clock Phase Select"]
    #[inline(always)]
    #[must_use]
    pub fn slphsel(&mut self) -> SlphselW<PcrSscmodeSpec> {
        SlphselW::new(self, 25)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MclkW<PcrSscmodeSpec> {
        MclkW::new(self, 31)
    }
}
#[doc = "Protocol Control Register \\[SSC Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_sscmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_sscmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSscmodeSpec;
impl crate::RegisterSpec for PcrSscmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr_sscmode::R`](R) reader structure"]
impl crate::Readable for PcrSscmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr_sscmode::W`](W) writer structure"]
impl crate::Writable for PcrSscmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR_SSCMode to value 0"]
impl crate::Resettable for PcrSscmodeSpec {
    const RESET_VALUE: u32 = 0;
}
