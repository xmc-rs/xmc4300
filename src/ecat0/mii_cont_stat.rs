#[doc = "Reader of register MII_CONT_STAT"]
pub type R = crate::R<u16, super::MII_CONT_STAT>;
#[doc = "Writer for register MII_CONT_STAT"]
pub type W = crate::W<u16, super::MII_CONT_STAT>;
#[doc = "Register MII_CONT_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::MII_CONT_STAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum W_EN_A {
    #[doc = "0: Write disabled"]
    VALUE1,
    #[doc = "1: Write enabled"]
    VALUE2,
}
impl From<W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: W_EN_A) -> Self {
        match variant {
            W_EN_A::VALUE1 => false,
            W_EN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `W_EN`"]
pub type W_EN_R = crate::R<bool, W_EN_A>;
impl W_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> W_EN_A {
        match self.bits {
            false => W_EN_A::VALUE1,
            true => W_EN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == W_EN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == W_EN_A::VALUE2
    }
}
#[doc = "Management Interface can be controlled by PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIC_PDI_A {
    #[doc = "0: Only ECAT control"]
    VALUE1,
    #[doc = "1: PDI control possible"]
    VALUE2,
}
impl From<MIC_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: MIC_PDI_A) -> Self {
        match variant {
            MIC_PDI_A::VALUE1 => false,
            MIC_PDI_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `MIC_PDI`"]
pub type MIC_PDI_R = crate::R<bool, MIC_PDI_A>;
impl MIC_PDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MIC_PDI_A {
        match self.bits {
            false => MIC_PDI_A::VALUE1,
            true => MIC_PDI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MIC_PDI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MIC_PDI_A::VALUE2
    }
}
#[doc = "MI link detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MI_LD_A {
    #[doc = "0: Not available"]
    VALUE1,
    #[doc = "1: MI link detection active"]
    VALUE2,
}
impl From<MI_LD_A> for bool {
    #[inline(always)]
    fn from(variant: MI_LD_A) -> Self {
        match variant {
            MI_LD_A::VALUE1 => false,
            MI_LD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `MI_LD`"]
pub type MI_LD_R = crate::R<bool, MI_LD_A>;
impl MI_LD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MI_LD_A {
        match self.bits {
            false => MI_LD_A::VALUE1,
            true => MI_LD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MI_LD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MI_LD_A::VALUE2
    }
}
#[doc = "Reader of field `PHY_ADDR`"]
pub type PHY_ADDR_R = crate::R<u8, u8>;
#[doc = "Command register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_REG_A {
    #[doc = "0: No command/MII idle (clear error bits)"]
    VALUE1,
    #[doc = "1: Read"]
    VALUE2,
    #[doc = "2: Write"]
    VALUE3,
}
impl From<CMD_REG_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_REG_A) -> Self {
        match variant {
            CMD_REG_A::VALUE1 => 0,
            CMD_REG_A::VALUE2 => 1,
            CMD_REG_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `CMD_REG`"]
pub type CMD_REG_R = crate::R<u8, CMD_REG_A>;
impl CMD_REG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMD_REG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMD_REG_A::VALUE1),
            1 => Val(CMD_REG_A::VALUE2),
            2 => Val(CMD_REG_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMD_REG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMD_REG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMD_REG_A::VALUE3
    }
}
#[doc = "Write proxy for field `CMD_REG`"]
pub struct CMD_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_REG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_REG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No command/MII idle (clear error bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMD_REG_A::VALUE1)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMD_REG_A::VALUE2)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMD_REG_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Command error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Last Command was successful"]
    VALUE1,
    #[doc = "1: Invalid command or write command without Write Enable"]
    VALUE2,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        match variant {
            ERROR_A::VALUE1 => false,
            ERROR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::VALUE1,
            true => ERROR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERROR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERROR_A::VALUE2
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: MI control state machine is idle"]
    VALUE1,
    #[doc = "1: MI control state machine is active"]
    VALUE2,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::VALUE1 => false,
            BUSY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Write enable"]
    #[inline(always)]
    pub fn w_en(&self) -> W_EN_R {
        W_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Management Interface can be controlled by PDI"]
    #[inline(always)]
    pub fn mic_pdi(&self) -> MIC_PDI_R {
        MIC_PDI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MI link detection"]
    #[inline(always)]
    pub fn mi_ld(&self) -> MI_LD_R {
        MI_LD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - PHY address of port 0"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&self) -> CMD_REG_R {
        CMD_REG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Command error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&mut self) -> CMD_REG_W {
        CMD_REG_W { w: self }
    }
}
