#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROVSE` reader - DMAEN"]
pub type ROVSE_R = crate::BitReader<ROVSE_A>;
#[doc = "DMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE_A {
    #[doc = "0: Regular Oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Regular Oversampling enabled"]
    Enabled = 1,
}
impl From<ROVSE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSE_A {
        match self.bits {
            false => ROVSE_A::Disabled,
            true => ROVSE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE_A::Enabled
    }
}
#[doc = "Field `ROVSE` writer - DMAEN"]
pub type ROVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSE_A, O>;
impl<'a, const O: u8> ROVSE_W<'a, O> {
    #[doc = "Regular Oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Disabled)
    }
    #[doc = "Regular Oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Enabled)
    }
}
#[doc = "Field `JOVSE` reader - DMACFG"]
pub type JOVSE_R = crate::BitReader<JOVSE_A>;
#[doc = "DMACFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE_A {
    #[doc = "0: Injected Oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Injected Oversampling enabled"]
    Enabled = 1,
}
impl From<JOVSE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl JOVSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVSE_A {
        match self.bits {
            false => JOVSE_A::Disabled,
            true => JOVSE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE_A::Enabled
    }
}
#[doc = "Field `JOVSE` writer - DMACFG"]
pub type JOVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, JOVSE_A, O>;
impl<'a, const O: u8> JOVSE_W<'a, O> {
    #[doc = "Injected Oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Disabled)
    }
    #[doc = "Injected Oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Enabled)
    }
}
#[doc = "Field `OVSR` reader - RES"]
pub type OVSR_R = crate::FieldReader<u8, OVSR_A>;
#[doc = "RES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR_A {
    #[doc = "0: 2x"]
    Ratio2 = 0,
    #[doc = "1: 4x"]
    Ratio4 = 1,
    #[doc = "2: 8x"]
    Ratio8 = 2,
    #[doc = "3: 16x"]
    Ratio16 = 3,
    #[doc = "4: 32x"]
    Ratio32 = 4,
    #[doc = "5: 64x"]
    Ratio64 = 5,
    #[doc = "6: 128x"]
    Ratio128 = 6,
    #[doc = "7: 256x"]
    Ratio256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
