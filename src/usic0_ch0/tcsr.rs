#[doc = "Reader of register TCSR"]
pub type R = crate::R<u32, super::TCSR>;
#[doc = "Writer for register TCSR"]
pub type W = crate::W<u32, super::TCSR>;
#[doc = "Register TCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WLE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLEMD_A {
    #[doc = "0: The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    VALUE1,
    #[doc = "1: The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    VALUE2,
}
impl From<WLEMD_A> for bool {
    #[inline(always)]
    fn from(variant: WLEMD_A) -> Self {
        match variant {
            WLEMD_A::VALUE1 => false,
            WLEMD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WLEMD`"]
pub type WLEMD_R = crate::R<bool, WLEMD_A>;
impl WLEMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLEMD_A {
        match self.bits {
            false => WLEMD_A::VALUE1,
            true => WLEMD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WLEMD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WLEMD_A::VALUE2
    }
}
#[doc = "Write proxy for field `WLEMD`"]
pub struct WLEMD_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLEMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WLEMD_A::VALUE1)
    }
    #[doc = "The automatic update of SCTR.WLE and TCSR.EOF is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WLEMD_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELMD_A {
    #[doc = "0: The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    VALUE1,
    #[doc = "1: The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    VALUE2,
}
impl From<SELMD_A> for bool {
    #[inline(always)]
    fn from(variant: SELMD_A) -> Self {
        match variant {
            SELMD_A::VALUE1 => false,
            SELMD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SELMD`"]
pub type SELMD_R = crate::R<bool, SELMD_A>;
impl SELMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMD_A {
        match self.bits {
            false => SELMD_A::VALUE1,
            true => SELMD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELMD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELMD_A::VALUE2
    }
}
#[doc = "Write proxy for field `SELMD`"]
pub struct SELMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SELMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELMD_A::VALUE1)
    }
    #[doc = "The automatic update of PCR.CTR\\[23:16\\] is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELMD_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "FLE Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEMD_A {
    #[doc = "0: The automatic update of FLE is disabled."]
    VALUE1,
    #[doc = "1: The automatic update of FLE is enabled."]
    VALUE2,
}
impl From<FLEMD_A> for bool {
    #[inline(always)]
    fn from(variant: FLEMD_A) -> Self {
        match variant {
            FLEMD_A::VALUE1 => false,
            FLEMD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FLEMD`"]
pub type FLEMD_R = crate::R<bool, FLEMD_A>;
impl FLEMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEMD_A {
        match self.bits {
            false => FLEMD_A::VALUE1,
            true => FLEMD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FLEMD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FLEMD_A::VALUE2
    }
}
#[doc = "Write proxy for field `FLEMD`"]
pub struct FLEMD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The automatic update of FLE is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLEMD_A::VALUE1)
    }
    #[doc = "The automatic update of FLE is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLEMD_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "WA Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAMD_A {
    #[doc = "0: The automatic update of bit WA is disabled."]
    VALUE1,
    #[doc = "1: The automatic update of bit WA is enabled."]
    VALUE2,
}
impl From<WAMD_A> for bool {
    #[inline(always)]
    fn from(variant: WAMD_A) -> Self {
        match variant {
            WAMD_A::VALUE1 => false,
            WAMD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WAMD`"]
pub type WAMD_R = crate::R<bool, WAMD_A>;
impl WAMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAMD_A {
        match self.bits {
            false => WAMD_A::VALUE1,
            true => WAMD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAMD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAMD_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAMD`"]
pub struct WAMD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The automatic update of bit WA is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAMD_A::VALUE1)
    }
    #[doc = "The automatic update of bit WA is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAMD_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Hardware Port Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCMD_A {
    #[doc = "0: The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    VALUE1,
    #[doc = "1: The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    VALUE2,
}
impl From<HPCMD_A> for bool {
    #[inline(always)]
    fn from(variant: HPCMD_A) -> Self {
        match variant {
            HPCMD_A::VALUE1 => false,
            HPCMD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `HPCMD`"]
pub type HPCMD_R = crate::R<bool, HPCMD_A>;
impl HPCMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPCMD_A {
        match self.bits {
            false => HPCMD_A::VALUE1,
            true => HPCMD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HPCMD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HPCMD_A::VALUE2
    }
}
#[doc = "Write proxy for field `HPCMD`"]
pub struct HPCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> HPCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPCMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HPCMD_A::VALUE1)
    }
    #[doc = "The automatic update of bits SCTR.DSM and SCTR.HPCDIR is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HPCMD_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: The data word in TBUF is not considered as first word of a frame."]
    VALUE1,
    #[doc = "1: The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    VALUE2,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        match variant {
            SOF_A::VALUE1 => false,
            SOF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, SOF_A>;
impl SOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::VALUE1,
            true => SOF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOF_A::VALUE2
    }
}
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The data word in TBUF is not considered as first word of a frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SOF_A::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as first word of a frame. A currently running frame is finished and MSLS becomes deactivated (respecting the programmed delays)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SOF_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "End Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_A {
    #[doc = "0: The data word in TBUF is not considered as last word of an SSC frame."]
    VALUE1,
    #[doc = "1: The data word in TBUF is considered as last word of an SSC frame."]
    VALUE2,
}
impl From<EOF_A> for bool {
    #[inline(always)]
    fn from(variant: EOF_A) -> Self {
        match variant {
            EOF_A::VALUE1 => false,
            EOF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EOF`"]
pub type EOF_R = crate::R<bool, EOF_A>;
impl EOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOF_A {
        match self.bits {
            false => EOF_A::VALUE1,
            true => EOF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EOF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EOF_A::VALUE2
    }
}
#[doc = "Write proxy for field `EOF`"]
pub struct EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The data word in TBUF is not considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EOF_A::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as last word of an SSC frame."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EOF_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Transmit Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDV_A {
    #[doc = "0: The data word in TBUF is not valid for transmission."]
    VALUE1,
    #[doc = "1: The data word in TBUF is valid for transmission and a transmission start is possible. New data should not be written to a TBUFx input location while TDV = 1."]
    VALUE2,
}
impl From<TDV_A> for bool {
    #[inline(always)]
    fn from(variant: TDV_A) -> Self {
        match variant {
            TDV_A::VALUE1 => false,
            TDV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TDV`"]
pub type TDV_R = crate::R<bool, TDV_A>;
impl TDV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDV_A {
        match self.bits {
            false => TDV_A::VALUE1,
            true => TDV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDV_A::VALUE2
    }
}
#[doc = "TBUF Data Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDSSM_A {
    #[doc = "0: The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    VALUE1,
    #[doc = "1: The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    VALUE2,
}
impl From<TDSSM_A> for bool {
    #[inline(always)]
    fn from(variant: TDSSM_A) -> Self {
        match variant {
            TDSSM_A::VALUE1 => false,
            TDSSM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TDSSM`"]
pub type TDSSM_R = crate::R<bool, TDSSM_A>;
impl TDSSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDSSM_A {
        match self.bits {
            false => TDSSM_A::VALUE1,
            true => TDSSM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDSSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDSSM_A::VALUE2
    }
}
#[doc = "Write proxy for field `TDSSM`"]
pub struct TDSSM_W<'a> {
    w: &'a mut W,
}
impl<'a> TDSSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDSSM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The data word in TBUF is not considered as invalid after it has been loaded into the transmit shift register. The loading of the TBUF data into the shift register does not clear TDV."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TDSSM_A::VALUE1)
    }
    #[doc = "The data word in TBUF is considered as invalid after it has been loaded into the shift register. In ASC and IIC mode, TDV is cleared with the TBI event, whereas in SSC and IIS mode, it is cleared with the RSI event. TDSSM = 1 has to be programmed if an optional data buffer is used."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TDSSM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "TBUF Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDEN_A {
    #[doc = "0: A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    VALUE1,
    #[doc = "1: A transmission of the data word in TBUF can be started if TDV = 1."]
    VALUE2,
    #[doc = "2: A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    VALUE3,
    #[doc = "3: A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    VALUE4,
}
impl From<TDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TDEN_A) -> Self {
        match variant {
            TDEN_A::VALUE1 => 0,
            TDEN_A::VALUE2 => 1,
            TDEN_A::VALUE3 => 2,
            TDEN_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `TDEN`"]
pub type TDEN_R = crate::R<u8, TDEN_A>;
impl TDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDEN_A {
        match self.bits {
            0 => TDEN_A::VALUE1,
            1 => TDEN_A::VALUE2,
            2 => TDEN_A::VALUE3,
            3 => TDEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TDEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TDEN_A::VALUE4
    }
}
#[doc = "Write proxy for field `TDEN`"]
pub struct TDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "A transmission start of the data word in TBUF is disabled. If a transmission is started, the passive data level is sent out."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TDEN_A::VALUE1)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TDEN_A::VALUE2)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 0."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TDEN_A::VALUE3)
    }
    #[doc = "A transmission of the data word in TBUF can be started if TDV = 1 while DX2S = 1."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TDEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "TBUF Data Valid Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDVTR_A {
    #[doc = "0: Bit TCSR.TE is permanently set."]
    VALUE1,
    #[doc = "1: Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    VALUE2,
}
impl From<TDVTR_A> for bool {
    #[inline(always)]
    fn from(variant: TDVTR_A) -> Self {
        match variant {
            TDVTR_A::VALUE1 => false,
            TDVTR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TDVTR`"]
pub type TDVTR_R = crate::R<bool, TDVTR_A>;
impl TDVTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDVTR_A {
        match self.bits {
            false => TDVTR_A::VALUE1,
            true => TDVTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TDVTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TDVTR_A::VALUE2
    }
}
#[doc = "Write proxy for field `TDVTR`"]
pub struct TDVTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDVTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDVTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit TCSR.TE is permanently set."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TDVTR_A::VALUE1)
    }
    #[doc = "Bit TCSR.TE is set if DX2T becomes active while TDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TDVTR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Word Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WA_A {
    #[doc = "0: The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    VALUE1,
    #[doc = "1: The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    VALUE2,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        match variant {
            WA_A::VALUE1 => false,
            WA_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `WA`"]
pub type WA_R = crate::R<bool, WA_A>;
impl WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::VALUE1,
            true => WA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WA_A::VALUE2
    }
}
#[doc = "Write proxy for field `WA`"]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The data word in TBUF will be transmitted after a falling edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WA_A::VALUE1)
    }
    #[doc = "The data word in TBUF will be transmitted after a rising edge of WA has been detected (referring to PSR.WA)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WA_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Transmitted Start Of Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOF_A {
    #[doc = "0: The latest data word transmission has not been started for the first word of a data frame."]
    VALUE1,
    #[doc = "1: The latest data word transmission has been started for the first word of a data frame."]
    VALUE2,
}
impl From<TSOF_A> for bool {
    #[inline(always)]
    fn from(variant: TSOF_A) -> Self {
        match variant {
            TSOF_A::VALUE1 => false,
            TSOF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TSOF`"]
pub type TSOF_R = crate::R<bool, TSOF_A>;
impl TSOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSOF_A {
        match self.bits {
            false => TSOF_A::VALUE1,
            true => TSOF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSOF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSOF_A::VALUE2
    }
}
#[doc = "Transmission Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TV_A {
    #[doc = "0: The latest start of a data word transmission has taken place while no valid data was available. As a result, the transmission of a data words with passive level (SCTR.PDL) has been started."]
    VALUE1,
    #[doc = "1: The latest start of a data word transmission has taken place with valid data from TBUF."]
    VALUE2,
}
impl From<TV_A> for bool {
    #[inline(always)]
    fn from(variant: TV_A) -> Self {
        match variant {
            TV_A::VALUE1 => false,
            TV_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TV`"]
pub type TV_R = crate::R<bool, TV_A>;
impl TV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TV_A {
        match self.bits {
            false => TV_A::VALUE1,
            true => TV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TV_A::VALUE2
    }
}
#[doc = "Transmission Valid Cumulated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TVC_A {
    #[doc = "0: Since TVC has been set, at least one data buffer underflow condition has occurred."]
    VALUE1,
    #[doc = "1: Since TVC has been set, no data buffer underflow condition has occurred."]
    VALUE2,
}
impl From<TVC_A> for bool {
    #[inline(always)]
    fn from(variant: TVC_A) -> Self {
        match variant {
            TVC_A::VALUE1 => false,
            TVC_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TVC`"]
pub type TVC_R = crate::R<bool, TVC_A>;
impl TVC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TVC_A {
        match self.bits {
            false => TVC_A::VALUE1,
            true => TVC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TVC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TVC_A::VALUE2
    }
}
#[doc = "Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    #[doc = "0: The trigger event has not yet been detected. A transmission of the data word in TBUF can not be started."]
    VALUE1,
    #[doc = "1: The trigger event has been detected (or the trigger mechanism is switched off) and a transmission of the data word in TBUF can not be started."]
    VALUE2,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        match variant {
            TE_A::VALUE1 => false,
            TE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TE`"]
pub type TE_R = crate::R<bool, TE_A>;
impl TE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::VALUE1,
            true => TE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TE_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - WLE Mode"]
    #[inline(always)]
    pub fn wlemd(&self) -> WLEMD_R {
        WLEMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline(always)]
    pub fn selmd(&self) -> SELMD_R {
        SELMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline(always)]
    pub fn flemd(&self) -> FLEMD_R {
        FLEMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline(always)]
    pub fn wamd(&self) -> WAMD_R {
        WAMD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline(always)]
    pub fn hpcmd(&self) -> HPCMD_R {
        HPCMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Valid"]
    #[inline(always)]
    pub fn tdv(&self) -> TDV_R {
        TDV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline(always)]
    pub fn tdssm(&self) -> TDSSM_R {
        TDSSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline(always)]
    pub fn tden(&self) -> TDEN_R {
        TDEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline(always)]
    pub fn tdvtr(&self) -> TDVTR_R {
        TDVTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Transmitted Start Of Frame"]
    #[inline(always)]
    pub fn tsof(&self) -> TSOF_R {
        TSOF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmission Valid"]
    #[inline(always)]
    pub fn tv(&self) -> TV_R {
        TV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmission Valid Cumulated"]
    #[inline(always)]
    pub fn tvc(&self) -> TVC_R {
        TVC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Trigger Event"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WLE Mode"]
    #[inline(always)]
    pub fn wlemd(&mut self) -> WLEMD_W {
        WLEMD_W { w: self }
    }
    #[doc = "Bit 1 - Select Mode"]
    #[inline(always)]
    pub fn selmd(&mut self) -> SELMD_W {
        SELMD_W { w: self }
    }
    #[doc = "Bit 2 - FLE Mode"]
    #[inline(always)]
    pub fn flemd(&mut self) -> FLEMD_W {
        FLEMD_W { w: self }
    }
    #[doc = "Bit 3 - WA Mode"]
    #[inline(always)]
    pub fn wamd(&mut self) -> WAMD_W {
        WAMD_W { w: self }
    }
    #[doc = "Bit 4 - Hardware Port Control Mode"]
    #[inline(always)]
    pub fn hpcmd(&mut self) -> HPCMD_W {
        HPCMD_W { w: self }
    }
    #[doc = "Bit 5 - Start Of Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 6 - End Of Frame"]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W {
        EOF_W { w: self }
    }
    #[doc = "Bit 8 - TBUF Data Single Shot Mode"]
    #[inline(always)]
    pub fn tdssm(&mut self) -> TDSSM_W {
        TDSSM_W { w: self }
    }
    #[doc = "Bits 10:11 - TBUF Data Enable"]
    #[inline(always)]
    pub fn tden(&mut self) -> TDEN_W {
        TDEN_W { w: self }
    }
    #[doc = "Bit 12 - TBUF Data Valid Trigger"]
    #[inline(always)]
    pub fn tdvtr(&mut self) -> TDVTR_W {
        TDVTR_W { w: self }
    }
    #[doc = "Bit 13 - Word Address"]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
}
