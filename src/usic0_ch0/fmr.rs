#[doc = "Writer for register FMR"]
pub type W = crate::W<u32, super::FMR>;
#[doc = "Register FMR `reset()`'s with value 0"]
impl crate::ResetValue for super::FMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Modify Transmit Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTDV_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: Bit TDV is set, TE is unchanged."]
    VALUE2,
    #[doc = "2: Bits TDV and TE are cleared."]
    VALUE3,
}
impl From<MTDV_AW> for u8 {
    #[inline(always)]
    fn from(variant: MTDV_AW) -> Self {
        match variant {
            MTDV_AW::VALUE1 => 0,
            MTDV_AW::VALUE2 => 1,
            MTDV_AW::VALUE3 => 2,
        }
    }
}
#[doc = "Write proxy for field `MTDV`"]
pub struct MTDV_W<'a> {
    w: &'a mut W,
}
impl<'a> MTDV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTDV_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTDV_AW::VALUE1)
    }
    #[doc = "Bit TDV is set, TE is unchanged."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTDV_AW::VALUE2)
    }
    #[doc = "Bits TDV and TE are cleared."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MTDV_AW::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Activate Bit TVC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATVC_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: Bit TCSR.TVC is set."]
    VALUE2,
}
impl From<ATVC_AW> for bool {
    #[inline(always)]
    fn from(variant: ATVC_AW) -> Self {
        match variant {
            ATVC_AW::VALUE1 => false,
            ATVC_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `ATVC`"]
pub struct ATVC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATVC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATVC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ATVC_AW::VALUE1)
    }
    #[doc = "Bit TCSR.TVC is set."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ATVC_AW::VALUE2)
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
#[doc = "Clear Bits RDV for RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRDV0_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    VALUE2,
}
impl From<CRDV0_AW> for bool {
    #[inline(always)]
    fn from(variant: CRDV0_AW) -> Self {
        match variant {
            CRDV0_AW::VALUE1 => false,
            CRDV0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CRDV0`"]
pub struct CRDV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRDV0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRDV0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRDV0_AW::VALUE1)
    }
    #[doc = "Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRDV0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Clear Bit RDV for RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRDV1_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    VALUE2,
}
impl From<CRDV1_AW> for bool {
    #[inline(always)]
    fn from(variant: CRDV1_AW) -> Self {
        match variant {
            CRDV1_AW::VALUE1 => false,
            CRDV1_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CRDV1`"]
pub struct CRDV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CRDV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRDV1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRDV1_AW::VALUE1)
    }
    #[doc = "Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRDV1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIO0_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2,
}
impl From<SIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO0_AW) -> Self {
        match variant {
            SIO0_AW::VALUE1 => false,
            SIO0_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SIO0`"]
pub struct SIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIO0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO0_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO0_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIO1_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2,
}
impl From<SIO1_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO1_AW) -> Self {
        match variant {
            SIO1_AW::VALUE1 => false,
            SIO1_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SIO1`"]
pub struct SIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIO1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO1_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO1_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIO2_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2,
}
impl From<SIO2_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO2_AW) -> Self {
        match variant {
            SIO2_AW::VALUE1 => false,
            SIO2_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SIO2`"]
pub struct SIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIO2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO2_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO2_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIO3_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2,
}
impl From<SIO3_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO3_AW) -> Self {
        match variant {
            SIO3_AW::VALUE1 => false,
            SIO3_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SIO3`"]
pub struct SIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIO3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO3_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO3_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIO4_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2,
}
impl From<SIO4_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO4_AW) -> Self {
        match variant {
            SIO4_AW::VALUE1 => false,
            SIO4_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SIO4`"]
pub struct SIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIO4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO4_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO4_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIO5_AW {
    #[doc = "0: No action."]
    VALUE1,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2,
}
impl From<SIO5_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO5_AW) -> Self {
        match variant {
            SIO5_AW::VALUE1 => false,
            SIO5_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `SIO5`"]
pub struct SIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> SIO5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIO5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIO5_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIO5_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:1 - Modify Transmit Data Valid"]
    #[inline(always)]
    pub fn mtdv(&mut self) -> MTDV_W {
        MTDV_W { w: self }
    }
    #[doc = "Bit 4 - Activate Bit TVC"]
    #[inline(always)]
    pub fn atvc(&mut self) -> ATVC_W {
        ATVC_W { w: self }
    }
    #[doc = "Bit 14 - Clear Bits RDV for RBUF0"]
    #[inline(always)]
    pub fn crdv0(&mut self) -> CRDV0_W {
        CRDV0_W { w: self }
    }
    #[doc = "Bit 15 - Clear Bit RDV for RBUF1"]
    #[inline(always)]
    pub fn crdv1(&mut self) -> CRDV1_W {
        CRDV1_W { w: self }
    }
    #[doc = "Bit 16 - Set Interrupt Output SRx"]
    #[inline(always)]
    pub fn sio0(&mut self) -> SIO0_W {
        SIO0_W { w: self }
    }
    #[doc = "Bit 17 - Set Interrupt Output SRx"]
    #[inline(always)]
    pub fn sio1(&mut self) -> SIO1_W {
        SIO1_W { w: self }
    }
    #[doc = "Bit 18 - Set Interrupt Output SRx"]
    #[inline(always)]
    pub fn sio2(&mut self) -> SIO2_W {
        SIO2_W { w: self }
    }
    #[doc = "Bit 19 - Set Interrupt Output SRx"]
    #[inline(always)]
    pub fn sio3(&mut self) -> SIO3_W {
        SIO3_W { w: self }
    }
    #[doc = "Bit 20 - Set Interrupt Output SRx"]
    #[inline(always)]
    pub fn sio4(&mut self) -> SIO4_W {
        SIO4_W { w: self }
    }
    #[doc = "Bit 21 - Set Interrupt Output SRx"]
    #[inline(always)]
    pub fn sio5(&mut self) -> SIO5_W {
        SIO5_W { w: self }
    }
}
