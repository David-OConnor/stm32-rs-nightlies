#[doc = "Register `FMAC_SR` reader"]
pub struct R(crate::R<FMAC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMAC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMAC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMAC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `YEMPTY` reader - Y buffer empty flag The buffer is flagged as empty if the number of unread data is less than the EMPTY_WM threshold. The number of unread data is the difference between the read pointer and the current output destination address. This flag is set and cleared by hardware, or by a reset. Note: after the last sample is read from the Y buffer there is a delay of 3 clock cycles before the YEMPTY flag goes high. To avoid any risk of underflow it is recommended to insert a software delay after reading from the Y buffer before reading the FMAC_SR. Alternatively, an EMPTY_WM threshold of 2 can be used."]
pub type YEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `X1FULL` reader - X1 buffer full flag The buffer is flagged as full if the number of available spaces is less than the FULL_WM threshold. The number of available spaces is the difference between the write pointer and the least recent sample currently in use. This flag is set and cleared by hardware, or by a reset. Note: after the last available space in the X1 buffer is filled there is a delay of 3 clock cycles before the X1FULL flag goes high. To avoid any risk of overflow it is recommended to insert a software delay after writing to the X1 buffer before reading the FMAC_SR. Alternatively, a FULL_WM threshold of 2 can be used."]
pub type X1FULL_R = crate::BitReader<bool>;
#[doc = "Field `OVFL` reader - Overflow error flag An overflow occurs when a write is made to FMAC_WDATA when no free space is available in the X1 buffer. This flag is cleared by a reset of the unit."]
pub type OVFL_R = crate::BitReader<bool>;
#[doc = "Field `UNFL` reader - Underflow error flag An underflow occurs when a read is made from FMAC_RDATA when no valid data is available in the Y buffer. This flag is cleared by a reset of the unit."]
pub type UNFL_R = crate::BitReader<bool>;
#[doc = "Field `SAT` reader - Saturation error flag Saturation occurs when the result of an accumulation exceeds the numeric range of the accumulator. This flag is cleared by a reset of the unit."]
pub type SAT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Y buffer empty flag The buffer is flagged as empty if the number of unread data is less than the EMPTY_WM threshold. The number of unread data is the difference between the read pointer and the current output destination address. This flag is set and cleared by hardware, or by a reset. Note: after the last sample is read from the Y buffer there is a delay of 3 clock cycles before the YEMPTY flag goes high. To avoid any risk of underflow it is recommended to insert a software delay after reading from the Y buffer before reading the FMAC_SR. Alternatively, an EMPTY_WM threshold of 2 can be used."]
    #[inline(always)]
    pub fn yempty(&self) -> YEMPTY_R {
        YEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - X1 buffer full flag The buffer is flagged as full if the number of available spaces is less than the FULL_WM threshold. The number of available spaces is the difference between the write pointer and the least recent sample currently in use. This flag is set and cleared by hardware, or by a reset. Note: after the last available space in the X1 buffer is filled there is a delay of 3 clock cycles before the X1FULL flag goes high. To avoid any risk of overflow it is recommended to insert a software delay after writing to the X1 buffer before reading the FMAC_SR. Alternatively, a FULL_WM threshold of 2 can be used."]
    #[inline(always)]
    pub fn x1full(&self) -> X1FULL_R {
        X1FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Overflow error flag An overflow occurs when a write is made to FMAC_WDATA when no free space is available in the X1 buffer. This flag is cleared by a reset of the unit."]
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Underflow error flag An underflow occurs when a read is made from FMAC_RDATA when no valid data is available in the Y buffer. This flag is cleared by a reset of the unit."]
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Saturation error flag Saturation occurs when the result of an accumulation exceeds the numeric range of the accumulator. This flag is cleared by a reset of the unit."]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "FMAC status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmac_sr](index.html) module"]
pub struct FMAC_SR_SPEC;
impl crate::RegisterSpec for FMAC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmac_sr::R](R) reader structure"]
impl crate::Readable for FMAC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMAC_SR to value 0x01"]
impl crate::Resettable for FMAC_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
