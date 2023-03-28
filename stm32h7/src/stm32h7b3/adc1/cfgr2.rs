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
#[doc = "Field `ROVSE` reader - ADC oversampler enable on scope ADC group regular"]
pub type ROVSE_R = crate::BitReader<ROVSE_A>;
#[doc = "ADC oversampler enable on scope ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE_A {
    #[doc = "0: Regular oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Regular oversampling enabled"]
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
#[doc = "Field `ROVSE` writer - ADC oversampler enable on scope ADC group regular"]
pub type ROVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSE_A, O>;
impl<'a, const O: u8> ROVSE_W<'a, O> {
    #[doc = "Regular oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Disabled)
    }
    #[doc = "Regular oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Enabled)
    }
}
#[doc = "Field `JOVSE` reader - ADC oversampler enable on scope ADC group injected"]
pub type JOVSE_R = crate::BitReader<JOVSE_A>;
#[doc = "ADC oversampler enable on scope ADC group injected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE_A {
    #[doc = "0: Injected oversampling disabled"]
    Disabled = 0,
    #[doc = "1: Injected oversampling enabled"]
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
#[doc = "Field `JOVSE` writer - ADC oversampler enable on scope ADC group injected"]
pub type JOVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, JOVSE_A, O>;
impl<'a, const O: u8> JOVSE_W<'a, O> {
    #[doc = "Injected oversampling disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Disabled)
    }
    #[doc = "Injected oversampling enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Enabled)
    }
}
#[doc = "Field `OVSS` reader - ADC oversampling shift"]
pub type OVSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVSS` writer - ADC oversampling shift"]
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TROVS` reader - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TROVS_R = crate::BitReader<TROVS_A>;
#[doc = "ADC oversampling discontinuous mode (triggered mode) for ADC group regular\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS_A {
    #[doc = "0: All oversampled conversions for a channel are run following a trigger"]
    Automatic = 0,
    #[doc = "1: Each oversampled conversion for a channel needs a new trigger"]
    Triggered = 1,
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
            false => TROVS_A::Automatic,
            true => TROVS_A::Triggered,
        }
    }
    #[doc = "Checks if the value of the field is `Automatic`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS_A::Automatic
    }
    #[doc = "Checks if the value of the field is `Triggered`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS_A::Triggered
    }
}
#[doc = "Field `TROVS` writer - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TROVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, TROVS_A, O>;
impl<'a, const O: u8> TROVS_W<'a, O> {
    #[doc = "All oversampled conversions for a channel are run following a trigger"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(TROVS_A::Automatic)
    }
    #[doc = "Each oversampled conversion for a channel needs a new trigger"]
    #[inline(always)]
    pub fn triggered(self) -> &'a mut W {
        self.variant(TROVS_A::Triggered)
    }
}
#[doc = "Field `ROVSM` reader - Regular Oversampling mode"]
pub type ROVSM_R = crate::BitReader<ROVSM_A>;
#[doc = "Regular Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM_A {
    #[doc = "0: Oversampling is temporary stopped and continued after injection sequence"]
    Continued = 0,
    #[doc = "1: Oversampling is aborted and resumed from start after injection sequence"]
    Resumed = 1,
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
            false => ROVSM_A::Continued,
            true => ROVSM_A::Resumed,
        }
    }
    #[doc = "Checks if the value of the field is `Continued`"]
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM_A::Continued
    }
    #[doc = "Checks if the value of the field is `Resumed`"]
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM_A::Resumed
    }
}
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub type ROVSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSM_A, O>;
impl<'a, const O: u8> ROVSM_W<'a, O> {
    #[doc = "Oversampling is temporary stopped and continued after injection sequence"]
    #[inline(always)]
    pub fn continued(self) -> &'a mut W {
        self.variant(ROVSM_A::Continued)
    }
    #[doc = "Oversampling is aborted and resumed from start after injection sequence"]
    #[inline(always)]
    pub fn resumed(self) -> &'a mut W {
        self.variant(ROVSM_A::Resumed)
    }
}
#[doc = "Field `RSHIFT1` reader - Right-shift data after Offset 1 correction"]
pub type RSHIFT1_R = crate::BitReader<RSHIFT1_A>;
#[doc = "Right-shift data after Offset 1 correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSHIFT1_A {
    #[doc = "0: Right-shifting disabled"]
    Disabled = 0,
    #[doc = "1: Data is right-shifted 1-bit"]
    Enabled = 1,
}
impl From<RSHIFT1_A> for bool {
    #[inline(always)]
    fn from(variant: RSHIFT1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSHIFT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSHIFT1_A {
        match self.bits {
            false => RSHIFT1_A::Disabled,
            true => RSHIFT1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSHIFT1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSHIFT1_A::Enabled
    }
}
#[doc = "Field `RSHIFT1` writer - Right-shift data after Offset 1 correction"]
pub type RSHIFT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, RSHIFT1_A, O>;
impl<'a, const O: u8> RSHIFT1_W<'a, O> {
    #[doc = "Right-shifting disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSHIFT1_A::Disabled)
    }
    #[doc = "Data is right-shifted 1-bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSHIFT1_A::Enabled)
    }
}
#[doc = "Field `RSHIFT2` reader - Right-shift data after Offset 2 correction"]
pub use RSHIFT1_R as RSHIFT2_R;
#[doc = "Field `RSHIFT3` reader - Right-shift data after Offset 3 correction"]
pub use RSHIFT1_R as RSHIFT3_R;
#[doc = "Field `RSHIFT4` reader - Right-shift data after Offset 4 correction"]
pub use RSHIFT1_R as RSHIFT4_R;
#[doc = "Field `RSHIFT2` writer - Right-shift data after Offset 2 correction"]
pub use RSHIFT1_W as RSHIFT2_W;
#[doc = "Field `RSHIFT3` writer - Right-shift data after Offset 3 correction"]
pub use RSHIFT1_W as RSHIFT3_W;
#[doc = "Field `RSHIFT4` writer - Right-shift data after Offset 4 correction"]
pub use RSHIFT1_W as RSHIFT4_W;
#[doc = "Field `OSVR` reader - Oversampling ratio"]
pub type OSVR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OSVR` writer - Oversampling ratio"]
pub type OSVR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u16, u16, 10, O>;
#[doc = "Field `LSHIFT` reader - Left shift factor"]
pub type LSHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSHIFT` writer - Left shift factor"]
pub type LSHIFT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    pub fn rshift1(&self) -> RSHIFT1_R {
        RSHIFT1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    pub fn rshift2(&self) -> RSHIFT2_R {
        RSHIFT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    pub fn rshift3(&self) -> RSHIFT3_R {
        RSHIFT3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    pub fn rshift4(&self) -> RSHIFT4_R {
        RSHIFT4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    pub fn osvr(&self) -> OSVR_R {
        OSVR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<0> {
        ROVSE_W::new(self)
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<1> {
        JOVSE_W::new(self)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
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
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift1(&mut self) -> RSHIFT1_W<11> {
        RSHIFT1_W::new(self)
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift2(&mut self) -> RSHIFT2_W<12> {
        RSHIFT2_W::new(self)
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift3(&mut self) -> RSHIFT3_W<13> {
        RSHIFT3_W::new(self)
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    #[must_use]
    pub fn rshift4(&mut self) -> RSHIFT4_W<14> {
        RSHIFT4_W::new(self)
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn osvr(&mut self) -> OSVR_W<16> {
        OSVR_W::new(self)
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    #[must_use]
    pub fn lshift(&mut self) -> LSHIFT_W<28> {
        LSHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
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