impl OVSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::Ratio2,
            1 => OVSR_A::Ratio4,
            2 => OVSR_A::Ratio8,
            3 => OVSR_A::Ratio16,
            4 => OVSR_A::Ratio32,
            5 => OVSR_A::Ratio64,
            6 => OVSR_A::Ratio128,
            7 => OVSR_A::Ratio256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ratio2`"]
    #[inline(always)]
    pub fn is_ratio2(&self) -> bool {
        *self == OVSR_A::Ratio2
    }
    #[doc = "Checks if the value of the field is `Ratio4`"]
    #[inline(always)]
    pub fn is_ratio4(&self) -> bool {
        *self == OVSR_A::Ratio4
    }
    #[doc = "Checks if the value of the field is `Ratio8`"]
    #[inline(always)]
    pub fn is_ratio8(&self) -> bool {
        *self == OVSR_A::Ratio8
    }
    #[doc = "Checks if the value of the field is `Ratio16`"]
    #[inline(always)]
    pub fn is_ratio16(&self) -> bool {
        *self == OVSR_A::Ratio16
    }
    #[doc = "Checks if the value of the field is `Ratio32`"]
    #[inline(always)]
    pub fn is_ratio32(&self) -> bool {
        *self == OVSR_A::Ratio32
    }
    #[doc = "Checks if the value of the field is `Ratio64`"]
    #[inline(always)]
    pub fn is_ratio64(&self) -> bool {
        *self == OVSR_A::Ratio64
    }
    #[doc = "Checks if the value of the field is `Ratio128`"]
    #[inline(always)]
    pub fn is_ratio128(&self) -> bool {
        *self == OVSR_A::Ratio128
    }
    #[doc = "Checks if the value of the field is `Ratio256`"]
    #[inline(always)]
    pub fn is_ratio256(&self) -> bool {
        *self == OVSR_A::Ratio256
    }
}
#[doc = "Field `OVSR` writer - RES"]
pub type OVSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, OVSR_A, 3, O>;
impl<'a, const O: u8> OVSR_W<'a, O> {
    #[doc = "2x"]
    #[inline(always)]
    pub fn ratio2(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio2)
    }
    #[doc = "4x"]
    #[inline(always)]
    pub fn ratio4(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio4)
    }
    #[doc = "8x"]
    #[inline(always)]
    pub fn ratio8(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio8)
    }
    #[doc = "16x"]
    #[inline(always)]
    pub fn ratio16(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio16)
    }
    #[doc = "32x"]
    #[inline(always)]
    pub fn ratio32(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio32)
    }
    #[doc = "64x"]
    #[inline(always)]
    pub fn ratio64(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio64)
    }
    #[doc = "128x"]
    #[inline(always)]
    pub fn ratio128(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio128)
    }
    #[doc = "256x"]
    #[inline(always)]
    pub fn ratio256(self) -> &'a mut W {
        self.variant(OVSR_A::Ratio256)
    }
}
#[doc = "Field `OVSS` reader - ALIGN"]
pub type OVSS_R = crate::FieldReader<u8, OVSS_A>;
#[doc = "ALIGN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS_A {
    #[doc = "0: No Shift"]
    NoShift = 0,
    #[doc = "1: Shift 1-bit"]
    Shift1bit = 1,
    #[doc = "2: Shift 2-bit"]
    Shift2bit = 2,
    #[doc = "3: Shift 3-bit"]
    Shift3bit = 3,
    #[doc = "4: Shift 4-bit"]
    Shift4bit = 4,
    #[doc = "5: Shift 5-bit"]
    Shift5bit = 5,
    #[doc = "6: Shift 6-bit"]
    Shift6bit = 6,
    #[doc = "7: Shift 7-bit"]
    Shift7bit = 7,
    #[doc = "8: Shift 8-bit"]
    Shift8bit = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
impl OVSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVSS_A> {
        match self.bits {
            0 => Some(OVSS_A::NoShift),
            1 => Some(OVSS_A::Shift1bit),
            2 => Some(OVSS_A::Shift2bit),
            3 => Some(OVSS_A::Shift3bit),
            4 => Some(OVSS_A::Shift4bit),
            5 => Some(OVSS_A::Shift5bit),
            6 => Some(OVSS_A::Shift6bit),
            7 => Some(OVSS_A::Shift7bit),
            8 => Some(OVSS_A::Shift8bit),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoShift`"]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS_A::NoShift
    }
    #[doc = "Checks if the value of the field is `Shift1bit`"]
    #[inline(always)]
    pub fn is_shift1bit(&self) -> bool {
        *self == OVSS_A::Shift1bit
    }
    #[doc = "Checks if the value of the field is `Shift2bit`"]
    #[inline(always)]
    pub fn is_shift2bit(&self) -> bool {
        *self == OVSS_A::Shift2bit
    }
    #[doc = "Checks if the value of the field is `Shift3bit`"]
    #[inline(always)]
    pub fn is_shift3bit(&self) -> bool {
        *self == OVSS_A::Shift3bit
    }
    #[doc = "Checks if the value of the field is `Shift4bit`"]
    #[inline(always)]
    pub fn is_shift4bit(&self) -> bool {
        *self == OVSS_A::Shift4bit
    }
    #[doc = "Checks if the value of the field is `Shift5bit`"]
    #[inline(always)]
    pub fn is_shift5bit(&self) -> bool {
        *self == OVSS_A::Shift5bit
    }
    #[doc = "Checks if the value of the field is `Shift6bit`"]
    #[inline(always)]
    pub fn is_shift6bit(&self) -> bool {
        *self == OVSS_A::Shift6bit
    }
    #[doc = "Checks if the value of the field is `Shift7bit`"]
    #[inline(always)]
    pub fn is_shift7bit(&self) -> bool {
        *self == OVSS_A::Shift7bit
    }
    #[doc = "Checks if the value of the field is `Shift8bit`"]
    #[inline(always)]
    pub fn is_shift8bit(&self) -> bool {
        *self == OVSS_A::Shift8bit
    }
}
#[doc = "Field `OVSS` writer - ALIGN"]
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, OVSS_A, 4, O>;
impl<'a, const O: u8> OVSS_W<'a, O> {
    #[doc = "No Shift"]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut W {
        self.variant(OVSS_A::NoShift)
    }
    #[doc = "Shift 1-bit"]
    #[inline(always)]
    pub fn shift1bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift1bit)
    }
    #[doc = "Shift 2-bit"]
    #[inline(always)]
    pub fn shift2bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift2bit)
    }
    #[doc = "Shift 3-bit"]
    #[inline(always)]
    pub fn shift3bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift3bit)
    }
    #[doc = "Shift 4-bit"]
    #[inline(always)]
    pub fn shift4bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift4bit)
    }
    #[doc = "Shift 5-bit"]
    #[inline(always)]
    pub fn shift5bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift5bit)
    }
    #[doc = "Shift 6-bit"]
    #[inline(always)]
    pub fn shift6bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift6bit)
    }
    #[doc = "Shift 7-bit"]
    #[inline(always)]
    pub fn shift7bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift7bit)
    }
    #[doc = "Shift 8-bit"]
    #[inline(always)]
    pub fn shift8bit(self) -> &'a mut W {
        self.variant(OVSS_A::Shift8bit)
    }
}
#[doc = "Field `TROVS` reader - Triggered Regular Oversampling"]
pub type TROVS_R = crate::BitReader<TROVS_A>;
#[doc = "Triggered Regular Oversampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS_A {
    #[doc = "0: All oversampled conversions for a channel are done consecutively following a trigger"]
    All = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    Single = 1,
}
impl From<TROVS_A> for bool {
    #[inline(always)]
    fn from(variant: TROVS_A) -> Self {
        variant as u8 != 0
    }
}
impl TROVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TROVS_A {
        match self.bits {
            false => TROVS_A::All,
            true => TROVS_A::Single,
        }
    }
    #[doc = "Checks if the value of the field is `All`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == TROVS_A::All
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TROVS_A::Single
    }
}
#[doc = "Field `TROVS` writer - Triggered Regular Oversampling"]
pub type TROVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, TROVS_A, O>;
impl<'a, const O: u8> TROVS_W<'a, O> {
    #[doc = "All oversampled conversions for a channel are done consecutively following a trigger"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(TROVS_A::All)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TROVS_A::Single)
    }
}
#[doc = "Field `ROVSM` reader - Regular Oversampling mode"]
pub type ROVSM_R = crate::BitReader<ROVSM_A>;
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM_A {
    #[doc = "0: When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    ContinuedMode = 0,
    #[doc = "1: When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    ResumedMode = 1,
}
impl From<ROVSM_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSM_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVSM_A {
        match self.bits {
            false => ROVSM_A::ContinuedMode,
            true => ROVSM_A::ResumedMode,
        }
    }
    #[doc = "Checks if the value of the field is `ContinuedMode`"]
    #[inline(always)]
    pub fn is_continued_mode(&self) -> bool {
        *self == ROVSM_A::ContinuedMode
    }
    #[doc = "Checks if the value of the field is `ResumedMode`"]
    #[inline(always)]
    pub fn is_resumed_mode(&self) -> bool {
        *self == ROVSM_A::ResumedMode
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub type ROVSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSM_A, O>;
impl<'a, const O: u8> ROVSM_W<'a, O> {
    #[doc = "When injected conversions are triggered, the oversampling is temporary stopped and continued after the injection sequence (oversampling buffer is maintained during injected sequence)"]
    #[inline(always)]
    pub fn continued_mode(self) -> &'a mut W {
        self.variant(ROVSM_A::ContinuedMode)
    }
    #[doc = "When injected conversions are triggered, the current oversampling is aborted and resumed from start after the injection sequence (oversampling buffer is zeroed by injected sequence start)"]
    #[inline(always)]
    pub fn resumed_mode(self) -> &'a mut W {
        self.variant(ROVSM_A::ResumedMode)
    }
}
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<0> {
        ROVSE_W::new(self)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<1> {
        JOVSE_W::new(self)
    }
    #[doc = "Bits 2:4 - RES"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<2> {
        OVSR_W::new(self)
    }
    #[doc = "Bits 5:8 - ALIGN"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    #[doc = "Bit 9 - Triggered Regular Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<9> {
        TROVS_W::new(self)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<10> {
        ROVSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
