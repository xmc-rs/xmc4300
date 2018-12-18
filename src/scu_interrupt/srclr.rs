#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRCLR {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = "Values that can be written to the field `PRWARN`"]
pub enum PRWARNW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl PRWARNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRWARNW::CONST_0 => false,
            PRWARNW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _PRWARNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRWARNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PRWARNW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PRWARNW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PI`"]
pub enum PIW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl PIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIW::CONST_0 => false,
            PIW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIW<'a> {
    w: &'a mut W,
}
impl<'a> _PIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(PIW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(PIW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AI`"]
pub enum AIW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl AIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIW::CONST_0 => false,
            AIW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIW<'a> {
    w: &'a mut W,
}
impl<'a> _AIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(AIW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(AIW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DLROVR`"]
pub enum DLROVRW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl DLROVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLROVRW::CONST_0 => false,
            DLROVRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLROVRW<'a> {
    w: &'a mut W,
}
impl<'a> _DLROVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLROVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(DLROVRW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(DLROVRW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HDCLR`"]
pub enum HDCLRW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl HDCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDCLRW::CONST_0 => false,
            HDCLRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _HDCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDCLRW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDCLRW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HDSET`"]
pub enum HDSETW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl HDSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDSETW::CONST_0 => false,
            HDSETW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDSETW<'a> {
    w: &'a mut W,
}
impl<'a> _HDSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDSETW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDSETW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HDCR`"]
pub enum HDCRW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl HDCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDCRW::CONST_0 => false,
            HDCRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDCRW<'a> {
    w: &'a mut W,
}
impl<'a> _HDCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(HDCRW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(HDCRW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCSICTRL`"]
pub enum OSCSICTRLW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl OSCSICTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCSICTRLW::CONST_0 => false,
            OSCSICTRLW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSICTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSICTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSICTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCSICTRLW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCSICTRLW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCULCTRL`"]
pub enum OSCULCTRLW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl OSCULCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCULCTRLW::CONST_0 => false,
            OSCULCTRLW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCULCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCULCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCULCTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(OSCULCTRLW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(OSCULCTRLW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_CTR`"]
pub enum RTC_CTRW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl RTC_CTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_CTRW::CONST_0 => false,
            RTC_CTRW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_CTRW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_CTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_CTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_CTRW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_CTRW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_ATIM0`"]
pub enum RTC_ATIM0W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl RTC_ATIM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ATIM0W::CONST_0 => false,
            RTC_ATIM0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ATIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ATIM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ATIM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_ATIM0W::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_ATIM0W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_ATIM1`"]
pub enum RTC_ATIM1W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl RTC_ATIM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_ATIM1W::CONST_0 => false,
            RTC_ATIM1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_ATIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_ATIM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_ATIM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_ATIM1W::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_ATIM1W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_TIM0`"]
pub enum RTC_TIM0W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl RTC_TIM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_TIM0W::CONST_0 => false,
            RTC_TIM0W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_TIM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_TIM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_TIM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_TIM0W::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_TIM0W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTC_TIM1`"]
pub enum RTC_TIM1W {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl RTC_TIM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTC_TIM1W::CONST_0 => false,
            RTC_TIM1W::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTC_TIM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_TIM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_TIM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RTC_TIM1W::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RTC_TIM1W::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMX`"]
pub enum RMXW {
    #[doc = "No effect"]
    CONST_0,
    #[doc = "Clear the status bit"]
    CONST_1,
}
impl RMXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMXW::CONST_0 => false,
            RMXW::CONST_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMXW<'a> {
    w: &'a mut W,
}
impl<'a> _RMXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn const_0(self) -> &'a mut W {
        self.variant(RMXW::CONST_0)
    }
    #[doc = "Clear the status bit"]
    #[inline]
    pub fn const_1(self) -> &'a mut W {
        self.variant(RMXW::CONST_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - WDT pre-warning Interrupt Clear"]
    #[inline]
    pub fn prwarn(&mut self) -> _PRWARNW {
        _PRWARNW { w: self }
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Clear"]
    #[inline]
    pub fn pi(&mut self) -> _PIW {
        _PIW { w: self }
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Clear"]
    #[inline]
    pub fn ai(&mut self) -> _AIW {
        _AIW { w: self }
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt clear"]
    #[inline]
    pub fn dlrovr(&mut self) -> _DLROVRW {
        _DLROVRW { w: self }
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Clear"]
    #[inline]
    pub fn hdclr(&mut self) -> _HDCLRW {
        _HDCLRW { w: self }
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Clear"]
    #[inline]
    pub fn hdset(&mut self) -> _HDSETW {
        _HDSETW { w: self }
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Clear"]
    #[inline]
    pub fn hdcr(&mut self) -> _HDCRW {
        _HDCRW { w: self }
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Clear"]
    #[inline]
    pub fn oscsictrl(&mut self) -> _OSCSICTRLW {
        _OSCSICTRLW { w: self }
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Clear"]
    #[inline]
    pub fn osculctrl(&mut self) -> _OSCULCTRLW {
        _OSCULCTRLW { w: self }
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_ctr(&mut self) -> _RTC_CTRW {
        _RTC_CTRW { w: self }
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_atim0(&mut self) -> _RTC_ATIM0W {
        _RTC_ATIM0W { w: self }
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_atim1(&mut self) -> _RTC_ATIM1W {
        _RTC_ATIM1W { w: self }
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_tim0(&mut self) -> _RTC_TIM0W {
        _RTC_TIM0W { w: self }
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Clear"]
    #[inline]
    pub fn rtc_tim1(&mut self) -> _RTC_TIM1W {
        _RTC_TIM1W { w: self }
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Clear"]
    #[inline]
    pub fn rmx(&mut self) -> _RMXW {
        _RMXW { w: self }
    }
}
