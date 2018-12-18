#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GAHBCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `GlblIntrMsk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLBLINTRMSKR {
    #[doc = "Mask the interrupt assertion to the application."]
    VALUE1,
    #[doc = "Unmask the interrupt assertion to the application."]
    VALUE2,
}
impl GLBLINTRMSKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GLBLINTRMSKR::VALUE1 => false,
            GLBLINTRMSKR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GLBLINTRMSKR {
        match value {
            false => GLBLINTRMSKR::VALUE1,
            true => GLBLINTRMSKR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GLBLINTRMSKR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GLBLINTRMSKR::VALUE2
    }
}
#[doc = "Possible values of the field `HBstLen`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HBSTLENR {
    #[doc = "Single"]
    VALUE1,
    #[doc = "INCR"]
    VALUE2,
    #[doc = "INCR4"]
    VALUE3,
    #[doc = "INCR8"]
    VALUE4,
    #[doc = "INCR16"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HBSTLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HBSTLENR::VALUE1 => 0,
            HBSTLENR::VALUE2 => 1,
            HBSTLENR::VALUE3 => 3,
            HBSTLENR::VALUE4 => 5,
            HBSTLENR::VALUE5 => 7,
            HBSTLENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HBSTLENR {
        match value {
            0 => HBSTLENR::VALUE1,
            1 => HBSTLENR::VALUE2,
            3 => HBSTLENR::VALUE3,
            5 => HBSTLENR::VALUE4,
            7 => HBSTLENR::VALUE5,
            i => HBSTLENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == HBSTLENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == HBSTLENR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == HBSTLENR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == HBSTLENR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == HBSTLENR::VALUE5
    }
}
#[doc = "Possible values of the field `DMAEn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "Core operates in Slave mode"]
    VALUE1,
    #[doc = "Core operates in a DMA mode"]
    VALUE2,
}
impl DMAENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMAENR::VALUE1 => false,
            DMAENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::VALUE1,
            true => DMAENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DMAENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DMAENR::VALUE2
    }
}
#[doc = "Possible values of the field `NPTxFEmpLvl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXFEMPLVLR {
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    VALUE1,
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    VALUE2,
}
impl NPTXFEMPLVLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            NPTXFEMPLVLR::VALUE1 => false,
            NPTXFEMPLVLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NPTXFEMPLVLR {
        match value {
            false => NPTXFEMPLVLR::VALUE1,
            true => NPTXFEMPLVLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NPTXFEMPLVLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NPTXFEMPLVLR::VALUE2
    }
}
#[doc = "Possible values of the field `PTxFEmpLvl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTXFEMPLVLR {
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    VALUE1,
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    VALUE2,
}
impl PTXFEMPLVLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PTXFEMPLVLR::VALUE1 => false,
            PTXFEMPLVLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTXFEMPLVLR {
        match value {
            false => PTXFEMPLVLR::VALUE1,
            true => PTXFEMPLVLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PTXFEMPLVLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PTXFEMPLVLR::VALUE2
    }
}
#[doc = "Possible values of the field `AHBSingle`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHBSINGLER {
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    VALUE1,
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    VALUE2,
}
impl AHBSINGLER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            AHBSINGLER::VALUE1 => false,
            AHBSINGLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHBSINGLER {
        match value {
            false => AHBSINGLER::VALUE1,
            true => AHBSINGLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == AHBSINGLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == AHBSINGLER::VALUE2
    }
}
#[doc = "Values that can be written to the field `GlblIntrMsk`"]
pub enum GLBLINTRMSKW {
    #[doc = "Mask the interrupt assertion to the application."]
    VALUE1,
    #[doc = "Unmask the interrupt assertion to the application."]
    VALUE2,
}
impl GLBLINTRMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GLBLINTRMSKW::VALUE1 => false,
            GLBLINTRMSKW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GLBLINTRMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _GLBLINTRMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GLBLINTRMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mask the interrupt assertion to the application."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GLBLINTRMSKW::VALUE1)
    }
    #[doc = "Unmask the interrupt assertion to the application."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GLBLINTRMSKW::VALUE2)
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
#[doc = "Values that can be written to the field `HBstLen`"]
pub enum HBSTLENW {
    #[doc = "Single"]
    VALUE1,
    #[doc = "INCR"]
    VALUE2,
    #[doc = "INCR4"]
    VALUE3,
    #[doc = "INCR8"]
    VALUE4,
    #[doc = "INCR16"]
    VALUE5,
}
impl HBSTLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HBSTLENW::VALUE1 => 0,
            HBSTLENW::VALUE2 => 1,
            HBSTLENW::VALUE3 => 3,
            HBSTLENW::VALUE4 => 5,
            HBSTLENW::VALUE5 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HBSTLENW<'a> {
    w: &'a mut W,
}
impl<'a> _HBSTLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HBSTLENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(HBSTLENW::VALUE1)
    }
    #[doc = "INCR"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(HBSTLENW::VALUE2)
    }
    #[doc = "INCR4"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(HBSTLENW::VALUE3)
    }
    #[doc = "INCR8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(HBSTLENW::VALUE4)
    }
    #[doc = "INCR16"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(HBSTLENW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAEn`"]
