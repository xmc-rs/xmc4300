#[doc = "Register `MII_CONT_STAT` reader"]
pub struct R(crate::R<MII_CONT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MII_CONT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MII_CONT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MII_CONT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MII_CONT_STAT` writer"]
pub struct W(crate::W<MII_CONT_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MII_CONT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MII_CONT_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MII_CONT_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W_EN` reader - Write enable"]
pub type W_EN_R = crate::BitReader<W_EN_A>;
#[doc = "Write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum W_EN_A {
    #[doc = "0: Write disabled"]
    VALUE1 = 0,
    #[doc = "1: Write enabled"]
    VALUE2 = 1,
}
impl From<W_EN_A> for bool {
    #[inline(always)]
    fn from(variant: W_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl W_EN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `MIC_PDI` reader - Management Interface can be controlled by PDI"]
pub type MIC_PDI_R = crate::BitReader<MIC_PDI_A>;
#[doc = "Management Interface can be controlled by PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MIC_PDI_A {
    #[doc = "0: Only ECAT control"]
    VALUE1 = 0,
    #[doc = "1: PDI control possible"]
    VALUE2 = 1,
}
impl From<MIC_PDI_A> for bool {
    #[inline(always)]
    fn from(variant: MIC_PDI_A) -> Self {
        variant as u8 != 0
    }
}
impl MIC_PDI_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `MI_LD` reader - MI link detection"]
pub type MI_LD_R = crate::BitReader<MI_LD_A>;
#[doc = "MI link detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MI_LD_A {
    #[doc = "0: Not available"]
    VALUE1 = 0,
    #[doc = "1: MI link detection active"]
    VALUE2 = 1,
}
impl From<MI_LD_A> for bool {
    #[inline(always)]
    fn from(variant: MI_LD_A) -> Self {
        variant as u8 != 0
    }
}
impl MI_LD_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `PHY_ADDR` reader - PHY address of port 0"]
pub type PHY_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_REG` reader - Command register"]
pub type CMD_REG_R = crate::FieldReader<u8, CMD_REG_A>;
#[doc = "Command register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_REG_A {
    #[doc = "0: No command/MII idle (clear error bits)"]
    VALUE1 = 0,
    #[doc = "1: Read"]
    VALUE2 = 1,
    #[doc = "2: Write"]
    VALUE3 = 2,
}
impl From<CMD_REG_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_REG_A) -> Self {
        variant as _
    }
}
impl CMD_REG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_REG_A> {
        match self.bits {
            0 => Some(CMD_REG_A::VALUE1),
            1 => Some(CMD_REG_A::VALUE2),
            2 => Some(CMD_REG_A::VALUE3),
            _ => None,
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
#[doc = "Field `CMD_REG` writer - Command register"]
pub type CMD_REG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MII_CONT_STAT_SPEC, u8, CMD_REG_A, 2, O>;
impl<'a, const O: u8> CMD_REG_W<'a, O> {
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
}
#[doc = "Field `ERROR` reader - Command error"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Command error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: Last Command was successful"]
    VALUE1 = 0,
    #[doc = "1: Invalid command or write command without Write Enable"]
    VALUE2 = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `BUSY` reader - Busy"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: MI control state machine is idle"]
    VALUE1 = 0,
    #[doc = "1: MI control state machine is active"]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
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
        W_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Management Interface can be controlled by PDI"]
    #[inline(always)]
    pub fn mic_pdi(&self) -> MIC_PDI_R {
        MIC_PDI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MI link detection"]
    #[inline(always)]
    pub fn mi_ld(&self) -> MI_LD_R {
        MI_LD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - PHY address of port 0"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PHY_ADDR_R {
        PHY_ADDR_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&self) -> CMD_REG_R {
        CMD_REG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 14 - Command error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Command register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_reg(&mut self) -> CMD_REG_W<8> {
        CMD_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Management Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mii_cont_stat](index.html) module"]
pub struct MII_CONT_STAT_SPEC;
impl crate::RegisterSpec for MII_CONT_STAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mii_cont_stat::R](R) reader structure"]
impl crate::Readable for MII_CONT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mii_cont_stat::W](W) writer structure"]
impl crate::Writable for MII_CONT_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MII_CONT_STAT to value 0"]
impl crate::Resettable for MII_CONT_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
