#[doc = "Register `TRANSFER_MODE` reader"]
pub type R = crate::R<TransferModeSpec>;
#[doc = "Register `TRANSFER_MODE` writer"]
pub type W = crate::W<TransferModeSpec>;
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockCountEn {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<BlockCountEn> for bool {
    #[inline(always)]
    fn from(variant: BlockCountEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCK_COUNT_EN` reader - Block Count Enable"]
pub type BlockCountEnR = crate::BitReader<BlockCountEn>;
impl BlockCountEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BlockCountEn {
        match self.bits {
            false => BlockCountEn::Value1,
            true => BlockCountEn::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BlockCountEn::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BlockCountEn::Value2
    }
}
#[doc = "Field `BLOCK_COUNT_EN` writer - Block Count Enable"]
pub type BlockCountEnW<'a, REG> = crate::BitWriter<'a, REG, BlockCountEn>;
impl<'a, REG> BlockCountEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(BlockCountEn::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(BlockCountEn::Value2)
    }
}
#[doc = "Auto CMD Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AcmdEn {
    #[doc = "0: Auto Command Disabled"]
    Value1 = 0,
    #[doc = "1: Auto CMD12 Enable"]
    Value2 = 1,
}
impl From<AcmdEn> for u8 {
    #[inline(always)]
    fn from(variant: AcmdEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AcmdEn {
    type Ux = u8;
}
#[doc = "Field `ACMD_EN` reader - Auto CMD Enable"]
pub type AcmdEnR = crate::FieldReader<AcmdEn>;
impl AcmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AcmdEn> {
        match self.bits {
            0 => Some(AcmdEn::Value1),
            1 => Some(AcmdEn::Value2),
            _ => None,
        }
    }
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AcmdEn::Value1
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AcmdEn::Value2
    }
}
#[doc = "Field `ACMD_EN` writer - Auto CMD Enable"]
pub type AcmdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, AcmdEn>;
impl<'a, REG> AcmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto Command Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AcmdEn::Value1)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AcmdEn::Value2)
    }
}
#[doc = "Data Transfer Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxDirSelect {
    #[doc = "0: Write (Host to Card)"]
    Value1 = 0,
    #[doc = "1: Read (Card to Host)"]
    Value2 = 1,
}
impl From<TxDirSelect> for bool {
    #[inline(always)]
    fn from(variant: TxDirSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DIR_SELECT` reader - Data Transfer Direction Select"]
pub type TxDirSelectR = crate::BitReader<TxDirSelect>;
impl TxDirSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxDirSelect {
        match self.bits {
            false => TxDirSelect::Value1,
            true => TxDirSelect::Value2,
        }
    }
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxDirSelect::Value1
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxDirSelect::Value2
    }
}
#[doc = "Field `TX_DIR_SELECT` writer - Data Transfer Direction Select"]
pub type TxDirSelectW<'a, REG> = crate::BitWriter<'a, REG, TxDirSelect>;
impl<'a, REG> TxDirSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write (Host to Card)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxDirSelect::Value1)
    }
    #[doc = "Read (Card to Host)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxDirSelect::Value2)
    }
}
#[doc = "Multi / Single Block Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MultiBlockSelect {
    #[doc = "0: Single Block"]
    Value1 = 0,
    #[doc = "1: Multiple Block"]
    Value2 = 1,
}
impl From<MultiBlockSelect> for bool {
    #[inline(always)]
    fn from(variant: MultiBlockSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTI_BLOCK_SELECT` reader - Multi / Single Block Select"]
pub type MultiBlockSelectR = crate::BitReader<MultiBlockSelect>;
impl MultiBlockSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MultiBlockSelect {
        match self.bits {
            false => MultiBlockSelect::Value1,
            true => MultiBlockSelect::Value2,
        }
    }
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MultiBlockSelect::Value1
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MultiBlockSelect::Value2
    }
}
#[doc = "Field `MULTI_BLOCK_SELECT` writer - Multi / Single Block Select"]
pub type MultiBlockSelectW<'a, REG> = crate::BitWriter<'a, REG, MultiBlockSelect>;
impl<'a, REG> MultiBlockSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single Block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MultiBlockSelect::Value1)
    }
    #[doc = "Multiple Block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MultiBlockSelect::Value2)
    }
}
#[doc = "Command Completion Signal Enable for CE-ATA Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmdCompAta {
    #[doc = "1: Device will send command completion Signal"]
    Value1 = 1,
    #[doc = "0: Device will not send command completion Signal"]
    Value2 = 0,
}
impl From<CmdCompAta> for bool {
    #[inline(always)]
    fn from(variant: CmdCompAta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_COMP_ATA` reader - Command Completion Signal Enable for CE-ATA Device"]
pub type CmdCompAtaR = crate::BitReader<CmdCompAta>;
impl CmdCompAtaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmdCompAta {
        match self.bits {
            true => CmdCompAta::Value1,
            false => CmdCompAta::Value2,
        }
    }
    #[doc = "Device will send command completion Signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CmdCompAta::Value1
    }
    #[doc = "Device will not send command completion Signal"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CmdCompAta::Value2
    }
}
#[doc = "Field `CMD_COMP_ATA` writer - Command Completion Signal Enable for CE-ATA Device"]
pub type CmdCompAtaW<'a, REG> = crate::BitWriter<'a, REG, CmdCompAta>;
impl<'a, REG> CmdCompAtaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device will send command completion Signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCompAta::Value1)
    }
    #[doc = "Device will not send command completion Signal"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CmdCompAta::Value2)
    }
}
impl R {
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn block_count_en(&self) -> BlockCountEnR {
        BlockCountEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto CMD Enable"]
    #[inline(always)]
    pub fn acmd_en(&self) -> AcmdEnR {
        AcmdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn tx_dir_select(&self) -> TxDirSelectR {
        TxDirSelectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    pub fn multi_block_select(&self) -> MultiBlockSelectR {
        MultiBlockSelectR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command Completion Signal Enable for CE-ATA Device"]
    #[inline(always)]
    pub fn cmd_comp_ata(&self) -> CmdCompAtaR {
        CmdCompAtaR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn block_count_en(&mut self) -> BlockCountEnW<TransferModeSpec> {
        BlockCountEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Auto CMD Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmd_en(&mut self) -> AcmdEnW<TransferModeSpec> {
        AcmdEnW::new(self, 2)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dir_select(&mut self) -> TxDirSelectW<TransferModeSpec> {
        TxDirSelectW::new(self, 4)
    }
    #[doc = "Bit 5 - Multi / Single Block Select"]
    #[inline(always)]
    #[must_use]
    pub fn multi_block_select(&mut self) -> MultiBlockSelectW<TransferModeSpec> {
        MultiBlockSelectW::new(self, 5)
    }
    #[doc = "Bit 6 - Command Completion Signal Enable for CE-ATA Device"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_comp_ata(&mut self) -> CmdCompAtaW<TransferModeSpec> {
        CmdCompAtaW::new(self, 6)
    }
}
#[doc = "Transfer Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transfer_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transfer_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransferModeSpec;
impl crate::RegisterSpec for TransferModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`transfer_mode::R`](R) reader structure"]
impl crate::Readable for TransferModeSpec {}
#[doc = "`write(|w| ..)` method takes [`transfer_mode::W`](W) writer structure"]
impl crate::Writable for TransferModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TRANSFER_MODE to value 0"]
impl crate::Resettable for TransferModeSpec {
    const RESET_VALUE: u16 = 0;
}
