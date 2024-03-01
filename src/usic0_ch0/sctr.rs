#[doc = "Register `SCTR` reader"]
pub type R = crate::R<SctrSpec>;
#[doc = "Register `SCTR` writer"]
pub type W = crate::W<SctrSpec>;
#[doc = "Shift Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdir {
    #[doc = "0: Shift LSB first. The first data bit of a data word is located at bit position 0."]
    Value1 = 0,
    #[doc = "1: Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    Value2 = 1,
}
impl From<Sdir> for bool {
    #[inline(always)]
    fn from(variant: Sdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIR` reader - Shift Direction"]
pub type SdirR = crate::BitReader<Sdir>;
impl SdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdir {
        match self.bits {
            false => Sdir::Value1,
            true => Sdir::Value2,
        }
    }
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sdir::Value1
    }
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sdir::Value2
    }
}
#[doc = "Field `SDIR` writer - Shift Direction"]
pub type SdirW<'a, REG> = crate::BitWriter<'a, REG, Sdir>;
impl<'a, REG> SdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdir::Value1)
    }
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sdir::Value2)
    }
}
#[doc = "Passive Data Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdl {
    #[doc = "0: The passive data level is 0."]
    Value1 = 0,
    #[doc = "1: The passive data level is 1."]
    Value2 = 1,
}
impl From<Pdl> for bool {
    #[inline(always)]
    fn from(variant: Pdl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDL` reader - Passive Data Level"]
pub type PdlR = crate::BitReader<Pdl>;
impl PdlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdl {
        match self.bits {
            false => Pdl::Value1,
            true => Pdl::Value2,
        }
    }
    #[doc = "The passive data level is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdl::Value1
    }
    #[doc = "The passive data level is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdl::Value2
    }
}
#[doc = "Field `PDL` writer - Passive Data Level"]
pub type PdlW<'a, REG> = crate::BitWriter<'a, REG, Pdl>;
impl<'a, REG> PdlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The passive data level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdl::Value1)
    }
    #[doc = "The passive data level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdl::Value2)
    }
}
#[doc = "Data Shift Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsm {
    #[doc = "0: Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    Value1 = 0,
    #[doc = "2: Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    Value3 = 2,
    #[doc = "3: Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    Value4 = 3,
}
impl From<Dsm> for u8 {
    #[inline(always)]
    fn from(variant: Dsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsm {
    type Ux = u8;
}
#[doc = "Field `DSM` reader - Data Shift Mode"]
pub type DsmR = crate::FieldReader<Dsm>;
impl DsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dsm> {
        match self.bits {
            0 => Some(Dsm::Value1),
            2 => Some(Dsm::Value3),
            3 => Some(Dsm::Value4),
            _ => None,
        }
    }
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dsm::Value1
    }
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Dsm::Value3
    }
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Dsm::Value4
    }
}
#[doc = "Field `DSM` writer - Data Shift Mode"]
pub type DsmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsm>;
impl<'a, REG> DsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsm::Value1)
    }
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Dsm::Value3)
    }
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Dsm::Value4)
    }
}
#[doc = "Port Control Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hpcdir {
    #[doc = "0: The pin(s) with hardware pin control enabled are selected to be in input mode."]
    Value1 = 0,
    #[doc = "1: The pin(s) with hardware pin control enabled are selected to be in output mode."]
    Value2 = 1,
}
impl From<Hpcdir> for bool {
    #[inline(always)]
    fn from(variant: Hpcdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPCDIR` reader - Port Control Direction"]
pub type HpcdirR = crate::BitReader<Hpcdir>;
impl HpcdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hpcdir {
        match self.bits {
            false => Hpcdir::Value1,
            true => Hpcdir::Value2,
        }
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hpcdir::Value1
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hpcdir::Value2
    }
}
#[doc = "Field `HPCDIR` writer - Port Control Direction"]
pub type HpcdirW<'a, REG> = crate::BitWriter<'a, REG, Hpcdir>;
impl<'a, REG> HpcdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcdir::Value1)
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcdir::Value2)
    }
}
#[doc = "Data Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Docfg {
    #[doc = "0: DOUTx = shift data value"]
    Value1 = 0,
    #[doc = "1: DOUTx = inverted shift data value"]
    Value2 = 1,
}
impl From<Docfg> for u8 {
    #[inline(always)]
    fn from(variant: Docfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Docfg {
    type Ux = u8;
}
#[doc = "Field `DOCFG` reader - Data Output Configuration"]
pub type DocfgR = crate::FieldReader<Docfg>;
impl DocfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Docfg> {
        match self.bits {
            0 => Some(Docfg::Value1),
            1 => Some(Docfg::Value2),
            _ => None,
        }
    }
    #[doc = "DOUTx = shift data value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Docfg::Value1
    }
    #[doc = "DOUTx = inverted shift data value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Docfg::Value2
    }
}
#[doc = "Field `DOCFG` writer - Data Output Configuration"]
pub type DocfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Docfg>;
impl<'a, REG> DocfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DOUTx = shift data value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Docfg::Value1)
    }
    #[doc = "DOUTx = inverted shift data value"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Docfg::Value2)
    }
}
#[doc = "Transmission Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trm {
    #[doc = "0: The shift control signal is considered as inactive and data frame transfers are not possible."]
    Value1 = 0,
    #[doc = "1: The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    Value2 = 1,
    #[doc = "2: The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    Value3 = 2,
    #[doc = "3: The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    Value4 = 3,
}
impl From<Trm> for u8 {
    #[inline(always)]
    fn from(variant: Trm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trm {
    type Ux = u8;
}
#[doc = "Field `TRM` reader - Transmission Mode"]
pub type TrmR = crate::FieldReader<Trm>;
impl TrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trm {
        match self.bits {
            0 => Trm::Value1,
            1 => Trm::Value2,
            2 => Trm::Value3,
            3 => Trm::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trm::Value1
    }
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trm::Value2
    }
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Trm::Value3
    }
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Trm::Value4
    }
}
#[doc = "Field `TRM` writer - Transmission Mode"]
pub type TrmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Trm>;
impl<'a, REG> TrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trm::Value1)
    }
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trm::Value2)
    }
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Trm::Value3)
    }
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Trm::Value4)
    }
}
#[doc = "Field `FLE` reader - Frame Length"]
pub type FleR = crate::FieldReader;
#[doc = "Field `FLE` writer - Frame Length"]
pub type FleW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wle {
    #[doc = "0: The data word contains 1 data bit located at bit position 0."]
    Value1 = 0,
    #[doc = "1: The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    Value2 = 1,
    #[doc = "14: The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    Value3 = 14,
    #[doc = "15: The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    Value4 = 15,
}
impl From<Wle> for u8 {
    #[inline(always)]
    fn from(variant: Wle) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wle {
    type Ux = u8;
}
#[doc = "Field `WLE` reader - Word Length"]
pub type WleR = crate::FieldReader<Wle>;
impl WleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wle> {
        match self.bits {
            0 => Some(Wle::Value1),
            1 => Some(Wle::Value2),
            14 => Some(Wle::Value3),
            15 => Some(Wle::Value4),
            _ => None,
        }
    }
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wle::Value1
    }
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wle::Value2
    }
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Wle::Value3
    }
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Wle::Value4
    }
}
#[doc = "Field `WLE` writer - Word Length"]
pub type WleW<'a, REG> = crate::FieldWriter<'a, REG, 4, Wle>;
impl<'a, REG> WleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wle::Value1)
    }
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wle::Value2)
    }
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Wle::Value3)
    }
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Wle::Value4)
    }
}
impl R {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    pub fn sdir(&self) -> SdirR {
        SdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    pub fn pdl(&self) -> PdlR {
        PdlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    pub fn dsm(&self) -> DsmR {
        DsmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    pub fn hpcdir(&self) -> HpcdirR {
        HpcdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    pub fn docfg(&self) -> DocfgR {
        DocfgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    pub fn trm(&self) -> TrmR {
        TrmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    pub fn fle(&self) -> FleR {
        FleR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    pub fn wle(&self) -> WleR {
        WleR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    #[must_use]
    pub fn sdir(&mut self) -> SdirW<SctrSpec> {
        SdirW::new(self, 0)
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    #[must_use]
    pub fn pdl(&mut self) -> PdlW<SctrSpec> {
        PdlW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dsm(&mut self) -> DsmW<SctrSpec> {
        DsmW::new(self, 2)
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    #[must_use]
    pub fn hpcdir(&mut self) -> HpcdirW<SctrSpec> {
        HpcdirW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn docfg(&mut self) -> DocfgW<SctrSpec> {
        DocfgW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trm(&mut self) -> TrmW<SctrSpec> {
        TrmW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn fle(&mut self) -> FleW<SctrSpec> {
        FleW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn wle(&mut self) -> WleW<SctrSpec> {
        WleW::new(self, 24)
    }
}
#[doc = "Shift Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SctrSpec;
impl crate::RegisterSpec for SctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctr::R`](R) reader structure"]
impl crate::Readable for SctrSpec {}
#[doc = "`write(|w| ..)` method takes [`sctr::W`](W) writer structure"]
impl crate::Writable for SctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCTR to value 0"]
impl crate::Resettable for SctrSpec {
    const RESET_VALUE: u32 = 0;
}
