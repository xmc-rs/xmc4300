#[doc = "Register `TCSR` reader"]
pub type R = crate::R<TCSR_SPEC>;
#[doc = "Register `TCSR` writer"]
pub type W = crate::W<TCSR_SPEC>;
#[doc = "WLE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WLEMD_A {
    #[doc = "0: The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    VALUE1 = 0,
    #[doc = "1: The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    VALUE2 = 1,
}
impl From<WLEMD_A> for bool {
    #[inline(always)]
    fn from(variant: WLEMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WLEMD` reader - WLE Mode"]
pub type WLEMD_R = crate::BitReader<WLEMD_A>;
impl WLEMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WLEMD_A {
        match self.bits {
            false => WLEMD_A::VALUE1,
            true => WLEMD_A::VALUE2,
        }
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WLEMD_A::VALUE1
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WLEMD_A::VALUE2
    }
}
#[doc = "Field `WLEMD` writer - WLE Mode"]
pub type WLEMD_W<'a, REG> = crate::BitWriter<'a, REG, WLEMD_A>;
impl<'a, REG> WLEMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WLEMD_A::VALUE1)
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WLEMD_A::VALUE2)
    }
}
#[doc = "Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELMD_A {
    #[doc = "0: The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    VALUE1 = 0,
    #[doc = "1: The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    VALUE2 = 1,
}
impl From<SELMD_A> for bool {
    #[inline(always)]
    fn from(variant: SELMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELMD` reader - Select Mode"]
pub type SELMD_R = crate::BitReader<SELMD_A>;
impl SELMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELMD_A {
        match self.bits {
            false => SELMD_A::VALUE1,
            true => SELMD_A::VALUE2,
        }
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELMD_A::VALUE1
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELMD_A::VALUE2
    }
}
#[doc = "Field `SELMD` writer - Select Mode"]
pub type SELMD_W<'a, REG> = crate::BitWriter<'a, REG, SELMD_A>;
impl<'a, REG> SELMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELMD_A::VALUE1)
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\]
is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELMD_A::VALUE2)
    }
}
#[doc = "FLE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEMD_A {
    #[doc = "0: The automatic update of FLE is disabled."]
    VALUE1 = 0,
    #[doc = "1: The automatic update of FLE is enabled."]
    VALUE2 = 1,
}
impl From<FLEMD_A> for bool {
    #[inline(always)]
    fn from(variant: FLEMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEMD` reader - FLE Mode"]
pub type FLEMD_R = crate::BitReader<FLEMD_A>;
impl FLEMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLEMD_A {
        match self.bits {
            false => FLEMD_A::VALUE1,
            true => FLEMD_A::VALUE2,
        }
    }
    #[doc = "The automatic update of FLE is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FLEMD_A::VALUE1
    }
    #[doc = "The automatic update of FLE is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FLEMD_A::VALUE2
    }
}
#[doc = "Field `FLEMD` writer - FLE Mode"]
pub type FLEMD_W<'a, REG> = crate::BitWriter<'a, REG, FLEMD_A>;
impl<'a, REG> FLEMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of FLE is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLEMD_A::VALUE1)
    }
    #[doc = "The automatic update of FLE is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLEMD_A::VALUE2)
    }
}
#[doc = "WA Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAMD_A {
    #[doc = "0: The automatic update of bit WA is disabled."]
    VALUE1 = 0,
    #[doc = "1: The automatic update of bit WA is enabled."]
    VALUE2 = 1,
}
impl From<WAMD_A> for bool {
    #[inline(always)]
    fn from(variant: WAMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAMD` reader - WA Mode"]
pub type WAMD_R = crate::BitReader<WAMD_A>;
impl WAMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAMD_A {
        match self.bits {
            false => WAMD_A::VALUE1,
            true => WAMD_A::VALUE2,
        }
    }
    #[doc = "The automatic update of bit WA is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAMD_A::VALUE1
    }
    #[doc = "The automatic update of bit WA is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAMD_A::VALUE2
    }
}
#[doc = "Field `WAMD` writer - WA Mode"]
pub type WAMD_W<'a, REG> = crate::BitWriter<'a, REG, WAMD_A>;
impl<'a, REG> WAMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of bit WA is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAMD_A::VALUE1)
    }
    #[doc = "The automatic update of bit WA is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAMD_A::VALUE2)
    }
}
#[doc = "Hardware Port Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPCMD_A {
    #[doc = "0: The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    VALUE1 = 0,
    #[doc = "1: The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    VALUE2 = 1,
}
impl From<HPCMD_A> for bool {
    #[inline(always)]
    fn from(variant: HPCMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HPCMD` reader - Hardware Port Control Mode"]
pub type HPCMD_R = crate::BitReader<HPCMD_A>;
impl HPCMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPCMD_A {
        match self.bits {
            false => HPCMD_A::VALUE1,
            true => HPCMD_A::VALUE2,
        }
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HPCMD_A::VALUE1
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HPCMD_A::VALUE2
    }
}
#[doc = "Field `HPCMD` writer - Hardware Port Control Mode"]
pub type HPCMD_W<'a, REG> = crate::BitWriter<'a, REG, HPCMD_A>;
impl<'a, REG> HPCMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HPCMD_A::VALUE1)
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HPCMD_A::VALUE2)
    }
}
#[doc = "Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF_A {
    #[doc = "0: The data word in TBUF is not considered as first word of a frame."]
    VALUE1 = 0,
    #[doc = "1: The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    VALUE2 = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - Start Of Frame"]
pub type SOF_R = crate::BitReader<SOF_A>;
impl SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::VALUE1,
            true => SOF_A::VALUE2,
        }
    }
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOF_A::VALUE1
    }
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOF_A::VALUE2
    }
}
#[doc = "Field `SOF` writer - Start Of Frame"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG, SOF_A>;
impl<'a, REG> SOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SOF_A::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SOF_A::VALUE2)
    }
}
#[doc = "End Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOF_A {
    #[doc = "0: The data word in TBUF is not considered as last word of an SSC frame."]
    VALUE1 = 0,
    #[doc = "1: The data word in TBUF is considered as last word of an SSC frame."]
    VALUE2 = 1,
}
impl From<EOF_A> for bool {
    #[inline(always)]
    fn from(variant: EOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOF` reader - End Of Frame"]
pub type EOF_R = crate::BitReader<EOF_A>;
impl EOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOF_A {
        match self.bits {
            false => EOF_A::VALUE1,
            true => EOF_A::VALUE2,
        }
    }
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EOF_A::VALUE1
    }
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EOF_A::VALUE2
    }
}
#[doc = "Field `EOF` writer - End Of Frame"]
pub type EOF_W<'a, REG> = crate::BitWriter<'a, REG, EOF_A>;
impl<'a, REG> EOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EOF_A::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EOF_A::VALUE2)
    }
}
#[doc = "Transmit Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDV_A {
    #[doc = "0: The data word in TBUF is not valid for transmission."]
    VALUE1 = 0,
    #[doc = "1: The data word in TBUF is valid for transmission and a transmission start is possible. New data should not be written to a TBUFx input location while TDV = 1."]
    VALUE2 = 1,
}
impl From<TDV_A> for bool {
    #[inline(always)]
    fn from(variant: TDV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDV` reader - Transmit Data Valid"]
pub type TDV_R = crate::BitReader<TDV_A>;
impl TDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDV_A {
        match self.bits {
            false => TDV_A::VALUE1,
            true => TDV_A::VALUE2,
        }
    }
    #[doc = "The data word in TBUF is not valid for transmission."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDV_A::VALUE1
    }
    #[doc = "The data word in TBUF is valid for transmission and a transmission start is possible. New data should not be written to a TBUFx input location while TDV = 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDV_A::VALUE2
    }
}
#[doc = "TBUF Data Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDSSM_A {
    #[doc = "0: The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    VALUE1 = 0,
    #[doc = "1: The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    VALUE2 = 1,
}
impl From<TDSSM_A> for bool {
    #[inline(always)]
    fn from(variant: TDSSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDSSM` reader - TBUF Data Single Shot Mode"]
pub type TDSSM_R = crate::BitReader<TDSSM_A>;
impl TDSSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDSSM_A {
        match self.bits {
            false => TDSSM_A::VALUE1,
            true => TDSSM_A::VALUE2,
        }
    }
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDSSM_A::VALUE1
    }
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDSSM_A::VALUE2
    }
}
#[doc = "Field `TDSSM` writer - TBUF Data Single Shot Mode"]
pub type TDSSM_W<'a, REG> = crate::BitWriter<'a, REG, TDSSM_A>;
impl<'a, REG> TDSSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TDSSM_A::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TDSSM_A::VALUE2)
    }
}
#[doc = "TBUF Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TDEN_A {
    #[doc = "0: A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    VALUE1 = 0,
    #[doc = "1: A transmission of the data word in TBUF can be started if TDV = 1."]
    VALUE2 = 1,
    #[doc = "2: A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    VALUE3 = 2,
    #[doc = "3: A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    VALUE4 = 3,
}
impl From<TDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TDEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TDEN_A {
    type Ux = u8;
}
impl crate::IsEnum for TDEN_A {}
#[doc = "Field `TDEN` reader - TBUF Data Enable"]
pub type TDEN_R = crate::FieldReader<TDEN_A>;
impl TDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDEN_A {
        match self.bits {
            0 => TDEN_A::VALUE1,
            1 => TDEN_A::VALUE2,
            2 => TDEN_A::VALUE3,
            3 => TDEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDEN_A::VALUE1
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDEN_A::VALUE2
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TDEN_A::VALUE3
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TDEN_A::VALUE4
    }
}
#[doc = "Field `TDEN` writer - TBUF Data Enable"]
pub type TDEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TDEN_A, crate::Safe>;
impl<'a, REG> TDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TDEN_A::VALUE1)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TDEN_A::VALUE2)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TDEN_A::VALUE3)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TDEN_A::VALUE4)
    }
}
#[doc = "TBUF Data Valid Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDVTR_A {
    #[doc = "0: Bit TCSR.TE is permanently set."]
    VALUE1 = 0,
    #[doc = "1: Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    VALUE2 = 1,
}
impl From<TDVTR_A> for bool {
    #[inline(always)]
    fn from(variant: TDVTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDVTR` reader - TBUF Data Valid Trigger"]
pub type TDVTR_R = crate::BitReader<TDVTR_A>;
impl TDVTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDVTR_A {
        match self.bits {
            false => TDVTR_A::VALUE1,
            true => TDVTR_A::VALUE2,
        }
    }
    #[doc = "Bit TCSR.TE is permanently set."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDVTR_A::VALUE1
    }
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDVTR_A::VALUE2
    }
}
#[doc = "Field `TDVTR` writer - TBUF Data Valid Trigger"]
pub type TDVTR_W<'a, REG> = crate::BitWriter<'a, REG, TDVTR_A>;
impl<'a, REG> TDVTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit TCSR.TE is permanently set."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TDVTR_A::VALUE1)
    }
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TDVTR_A::VALUE2)
    }
}
#[doc = "Word Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WA_A {
    #[doc = "0: The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    VALUE1 = 0,
    #[doc = "1: The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    VALUE2 = 1,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WA` reader - Word Address"]
pub type WA_R = crate::BitReader<WA_A>;
impl WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::VALUE1,
            true => WA_A::VALUE2,
        }
    }
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WA_A::VALUE1
    }
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WA_A::VALUE2
    }
}
#[doc = "Field `WA` writer - Word Address"]
pub type WA_W<'a, REG> = crate::BitWriter<'a, REG, WA_A>;
impl<'a, REG> WA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WA_A::VALUE1)
    }
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WA_A::VALUE2)
    }
}
#[doc = "Transmitted Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSOF_A {
    #[doc = "0: The latest data word transmission has not been started for the first word of a data frame."]
    VALUE1 = 0,
    #[doc = "1: The latest data word transmission has been started for the first word of a data frame."]
    VALUE2 = 1,
}
impl From<TSOF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOF` reader - Transmitted Start Of Frame"]
pub type TSOF_R = crate::BitReader<TSOF_A>;
impl TSOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSOF_A {
        match self.bits {
            false => TSOF_A::VALUE1,
            true => TSOF_A::VALUE2,
        }
    }
    #[doc = "The latest data word transmission has not been started for the first word of a data frame."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSOF_A::VALUE1
    }
    #[doc = "The latest data word transmission has been started for the first word of a data frame."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSOF_A::VALUE2
    }
}
#[doc = "Transmission Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TV_A {
    #[doc = "0: The latest start of a data word transmission has taken place while no valid data was available. As a result, the transmission of a data words with passive level (SCTR.PDL) has been started."]
    VALUE1 = 0,
    #[doc = "1: The latest start of a data word transmission has taken place with valid data from TBUF."]
    VALUE2 = 1,
}
impl From<TV_A> for bool {
    #[inline(always)]
    fn from(variant: TV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TV` reader - Transmission Valid"]
