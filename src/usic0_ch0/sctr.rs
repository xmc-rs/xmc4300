#[doc = "Register `SCTR` reader"]
pub type R = crate::R<SCTR_SPEC>;
#[doc = "Register `SCTR` writer"]
pub type W = crate::W<SCTR_SPEC>;
#[doc = "Field `SDIR` reader - Shift Direction"]
pub type SDIR_R = crate::BitReader<SDIR_A>;
#[doc = "Shift Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIR_A {
    #[doc = "0: Shift LSB first. The first data bit of a data word is located at bit position 0."]
    VALUE1 = 0,
    #[doc = "1: Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    VALUE2 = 1,
}
impl From<SDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDIR_A {
        match self.bits {
            false => SDIR_A::VALUE1,
            true => SDIR_A::VALUE2,
        }
    }
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SDIR_A::VALUE1
    }
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SDIR_A::VALUE2
    }
}
#[doc = "Field `SDIR` writer - Shift Direction"]
pub type SDIR_W<'a, REG> = crate::BitWriter<'a, REG, SDIR_A>;
impl<'a, REG> SDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shift LSB first. The first data bit of a data word is located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SDIR_A::VALUE1)
    }
    #[doc = "Shift MSB first. The first data bit of a data word is located at the bit position given by bit field SCTR.WLE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SDIR_A::VALUE2)
    }
}
#[doc = "Field `PDL` reader - Passive Data Level"]
pub type PDL_R = crate::BitReader<PDL_A>;
#[doc = "Passive Data Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDL_A {
    #[doc = "0: The passive data level is 0."]
    VALUE1 = 0,
    #[doc = "1: The passive data level is 1."]
    VALUE2 = 1,
}
impl From<PDL_A> for bool {
    #[inline(always)]
    fn from(variant: PDL_A) -> Self {
        variant as u8 != 0
    }
}
impl PDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PDL_A {
        match self.bits {
            false => PDL_A::VALUE1,
            true => PDL_A::VALUE2,
        }
    }
    #[doc = "The passive data level is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PDL_A::VALUE1
    }
    #[doc = "The passive data level is 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PDL_A::VALUE2
    }
}
#[doc = "Field `PDL` writer - Passive Data Level"]
pub type PDL_W<'a, REG> = crate::BitWriter<'a, REG, PDL_A>;
impl<'a, REG> PDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The passive data level is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PDL_A::VALUE1)
    }
    #[doc = "The passive data level is 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PDL_A::VALUE2)
    }
}
#[doc = "Field `DSM` reader - Data Shift Mode"]
pub type DSM_R = crate::FieldReader<DSM_A>;
#[doc = "Data Shift Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSM_A {
    #[doc = "0: Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    VALUE1 = 0,
    #[doc = "2: Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    VALUE3 = 2,
    #[doc = "3: Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    VALUE4 = 3,
}
impl From<DSM_A> for u8 {
    #[inline(always)]
    fn from(variant: DSM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSM_A {
    type Ux = u8;
}
impl DSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DSM_A> {
        match self.bits {
            0 => Some(DSM_A::VALUE1),
            2 => Some(DSM_A::VALUE3),
            3 => Some(DSM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSM_A::VALUE1
    }
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DSM_A::VALUE3
    }
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DSM_A::VALUE4
    }
}
#[doc = "Field `DSM` writer - Data Shift Mode"]
pub type DSM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DSM_A>;
impl<'a, REG> DSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive and transmit data is shifted in and out one bit at a time through DX0 and DOUT0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DSM_A::VALUE1)
    }
    #[doc = "Receive and transmit data is shifted in and out two bits at a time through two input stages (DX0 and DX3) and DOUT\\[1:0\\]
respectively."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DSM_A::VALUE3)
    }
    #[doc = "Receive and transmit data is shifted in and out four bits at a time through four input stages (DX0, DX\\[5:3\\]) and DOUT\\[3:0\\]
