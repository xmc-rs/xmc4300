#[doc = "Register `MII_CONT_STAT` reader"]
pub type R = crate::R<MiiContStatSpec>;
#[doc = "Register `MII_CONT_STAT` writer"]
pub type W = crate::W<MiiContStatSpec>;
#[doc = "Write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WEn {
    #[doc = "0: Write disabled"]
    Value1 = 0,
    #[doc = "1: Write enabled"]
    Value2 = 1,
}
impl From<WEn> for bool {
    #[inline(always)]
    fn from(variant: WEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `W_EN` reader - Write enable"]
pub type WEnR = crate::BitReader<WEn>;
impl WEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WEn {
        match self.bits {
            false => WEn::Value1,
            true => WEn::Value2,
        }
    }
    #[doc = "Write disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WEn::Value1
    }
    #[doc = "Write enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WEn::Value2
    }
}
#[doc = "Management Interface can be controlled by PDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MicPdi {
    #[doc = "0: Only ECAT control"]
    Value1 = 0,
    #[doc = "1: PDI control possible"]
    Value2 = 1,
}
impl From<MicPdi> for bool {
    #[inline(always)]
    fn from(variant: MicPdi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIC_PDI` reader - Management Interface can be controlled by PDI"]
pub type MicPdiR = crate::BitReader<MicPdi>;
impl MicPdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MicPdi {
        match self.bits {
            false => MicPdi::Value1,
            true => MicPdi::Value2,
        }
    }
    #[doc = "Only ECAT control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MicPdi::Value1
    }
    #[doc = "PDI control possible"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MicPdi::Value2
    }
}
#[doc = "MI link detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MiLd {
    #[doc = "0: Not available"]
    Value1 = 0,
    #[doc = "1: MI link detection active"]
    Value2 = 1,
}
impl From<MiLd> for bool {
    #[inline(always)]
    fn from(variant: MiLd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MI_LD` reader - MI link detection"]
pub type MiLdR = crate::BitReader<MiLd>;
impl MiLdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MiLd {
        match self.bits {
            false => MiLd::Value1,
            true => MiLd::Value2,
        }
    }
    #[doc = "Not available"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MiLd::Value1
    }
    #[doc = "MI link detection active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MiLd::Value2
    }
}
#[doc = "Field `PHY_ADDR` reader - PHY address of port 0"]
pub type PhyAddrR = crate::FieldReader;
#[doc = "Command register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmdReg {
    #[doc = "0: No command/MII idle (clear error bits)"]
    Value1 = 0,
    #[doc = "1: Read"]
    Value2 = 1,
    #[doc = "2: Write"]
    Value3 = 2,
}
impl From<CmdReg> for u8 {
    #[inline(always)]
    fn from(variant: CmdReg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CmdReg {
    type Ux = u8;
}
#[doc = "Field `CMD_REG` reader - Command register"]
pub type CmdRegR = crate::FieldReader<CmdReg>;
impl CmdRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CmdReg> {
        match self.bits {
            0 => Some(CmdReg::Value1),
            1 => Some(CmdReg::Value2),
            2 => Some(CmdReg::Value3),
            _ => None,
        }
    }
    #[doc = "No command/MII idle (clear error bits)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdReg::Value1
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdReg::Value2
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CmdReg::Value3
    }
}
#[doc = "Field `CMD_REG` writer - Command register"]
pub type CmdRegW<'a, REG> = crate::FieldWriter<'a, REG, 2, CmdReg>;
impl<'a, REG> CmdRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No command/MII idle (clear error bits)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdReg::Value1)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdReg::Value2)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CmdReg::Value3)
    }
}
#[doc = "Command error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Last Command was successful"]
    Value1 = 0,
    #[doc = "1: Invalid command or write command without Write Enable"]
    Value2 = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Command error"]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::Value1,
            true => Error::Value2,
        }
    }
    #[doc = "Last Command was successful"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Error::Value1
    }
    #[doc = "Invalid command or write command without Write Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Error::Value2
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: MI control state machine is idle"]
    Value1 = 0,
    #[doc = "1: MI control state machine is active"]
    Value2 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Value1,
            true => Busy::Value2,
        }
    }
    #[doc = "MI control state machine is idle"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Busy::Value1
    }
    #[doc = "MI control state machine is active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Busy::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Write enable"]
    #[inline(always)]
    pub fn w_en(&self) -> WEnR {
        WEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Management Interface can be controlled by PDI"]
    #[inline(always)]
    pub fn mic_pdi(&self) -> MicPdiR {
        MicPdiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MI link detection"]
    #[inline(always)]
    pub fn mi_ld(&self) -> MiLdR {
        MiLdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - PHY address of port 0"]
    #[inline(always)]
    pub fn phy_addr(&self) -> PhyAddrR {
        PhyAddrR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Command register"]
    #[inline(always)]
    pub fn cmd_reg(&self) -> CmdRegR {
        CmdRegR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 14 - Command error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Command register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_reg(&mut self) -> CmdRegW<MiiContStatSpec> {
        CmdRegW::new(self, 8)
    }
}
#[doc = "MII Management Control/Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mii_cont_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mii_cont_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiiContStatSpec;
impl crate::RegisterSpec for MiiContStatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mii_cont_stat::R`](R) reader structure"]
impl crate::Readable for MiiContStatSpec {}
#[doc = "`write(|w| ..)` method takes [`mii_cont_stat::W`](W) writer structure"]
impl crate::Writable for MiiContStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MII_CONT_STAT to value 0"]
impl crate::Resettable for MiiContStatSpec {
    const RESET_VALUE: u16 = 0;
}