pub type TV_R = crate::BitReader<TV_A>;
impl TV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TV_A {
        match self.bits {
            false => TV_A::VALUE1,
            true => TV_A::VALUE2,
        }
    }
    #[doc = "The latest start of a data word transmission has taken place while no valid data was available. As a result, the transmission of a data words with passive level (SCTR.PDL) has been started."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TV_A::VALUE1
    }
    #[doc = "The latest start of a data word transmission has taken place with valid data from TBUF."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TV_A::VALUE2
    }
}
#[doc = "Transmission Valid Cumulated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TVC_A {
    #[doc = "0: Since TVC has been set, at least one data buffer underflow condition has occurred."]
    VALUE1 = 0,
    #[doc = "1: Since TVC has been set, no data buffer underflow condition has occurred."]
    VALUE2 = 1,
}
impl From<TVC_A> for bool {
    #[inline(always)]
    fn from(variant: TVC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TVC` reader - Transmission Valid Cumulated"]
pub type TVC_R = crate::BitReader<TVC_A>;
impl TVC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TVC_A {
        match self.bits {
            false => TVC_A::VALUE1,
            true => TVC_A::VALUE2,
        }
    }
    #[doc = "Since TVC has been set, at least one data buffer underflow condition has occurred."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TVC_A::VALUE1
    }
    #[doc = "Since TVC has been set, no data buffer underflow condition has occurred."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TVC_A::VALUE2
    }
}
#[doc = "Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: The trigger event has not yet been detected. A transmission of the data word in TBUF can not be started."]
    VALUE1 = 0,
    #[doc = "1: The trigger event has been detected (or the trigger mechanism is switched off) and a transmission of the data word in TBUF can not be started."]
    VALUE2 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Trigger Event"]
pub type TE_R = crate::BitReader<TE_A>;
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::VALUE1,
            true => TE_A::VALUE2,
        }
    }
    #[doc = "The trigger event has not yet been detected. A transmission of the data word in TBUF can not be started."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TE_A::VALUE1
    }
    #[doc = "The trigger event has been detected (or the trigger mechanism is switched off) and a transmission of the data word in TBUF can not be started."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TE_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - WLE Mode"]
    #[inline(always)]
    pub fn wlemd(&self) -> WLEMD_R {
        WLEMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline(always)]
    pub fn selmd(&self) -> SELMD_R {
        SELMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline(always)]
    pub fn flemd(&self) -> FLEMD_R {
        FLEMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline(always)]
    pub fn wamd(&self) -> WAMD_R {
        WAMD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline(always)]
    pub fn hpcmd(&self) -> HPCMD_R {
        HPCMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Valid"]
    #[inline(always)]
    pub fn tdv(&self) -> TDV_R {
        TDV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline(always)]
    pub fn tdssm(&self) -> TDSSM_R {
        TDSSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline(always)]
    pub fn tden(&self) -> TDEN_R {
        TDEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline(always)]
    pub fn tdvtr(&self) -> TDVTR_R {
        TDVTR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmitted Start Of Frame"]
    #[inline(always)]
    pub fn tsof(&self) -> TSOF_R {
        TSOF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmission Valid"]
    #[inline(always)]
    pub fn tv(&self) -> TV_R {
        TV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmission Valid Cumulated"]
    #[inline(always)]
    pub fn tvc(&self) -> TVC_R {
        TVC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Trigger Event"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WLE Mode"]
    #[inline(always)]
    pub fn wlemd(&mut self) -> WLEMD_W<TCSR_SPEC> {
        WLEMD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline(always)]
    pub fn selmd(&mut self) -> SELMD_W<TCSR_SPEC> {
        SELMD_W::new(self, 1)
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline(always)]
    pub fn flemd(&mut self) -> FLEMD_W<TCSR_SPEC> {
        FLEMD_W::new(self, 2)
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline(always)]
    pub fn wamd(&mut self) -> WAMD_W<TCSR_SPEC> {
        WAMD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline(always)]
    pub fn hpcmd(&mut self) -> HPCMD_W<TCSR_SPEC> {
        HPCMD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<TCSR_SPEC> {
        SOF_W::new(self, 5)
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W<TCSR_SPEC> {
        EOF_W::new(self, 6)
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline(always)]
    pub fn tdssm(&mut self) -> TDSSM_W<TCSR_SPEC> {
        TDSSM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline(always)]
    pub fn tden(&mut self) -> TDEN_W<TCSR_SPEC> {
        TDEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline(always)]
    pub fn tdvtr(&mut self) -> TDVTR_W<TCSR_SPEC> {
        TDVTR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W<TCSR_SPEC> {
        WA_W::new(self, 13)
    }
}
#[doc = "Transmit Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCSR_SPEC;
impl crate::RegisterSpec for TCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcsr::R`](R) reader structure"]
impl crate::Readable for TCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcsr::W`](W) writer structure"]
impl crate::Writable for TCSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCSR to value 0"]
impl crate::Resettable for TCSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