pub enum DMAENW {
    #[doc = "Core operates in Slave mode"]
    VALUE1,
    #[doc = "Core operates in a DMA mode"]
    VALUE2,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::VALUE1 => false,
            DMAENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Core operates in Slave mode"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMAENW::VALUE1)
    }
    #[doc = "Core operates in a DMA mode"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMAENW::VALUE2)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NPTxFEmpLvl`"]
pub enum NPTXFEMPLVLW {
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    VALUE1,
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    VALUE2,
}
impl NPTXFEMPLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NPTXFEMPLVLW::VALUE1 => false,
            NPTXFEMPLVLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NPTXFEMPLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _NPTXFEMPLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NPTXFEMPLVLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NPTXFEMPLVLW::VALUE1)
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NPTXFEMPLVLW::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTxFEmpLvl`"]
pub enum PTXFEMPLVLW {
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    VALUE1,
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    VALUE2,
}
impl PTXFEMPLVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTXFEMPLVLW::VALUE1 => false,
            PTXFEMPLVLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTXFEMPLVLW<'a> {
    w: &'a mut W,
}
impl<'a> _PTXFEMPLVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTXFEMPLVLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PTXFEMPLVLW::VALUE1)
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PTXFEMPLVLW::VALUE2)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHBSingle`"]
pub enum AHBSINGLEW {
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    VALUE1,
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    VALUE2,
}
impl AHBSINGLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHBSINGLEW::VALUE1 => false,
            AHBSINGLEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHBSINGLEW<'a> {
    w: &'a mut W,
}
impl<'a> _AHBSINGLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHBSINGLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHBSINGLEW::VALUE1)
    }
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHBSINGLEW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline]
    pub fn glbl_intr_msk(&self) -> GLBLINTRMSKR {
        GLBLINTRMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline]
    pub fn hbst_len(&self) -> HBSTLENR {
        HBSTLENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline]
    pub fn nptx_femp_lvl(&self) -> NPTXFEMPLVLR {
        NPTXFEMPLVLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline]
    pub fn ptx_femp_lvl(&self) -> PTXFEMPLVLR {
        PTXFEMPLVLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline]
    pub fn ahbsingle(&self) -> AHBSINGLER {
        AHBSINGLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline]
    pub fn glbl_intr_msk(&mut self) -> _GLBLINTRMSKW {
        _GLBLINTRMSKW { w: self }
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline]
    pub fn hbst_len(&mut self) -> _HBSTLENW {
        _HBSTLENW { w: self }
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline]
    pub fn nptx_femp_lvl(&mut self) -> _NPTXFEMPLVLW {
        _NPTXFEMPLVLW { w: self }
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline]
    pub fn ptx_femp_lvl(&mut self) -> _PTXFEMPLVLW {
        _PTXFEMPLVLW { w: self }
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline]
    pub fn ahbsingle(&mut self) -> _AHBSINGLEW {
        _AHBSINGLEW { w: self }
    }
}