respectively."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DSM_A::VALUE4)
    }
}
#[doc = "Field `HPCDIR` reader - Port Control Direction"]
pub type HPCDIR_R = crate::BitReader<HPCDIR_A>;
#[doc = "Port Control Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPCDIR_A {
    #[doc = "0: The pin(s) with hardware pin control enabled are selected to be in input mode."]
    VALUE1 = 0,
    #[doc = "1: The pin(s) with hardware pin control enabled are selected to be in output mode."]
    VALUE2 = 1,
}
impl From<HPCDIR_A> for bool {
    #[inline(always)]
    fn from(variant: HPCDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl HPCDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HPCDIR_A {
        match self.bits {
            false => HPCDIR_A::VALUE1,
            true => HPCDIR_A::VALUE2,
        }
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HPCDIR_A::VALUE1
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HPCDIR_A::VALUE2
    }
}
#[doc = "Field `HPCDIR` writer - Port Control Direction"]
pub type HPCDIR_W<'a, REG> = crate::BitWriter<'a, REG, HPCDIR_A>;
impl<'a, REG> HPCDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in input mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HPCDIR_A::VALUE1)
    }
    #[doc = "The pin(s) with hardware pin control enabled are selected to be in output mode."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HPCDIR_A::VALUE2)
    }
}
#[doc = "Field `DOCFG` reader - Data Output Configuration"]
pub type DOCFG_R = crate::FieldReader<DOCFG_A>;
#[doc = "Data Output Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DOCFG_A {
    #[doc = "0: DOUTx = shift data value"]
    VALUE1 = 0,
    #[doc = "1: DOUTx = inverted shift data value"]
    VALUE2 = 1,
}
impl From<DOCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DOCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DOCFG_A {
    type Ux = u8;
}
impl DOCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DOCFG_A> {
        match self.bits {
            0 => Some(DOCFG_A::VALUE1),
            1 => Some(DOCFG_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "DOUTx = shift data value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DOCFG_A::VALUE1
    }
    #[doc = "DOUTx = inverted shift data value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DOCFG_A::VALUE2
    }
}
#[doc = "Field `DOCFG` writer - Data Output Configuration"]
pub type DOCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DOCFG_A>;
impl<'a, REG> DOCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DOUTx = shift data value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DOCFG_A::VALUE1)
    }
    #[doc = "DOUTx = inverted shift data value"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DOCFG_A::VALUE2)
    }
}
#[doc = "Field `TRM` reader - Transmission Mode"]
pub type TRM_R = crate::FieldReader<TRM_A>;
#[doc = "Transmission Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRM_A {
    #[doc = "0: The shift control signal is considered as inactive and data frame transfers are not possible."]
    VALUE1 = 0,
    #[doc = "1: The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    VALUE2 = 1,
    #[doc = "2: The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    VALUE3 = 2,
    #[doc = "3: The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    VALUE4 = 3,
}
impl From<TRM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRM_A {
    type Ux = u8;
}
impl TRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRM_A {
        match self.bits {
            0 => TRM_A::VALUE1,
            1 => TRM_A::VALUE2,
            2 => TRM_A::VALUE3,
            3 => TRM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRM_A::VALUE1
    }
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRM_A::VALUE2
    }
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRM_A::VALUE3
    }
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRM_A::VALUE4
    }
}
#[doc = "Field `TRM` writer - Transmission Mode"]
pub type TRM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TRM_A>;
impl<'a, REG> TRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The shift control signal is considered as inactive and data frame transfers are not possible."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRM_A::VALUE1)
    }
    #[doc = "The shift control signal is considered active if it is at 1-level. This is the setting to be programmed to allow data transfers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRM_A::VALUE2)
    }
    #[doc = "The shift control signal is considered active if it is at 0-level. It is recommended to avoid this setting and to use the inversion in the DX2 stage in case of a low-active signal."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TRM_A::VALUE3)
    }
    #[doc = "The shift control signal is considered active without referring to the actual signal level. Data frame transfer is possible after each edge of the signal."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TRM_A::VALUE4)
    }
}
#[doc = "Field `FLE` reader - Frame Length"]
pub type FLE_R = crate::FieldReader;
#[doc = "Field `FLE` writer - Frame Length"]
pub type FLE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WLE` reader - Word Length"]
pub type WLE_R = crate::FieldReader<WLE_A>;
#[doc = "Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WLE_A {
    #[doc = "0: The data word contains 1 data bit located at bit position 0."]
    VALUE1 = 0,
    #[doc = "1: The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    VALUE2 = 1,
    #[doc = "14: The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    VALUE3 = 14,
    #[doc = "15: The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    VALUE4 = 15,
}
impl From<WLE_A> for u8 {
    #[inline(always)]
    fn from(variant: WLE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WLE_A {
    type Ux = u8;
}
impl WLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WLE_A> {
        match self.bits {
            0 => Some(WLE_A::VALUE1),
            1 => Some(WLE_A::VALUE2),
            14 => Some(WLE_A::VALUE3),
            15 => Some(WLE_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WLE_A::VALUE1
    }
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WLE_A::VALUE2
    }
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WLE_A::VALUE3
    }
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WLE_A::VALUE4
    }
}
#[doc = "Field `WLE` writer - Word Length"]
pub type WLE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, WLE_A>;
impl<'a, REG> WLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data word contains 1 data bit located at bit position 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WLE_A::VALUE1)
    }
    #[doc = "The data word contains 2 data bits located at bit positions \\[1:0\\]."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WLE_A::VALUE2)
    }
    #[doc = "The data word contains 15 data bits located at bit positions \\[14:0\\]."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WLE_A::VALUE3)
    }
    #[doc = "The data word contains 16 data bits located at bit positions \\[15:0\\]."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(WLE_A::VALUE4)
    }
}
impl R {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    pub fn sdir(&self) -> SDIR_R {
        SDIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    pub fn pdl(&self) -> PDL_R {
        PDL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    pub fn dsm(&self) -> DSM_R {
        DSM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    pub fn hpcdir(&self) -> HPCDIR_R {
        HPCDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    pub fn docfg(&self) -> DOCFG_R {
        DOCFG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    pub fn trm(&self) -> TRM_R {
        TRM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    pub fn fle(&self) -> FLE_R {
        FLE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    pub fn wle(&self) -> WLE_R {
        WLE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Shift Direction"]
    #[inline(always)]
    #[must_use]
    pub fn sdir(&mut self) -> SDIR_W<SCTR_SPEC> {
        SDIR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Passive Data Level"]
    #[inline(always)]
    #[must_use]
    pub fn pdl(&mut self) -> PDL_W<SCTR_SPEC> {
        PDL_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Data Shift Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dsm(&mut self) -> DSM_W<SCTR_SPEC> {
        DSM_W::new(self, 2)
    }
    #[doc = "Bit 4 - Port Control Direction"]
    #[inline(always)]
    #[must_use]
    pub fn hpcdir(&mut self) -> HPCDIR_W<SCTR_SPEC> {
        HPCDIR_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Data Output Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn docfg(&mut self) -> DOCFG_W<SCTR_SPEC> {
        DOCFG_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Transmission Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trm(&mut self) -> TRM_W<SCTR_SPEC> {
        TRM_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Frame Length"]
    #[inline(always)]
    #[must_use]
    pub fn fle(&mut self) -> FLE_W<SCTR_SPEC> {
        FLE_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn wle(&mut self) -> WLE_W<SCTR_SPEC> {
        WLE_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Shift Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTR_SPEC;
impl crate::RegisterSpec for SCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctr::R`](R) reader structure"]
impl crate::Readable for SCTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctr::W`](W) writer structure"]
impl crate::Writable for SCTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCTR to value 0"]
impl crate::Resettable for SCTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
