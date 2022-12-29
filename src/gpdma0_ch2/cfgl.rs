#[doc = "Register `CFGL` reader"]
pub struct R(crate::R<CFGL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGL` writer"]
pub struct W(crate::W<CFGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGL_SPEC>;
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
impl From<crate::W<CFGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_PRIOR` reader - Channel priority"]
pub type CH_PRIOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_PRIOR` writer - Channel priority"]
pub type CH_PRIOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CH_SUSP` reader - Channel Suspend"]
pub type CH_SUSP_R = crate::BitReader<CH_SUSP_A>;
#[doc = "Channel Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH_SUSP_A {
    #[doc = "0: Not suspended."]
    VALUE1 = 0,
    #[doc = "1: Suspend DMA transfer from the source."]
    VALUE2 = 1,
}
impl From<CH_SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: CH_SUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl CH_SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_SUSP_A {
        match self.bits {
            false => CH_SUSP_A::VALUE1,
            true => CH_SUSP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CH_SUSP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CH_SUSP_A::VALUE2
    }
}
#[doc = "Field `CH_SUSP` writer - Channel Suspend"]
pub type CH_SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGL_SPEC, CH_SUSP_A, O>;
impl<'a, const O: u8> CH_SUSP_W<'a, O> {
    #[doc = "Not suspended."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CH_SUSP_A::VALUE1)
    }
    #[doc = "Suspend DMA transfer from the source."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CH_SUSP_A::VALUE2)
    }
}
#[doc = "Field `FIFO_EMPTY` reader - Indicates if there is data left in the channel FIFO"]
pub type FIFO_EMPTY_R = crate::BitReader<FIFO_EMPTY_A>;
#[doc = "Indicates if there is data left in the channel FIFO\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_EMPTY_A {
    #[doc = "1: Channel FIFO empty"]
    VALUE1 = 1,
    #[doc = "0: Channel FIFO not empty"]
    VALUE2 = 0,
}
impl From<FIFO_EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_EMPTY_A {
        match self.bits {
            true => FIFO_EMPTY_A::VALUE1,
            false => FIFO_EMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FIFO_EMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FIFO_EMPTY_A::VALUE2
    }
}
#[doc = "Field `HS_SEL_DST` reader - Destination Software or Hardware Handshaking Select"]
pub type HS_SEL_DST_R = crate::BitReader<HS_SEL_DST_A>;
#[doc = "Destination Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_SEL_DST_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_DST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_DST_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_SEL_DST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_DST_A {
        match self.bits {
            false => HS_SEL_DST_A::VALUE1,
            true => HS_SEL_DST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_DST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_DST_A::VALUE2
    }
}
#[doc = "Field `HS_SEL_DST` writer - Destination Software or Hardware Handshaking Select"]
pub type HS_SEL_DST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGL_SPEC, HS_SEL_DST_A, O>;
impl<'a, const O: u8> HS_SEL_DST_W<'a, O> {
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_DST_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware- initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_DST_A::VALUE2)
    }
}
#[doc = "Field `HS_SEL_SRC` reader - Source Software or Hardware Handshaking Select"]
pub type HS_SEL_SRC_R = crate::BitReader<HS_SEL_SRC_A>;
#[doc = "Source Software or Hardware Handshaking Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_SEL_SRC_A {
    #[doc = "0: Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    VALUE1 = 0,
    #[doc = "1: Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    VALUE2 = 1,
}
impl From<HS_SEL_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_SEL_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_SRC_A {
        match self.bits {
            false => HS_SEL_SRC_A::VALUE1,
            true => HS_SEL_SRC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HS_SEL_SRC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HS_SEL_SRC_A::VALUE2
    }
}
#[doc = "Field `HS_SEL_SRC` writer - Source Software or Hardware Handshaking Select"]
pub type HS_SEL_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGL_SPEC, HS_SEL_SRC_A, O>;
impl<'a, const O: u8> HS_SEL_SRC_W<'a, O> {
    #[doc = "Hardware handshaking interface. Software-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::VALUE1)
    }
    #[doc = "Software handshaking interface. Hardware-initiated transaction requests are ignored."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::VALUE2)
    }
}
#[doc = "Field `LOCK_CH_L` reader - Channel Lock Level"]
pub type LOCK_CH_L_R = crate::FieldReader<u8, LOCK_CH_L_A>;
#[doc = "Channel Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_CH_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_CH_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_CH_L_A) -> Self {
        variant as _
    }
}
impl LOCK_CH_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_CH_L_A> {
        match self.bits {
            0 => Some(LOCK_CH_L_A::VALUE1),
            1 => Some(LOCK_CH_L_A::VALUE2),
            2 => Some(LOCK_CH_L_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_CH_L_A::VALUE3
    }
}
#[doc = "Field `LOCK_CH_L` writer - Channel Lock Level"]
pub type LOCK_CH_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGL_SPEC, u8, LOCK_CH_L_A, 2, O>;
impl<'a, const O: u8> LOCK_CH_L_W<'a, O> {
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::VALUE3)
    }
}
#[doc = "Field `LOCK_B_L` reader - Bus Lock Level"]
pub type LOCK_B_L_R = crate::FieldReader<u8, LOCK_B_L_A>;
#[doc = "Bus Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_B_L_A {
    #[doc = "0: Over complete DMA transfer"]
    VALUE1 = 0,
    #[doc = "1: Over complete DMA block transfer"]
    VALUE2 = 1,
    #[doc = "2: Over complete DMA transaction"]
    VALUE3 = 2,
}
impl From<LOCK_B_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_B_L_A) -> Self {
        variant as _
    }
}
impl LOCK_B_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_B_L_A> {
        match self.bits {
            0 => Some(LOCK_B_L_A::VALUE1),
            1 => Some(LOCK_B_L_A::VALUE2),
            2 => Some(LOCK_B_L_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOCK_B_L_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOCK_B_L_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LOCK_B_L_A::VALUE3
    }
}
#[doc = "Field `LOCK_B_L` writer - Bus Lock Level"]
pub type LOCK_B_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGL_SPEC, u8, LOCK_B_L_A, 2, O>;
impl<'a, const O: u8> LOCK_B_L_W<'a, O> {
    #[doc = "Over complete DMA transfer"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE1)
    }
    #[doc = "Over complete DMA block transfer"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE2)
    }
    #[doc = "Over complete DMA transaction"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LOCK_B_L_A::VALUE3)
    }
}
#[doc = "Field `LOCK_CH` reader - Channel Lock Bit"]
pub type LOCK_CH_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_CH` writer - Channel Lock Bit"]
pub type LOCK_CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGL_SPEC, bool, O>;
#[doc = "Field `LOCK_B` reader - Bus Lock Bit"]
pub type LOCK_B_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_B` writer - Bus Lock Bit"]
pub type LOCK_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGL_SPEC, bool, O>;
#[doc = "Field `DST_HS_POL` reader - Destination Handshaking Interface Polarity"]
pub type DST_HS_POL_R = crate::BitReader<DST_HS_POL_A>;
#[doc = "Destination Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DST_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<DST_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: DST_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl DST_HS_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_HS_POL_A {
        match self.bits {
            false => DST_HS_POL_A::VALUE1,
            true => DST_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DST_HS_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DST_HS_POL_A::VALUE2
    }
}
#[doc = "Field `DST_HS_POL` writer - Destination Handshaking Interface Polarity"]
pub type DST_HS_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGL_SPEC, DST_HS_POL_A, O>;
impl<'a, const O: u8> DST_HS_POL_W<'a, O> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DST_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DST_HS_POL_A::VALUE2)
    }
}
#[doc = "Field `SRC_HS_POL` reader - Source Handshaking Interface Polarity"]
pub type SRC_HS_POL_R = crate::BitReader<SRC_HS_POL_A>;
#[doc = "Source Handshaking Interface Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_HS_POL_A {
    #[doc = "0: Active high"]
    VALUE1 = 0,
    #[doc = "1: Active low"]
    VALUE2 = 1,
}
impl From<SRC_HS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_HS_POL_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_HS_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_HS_POL_A {
        match self.bits {
            false => SRC_HS_POL_A::VALUE1,
            true => SRC_HS_POL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRC_HS_POL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRC_HS_POL_A::VALUE2
    }
}
#[doc = "Field `SRC_HS_POL` writer - Source Handshaking Interface Polarity"]
pub type SRC_HS_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGL_SPEC, SRC_HS_POL_A, O>;
impl<'a, const O: u8> SRC_HS_POL_W<'a, O> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRC_HS_POL_A::VALUE1)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRC_HS_POL_A::VALUE2)
    }
}
#[doc = "Field `MAX_ABRST` reader - Maximum AMBA Burst Length"]
pub type MAX_ABRST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_ABRST` writer - Maximum AMBA Burst Length"]
pub type MAX_ABRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    pub fn ch_prior(&self) -> CH_PRIOR_R {
        CH_PRIOR_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    pub fn ch_susp(&self) -> CH_SUSP_R {
        CH_SUSP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates if there is data left in the channel FIFO"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HS_SEL_DST_R {
        HS_SEL_DST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HS_SEL_SRC_R {
        HS_SEL_SRC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LOCK_CH_L_R {
        LOCK_CH_L_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    pub fn lock_b_l(&self) -> LOCK_B_L_R {
        LOCK_B_L_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LOCK_CH_R {
        LOCK_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    pub fn lock_b(&self) -> LOCK_B_R {
        LOCK_B_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn dst_hs_pol(&self) -> DST_HS_POL_R {
        DST_HS_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    pub fn src_hs_pol(&self) -> SRC_HS_POL_R {
        SRC_HS_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    pub fn max_abrst(&self) -> MAX_ABRST_R {
        MAX_ABRST_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:7 - Channel priority"]
    #[inline(always)]
    #[must_use]
    pub fn ch_prior(&mut self) -> CH_PRIOR_W<5> {
        CH_PRIOR_W::new(self)
    }
    #[doc = "Bit 8 - Channel Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn ch_susp(&mut self) -> CH_SUSP_W<8> {
        CH_SUSP_W::new(self)
    }
    #[doc = "Bit 10 - Destination Software or Hardware Handshaking Select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_dst(&mut self) -> HS_SEL_DST_W<10> {
        HS_SEL_DST_W::new(self)
    }
    #[doc = "Bit 11 - Source Software or Hardware Handshaking Select"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_src(&mut self) -> HS_SEL_SRC_W<11> {
        HS_SEL_SRC_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel Lock Level"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch_l(&mut self) -> LOCK_CH_L_W<12> {
        LOCK_CH_L_W::new(self)
    }
    #[doc = "Bits 14:15 - Bus Lock Level"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b_l(&mut self) -> LOCK_B_L_W<14> {
        LOCK_B_L_W::new(self)
    }
    #[doc = "Bit 16 - Channel Lock Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch(&mut self) -> LOCK_CH_W<16> {
        LOCK_CH_W::new(self)
    }
    #[doc = "Bit 17 - Bus Lock Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock_b(&mut self) -> LOCK_B_W<17> {
        LOCK_B_W::new(self)
    }
    #[doc = "Bit 18 - Destination Handshaking Interface Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dst_hs_pol(&mut self) -> DST_HS_POL_W<18> {
        DST_HS_POL_W::new(self)
    }
    #[doc = "Bit 19 - Source Handshaking Interface Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn src_hs_pol(&mut self) -> SRC_HS_POL_W<19> {
        SRC_HS_POL_W::new(self)
    }
    #[doc = "Bits 20:29 - Maximum AMBA Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn max_abrst(&mut self) -> MAX_ABRST_W<20> {
        MAX_ABRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgl](index.html) module"]
pub struct CFGL_SPEC;
impl crate::RegisterSpec for CFGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgl::R](R) reader structure"]
impl crate::Readable for CFGL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgl::W](W) writer structure"]
impl crate::Writable for CFGL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGL to value 0x0e00"]
impl crate::Resettable for CFGL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e00;
}
