#[doc = "Register `TCSR` reader"]
pub type R = crate::R<TcsrSpec>;
#[doc = "Register `TCSR` writer"]
pub type W = crate::W<TcsrSpec>;
#[doc = "WLE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wlemd {
    #[doc = "0: The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    Value1 = 0,
    #[doc = "1: The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    Value2 = 1,
}
impl From<Wlemd> for bool {
    #[inline(always)]
    fn from(variant: Wlemd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WLEMD` reader - WLE Mode"]
pub type WlemdR = crate::BitReader<Wlemd>;
impl WlemdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wlemd {
        match self.bits {
            false => Wlemd::Value1,
            true => Wlemd::Value2,
        }
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wlemd::Value1
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wlemd::Value2
    }
}
#[doc = "Field `WLEMD` writer - WLE Mode"]
pub type WlemdW<'a, REG> = crate::BitWriter<'a, REG, Wlemd>;
impl<'a, REG> WlemdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wlemd::Value1)
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wlemd::Value2)
    }
}
#[doc = "Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selmd {
    #[doc = "0: The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    Value1 = 0,
    #[doc = "1: The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    Value2 = 1,
}
impl From<Selmd> for bool {
    #[inline(always)]
    fn from(variant: Selmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELMD` reader - Select Mode"]
pub type SelmdR = crate::BitReader<Selmd>;
impl SelmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selmd {
        match self.bits {
            false => Selmd::Value1,
            true => Selmd::Value2,
        }
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selmd::Value1
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Selmd::Value2
    }
}
#[doc = "Field `SELMD` writer - Select Mode"]
pub type SelmdW<'a, REG> = crate::BitWriter<'a, REG, Selmd>;
impl<'a, REG> SelmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selmd::Value1)
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selmd::Value2)
    }
}
#[doc = "FLE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flemd {
    #[doc = "0: The automatic update of FLE is disabled."]
    Value1 = 0,
    #[doc = "1: The automatic update of FLE is enabled."]
    Value2 = 1,
}
impl From<Flemd> for bool {
    #[inline(always)]
    fn from(variant: Flemd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEMD` reader - FLE Mode"]
pub type FlemdR = crate::BitReader<Flemd>;
impl FlemdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flemd {
        match self.bits {
            false => Flemd::Value1,
            true => Flemd::Value2,
        }
    }
    #[doc = "The automatic update of FLE is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Flemd::Value1
    }
    #[doc = "The automatic update of FLE is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Flemd::Value2
    }
}
#[doc = "Field `FLEMD` writer - FLE Mode"]
pub type FlemdW<'a, REG> = crate::BitWriter<'a, REG, Flemd>;
impl<'a, REG> FlemdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of FLE is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Flemd::Value1)
    }
    #[doc = "The automatic update of FLE is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flemd::Value2)
    }
}
#[doc = "WA Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wamd {
    #[doc = "0: The automatic update of bit WA is disabled."]
    Value1 = 0,
    #[doc = "1: The automatic update of bit WA is enabled."]
    Value2 = 1,
}
impl From<Wamd> for bool {
    #[inline(always)]
    fn from(variant: Wamd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAMD` reader - WA Mode"]
pub type WamdR = crate::BitReader<Wamd>;
impl WamdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wamd {
        match self.bits {
            false => Wamd::Value1,
            true => Wamd::Value2,
        }
    }
    #[doc = "The automatic update of bit WA is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wamd::Value1
    }
    #[doc = "The automatic update of bit WA is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wamd::Value2
    }
}
#[doc = "Field `WAMD` writer - WA Mode"]
pub type WamdW<'a, REG> = crate::BitWriter<'a, REG, Wamd>;
impl<'a, REG> WamdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of bit WA is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wamd::Value1)
    }
    #[doc = "The automatic update of bit WA is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wamd::Value2)
    }
}
#[doc = "Hardware Port Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hpcmd {
    #[doc = "0: The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    Value1 = 0,
    #[doc = "1: The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    Value2 = 1,
}
impl From<Hpcmd> for bool {
    #[inline(always)]
    fn from(variant: Hpcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPCMD` reader - Hardware Port Control Mode"]
pub type HpcmdR = crate::BitReader<Hpcmd>;
impl HpcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hpcmd {
        match self.bits {
            false => Hpcmd::Value1,
            true => Hpcmd::Value2,
        }
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hpcmd::Value1
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hpcmd::Value2
    }
}
#[doc = "Field `HPCMD` writer - Hardware Port Control Mode"]
pub type HpcmdW<'a, REG> = crate::BitWriter<'a, REG, Hpcmd>;
impl<'a, REG> HpcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcmd::Value1)
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hpcmd::Value2)
    }
}
#[doc = "Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sof {
    #[doc = "0: The data word in TBUF is not considered as first word of a frame."]
    Value1 = 0,
    #[doc = "1: The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    Value2 = 1,
}
impl From<Sof> for bool {
    #[inline(always)]
    fn from(variant: Sof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Start Of Frame"]
pub type SofR = crate::BitReader<Sof>;
impl SofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sof {
        match self.bits {
            false => Sof::Value1,
            true => Sof::Value2,
        }
    }
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sof::Value1
    }
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sof::Value2
    }
}
#[doc = "Field `SOF` writer - Start Of Frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG, Sof>;
impl<'a, REG> SofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::Value1)
    }
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sof::Value2)
    }
}
#[doc = "End Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eof {
    #[doc = "0: The data word in TBUF is not considered as last word of an SSC frame."]
    Value1 = 0,
    #[doc = "1: The data word in TBUF is considered as last word of an SSC frame."]
    Value2 = 1,
}
impl From<Eof> for bool {
    #[inline(always)]
    fn from(variant: Eof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOF` reader - End Of Frame"]
pub type EofR = crate::BitReader<Eof>;
impl EofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eof {
        match self.bits {
            false => Eof::Value1,
            true => Eof::Value2,
        }
    }
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eof::Value1
    }
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eof::Value2
    }
}
#[doc = "Field `EOF` writer - End Of Frame"]
pub type EofW<'a, REG> = crate::BitWriter<'a, REG, Eof>;
impl<'a, REG> EofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::Value1)
    }
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eof::Value2)
    }
}
#[doc = "Transmit Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdv {
    #[doc = "0: The data word in TBUF is not valid for transmission."]
    Value1 = 0,
    #[doc = "1: The data word in TBUF is valid for transmission and a transmission start is possible. New data should not be written to a TBUFx input location while TDV = 1."]
    Value2 = 1,
}
impl From<Tdv> for bool {
    #[inline(always)]
    fn from(variant: Tdv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDV` reader - Transmit Data Valid"]
pub type TdvR = crate::BitReader<Tdv>;
impl TdvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdv {
        match self.bits {
            false => Tdv::Value1,
            true => Tdv::Value2,
        }
    }
    #[doc = "The data word in TBUF is not valid for transmission."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tdv::Value1
    }
    #[doc = "The data word in TBUF is valid for transmission and a transmission start is possible. New data should not be written to a TBUFx input location while TDV = 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tdv::Value2
    }
}
#[doc = "TBUF Data Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdssm {
    #[doc = "0: The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    Value1 = 0,
    #[doc = "1: The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    Value2 = 1,
}
impl From<Tdssm> for bool {
    #[inline(always)]
    fn from(variant: Tdssm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDSSM` reader - TBUF Data Single Shot Mode"]
pub type TdssmR = crate::BitReader<Tdssm>;
impl TdssmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdssm {
        match self.bits {
            false => Tdssm::Value1,
            true => Tdssm::Value2,
        }
    }
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tdssm::Value1
    }
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tdssm::Value2
    }
}
#[doc = "Field `TDSSM` writer - TBUF Data Single Shot Mode"]
pub type TdssmW<'a, REG> = crate::BitWriter<'a, REG, Tdssm>;
impl<'a, REG> TdssmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdssm::Value1)
    }
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tdssm::Value2)
    }
}
#[doc = "TBUF Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tden {
    #[doc = "0: A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    Value1 = 0,
    #[doc = "1: A transmission of the data word in TBUF can be started if TDV = 1."]
    Value2 = 1,
    #[doc = "2: A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    Value3 = 2,
    #[doc = "3: A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    Value4 = 3,
}
impl From<Tden> for u8 {
    #[inline(always)]
    fn from(variant: Tden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tden {
    type Ux = u8;
}
#[doc = "Field `TDEN` reader - TBUF Data Enable"]
pub type TdenR = crate::FieldReader<Tden>;
impl TdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tden {
        match self.bits {
            0 => Tden::Value1,
            1 => Tden::Value2,
            2 => Tden::Value3,
            3 => Tden::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tden::Value1
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tden::Value2
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Tden::Value3
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Tden::Value4
    }
}
#[doc = "Field `TDEN` writer - TBUF Data Enable"]
pub type TdenW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tden>;
impl<'a, REG> TdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tden::Value1)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tden::Value2)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Tden::Value3)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Tden::Value4)
    }
}
#[doc = "TBUF Data Valid Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdvtr {
    #[doc = "0: Bit TCSR.TE is permanently set."]
    Value1 = 0,
    #[doc = "1: Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    Value2 = 1,
}
impl From<Tdvtr> for bool {
    #[inline(always)]
    fn from(variant: Tdvtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDVTR` reader - TBUF Data Valid Trigger"]
pub type TdvtrR = crate::BitReader<Tdvtr>;
impl TdvtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdvtr {
        match self.bits {
            false => Tdvtr::Value1,
            true => Tdvtr::Value2,
        }
    }
    #[doc = "Bit TCSR.TE is permanently set."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tdvtr::Value1
    }
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tdvtr::Value2
    }
}
#[doc = "Field `TDVTR` writer - TBUF Data Valid Trigger"]
pub type TdvtrW<'a, REG> = crate::BitWriter<'a, REG, Tdvtr>;
impl<'a, REG> TdvtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit TCSR.TE is permanently set."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdvtr::Value1)
    }
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tdvtr::Value2)
    }
}
#[doc = "Word Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wa {
    #[doc = "0: The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    Value1 = 0,
    #[doc = "1: The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    Value2 = 1,
}
impl From<Wa> for bool {
    #[inline(always)]
    fn from(variant: Wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WA` reader - Word Address"]
pub type WaR = crate::BitReader<Wa>;
impl WaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wa {
        match self.bits {
            false => Wa::Value1,
            true => Wa::Value2,
        }
    }
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wa::Value1
    }
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wa::Value2
    }
}
#[doc = "Field `WA` writer - Word Address"]
pub type WaW<'a, REG> = crate::BitWriter<'a, REG, Wa>;
impl<'a, REG> WaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wa::Value1)
    }
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wa::Value2)
    }
}
#[doc = "Transmitted Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsof {
    #[doc = "0: The latest data word transmission has not been started for the first word of a data frame."]
    Value1 = 0,
    #[doc = "1: The latest data word transmission has been started for the first word of a data frame."]
    Value2 = 1,
}
impl From<Tsof> for bool {
    #[inline(always)]
    fn from(variant: Tsof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOF` reader - Transmitted Start Of Frame"]
pub type TsofR = crate::BitReader<Tsof>;
impl TsofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsof {
        match self.bits {
            false => Tsof::Value1,
            true => Tsof::Value2,
        }
    }
    #[doc = "The latest data word transmission has not been started for the first word of a data frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsof::Value1
    }
    #[doc = "The latest data word transmission has been started for the first word of a data frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsof::Value2
    }
}
#[doc = "Transmission Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tv {
    #[doc = "0: The latest start of a data word transmission has taken place while no valid data was available. As a result, the transmission of a data words with passive level (SCTR.PDL) has been started."]
    Value1 = 0,
    #[doc = "1: The latest start of a data word transmission has taken place with valid data from TBUF."]
    Value2 = 1,
}
impl From<Tv> for bool {
    #[inline(always)]
    fn from(variant: Tv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TV` reader - Transmission Valid"]
pub type TvR = crate::BitReader<Tv>;
impl TvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tv {
        match self.bits {
            false => Tv::Value1,
            true => Tv::Value2,
        }
    }
    #[doc = "The latest start of a data word transmission has taken place while no valid data was available. As a result, the transmission of a data words with passive level (SCTR.PDL) has been started."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tv::Value1
    }
    #[doc = "The latest start of a data word transmission has taken place with valid data from TBUF."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tv::Value2
    }
}
#[doc = "Transmission Valid Cumulated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tvc {
    #[doc = "0: Since TVC has been set, at least one data buffer underflow condition has occurred."]
    Value1 = 0,
    #[doc = "1: Since TVC has been set, no data buffer underflow condition has occurred."]
    Value2 = 1,
}
impl From<Tvc> for bool {
    #[inline(always)]
    fn from(variant: Tvc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TVC` reader - Transmission Valid Cumulated"]
pub type TvcR = crate::BitReader<Tvc>;
impl TvcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tvc {
        match self.bits {
            false => Tvc::Value1,
            true => Tvc::Value2,
        }
    }
    #[doc = "Since TVC has been set, at least one data buffer underflow condition has occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tvc::Value1
    }
    #[doc = "Since TVC has been set, no data buffer underflow condition has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tvc::Value2
    }
}
#[doc = "Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: The trigger event has not yet been detected. A transmission of the data word in TBUF can not be started."]
    Value1 = 0,
    #[doc = "1: The trigger event has been detected (or the trigger mechanism is switched off) and a transmission of the data word in TBUF can not be started."]
    Value2 = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Trigger Event"]
pub type TeR = crate::BitReader<Te>;
impl TeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::Value1,
            true => Te::Value2,
        }
    }
    #[doc = "The trigger event has not yet been detected. A transmission of the data word in TBUF can not be started."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Te::Value1
    }
    #[doc = "The trigger event has been detected (or the trigger mechanism is switched off) and a transmission of the data word in TBUF can not be started."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Te::Value2
    }
}
impl R {
    #[doc = "Bit 0 - WLE Mode"]
    #[inline(always)]
    pub fn wlemd(&self) -> WlemdR {
        WlemdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline(always)]
    pub fn selmd(&self) -> SelmdR {
        SelmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline(always)]
    pub fn flemd(&self) -> FlemdR {
        FlemdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline(always)]
    pub fn wamd(&self) -> WamdR {
        WamdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline(always)]
    pub fn hpcmd(&self) -> HpcmdR {
        HpcmdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline(always)]
    pub fn eof(&self) -> EofR {
        EofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Valid"]
    #[inline(always)]
    pub fn tdv(&self) -> TdvR {
        TdvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline(always)]
    pub fn tdssm(&self) -> TdssmR {
        TdssmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline(always)]
    pub fn tden(&self) -> TdenR {
        TdenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline(always)]
    pub fn tdvtr(&self) -> TdvtrR {
        TdvtrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline(always)]
    pub fn wa(&self) -> WaR {
        WaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmitted Start Of Frame"]
    #[inline(always)]
    pub fn tsof(&self) -> TsofR {
        TsofR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Valid"]
    #[inline(always)]
    pub fn tv(&self) -> TvR {
        TvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Valid Cumulated"]
    #[inline(always)]
    pub fn tvc(&self) -> TvcR {
        TvcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Trigger Event"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WLE Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wlemd(&mut self) -> WlemdW<TcsrSpec> {
        WlemdW::new(self, 0)
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline(always)]
    #[must_use]
    pub fn selmd(&mut self) -> SelmdW<TcsrSpec> {
        SelmdW::new(self, 1)
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline(always)]
    #[must_use]
    pub fn flemd(&mut self) -> FlemdW<TcsrSpec> {
        FlemdW::new(self, 2)
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wamd(&mut self) -> WamdW<TcsrSpec> {
        WamdW::new(self, 3)
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hpcmd(&mut self) -> HpcmdW<TcsrSpec> {
        HpcmdW::new(self, 4)
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<TcsrSpec> {
        SofW::new(self, 5)
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EofW<TcsrSpec> {
        EofW::new(self, 6)
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tdssm(&mut self) -> TdssmW<TcsrSpec> {
        TdssmW::new(self, 8)
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tden(&mut self) -> TdenW<TcsrSpec> {
        TdenW::new(self, 10)
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn tdvtr(&mut self) -> TdvtrW<TcsrSpec> {
        TdvtrW::new(self, 12)
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WaW<TcsrSpec> {
        WaW::new(self, 13)
    }
}
#[doc = "Transmit Control/Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcsrSpec;
impl crate::RegisterSpec for TcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcsr::R`](R) reader structure"]
impl crate::Readable for TcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcsr::W`](W) writer structure"]
impl crate::Writable for TcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCSR to value 0"]
impl crate::Resettable for TcsrSpec {
    const RESET_VALUE: u32 = 0;
}
