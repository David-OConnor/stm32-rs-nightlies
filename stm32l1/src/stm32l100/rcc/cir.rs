#[doc = "Register `CIR` reader"]
pub struct R(crate::R<CIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR` writer"]
pub struct W(crate::W<CIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_SPEC>;
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
impl From<crate::W<CIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR_A>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR_A {
    #[doc = "0: Clock is not stable"]
    NotStable = 0,
    #[doc = "1: Clock is stable"]
    Stable = 1,
}
impl From<LSIRDYFR_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYFR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYFR_A {
        match self.bits {
            false => LSIRDYFR_A::NotStable,
            true => LSIRDYFR_A::Stable,
        }
    }
    #[doc = "Checks if the value of the field is `NotStable`"]
    #[inline(always)]
    pub fn is_not_stable(&self) -> bool {
        *self == LSIRDYFR_A::NotStable
    }
    #[doc = "Checks if the value of the field is `Stable`"]
    #[inline(always)]
    pub fn is_stable(&self) -> bool {
        *self == LSIRDYFR_A::Stable
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub use LSIRDYF_R as HSIRDYF_R;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub use LSIRDYF_R as PLLRDYF_R;
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub use LSIRDYF_R as MSIRDYF_R;
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag"]
pub type LSECSSF_R = crate::BitReader<LSECSSFR_A>;
#[doc = "LSE Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSFR_A {
    #[doc = "0: No failure detected on the external 32 KHz oscillator"]
    NoFailure = 0,
    #[doc = "1: A failure is detected on the external 32 kHz oscillator"]
    Failure = 1,
}
impl From<LSECSSFR_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSFR_A {
        match self.bits {
            false => LSECSSFR_A::NoFailure,
            true => LSECSSFR_A::Failure,
        }
    }
    #[doc = "Checks if the value of the field is `NoFailure`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSFR_A::NoFailure
    }
    #[doc = "Checks if the value of the field is `Failure`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSFR_A::Failure
    }
}
#[doc = "Field `LSECSSF` writer - LSE Clock security system interrupt flag"]
pub type LSECSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSECSSFR_A, O>;
impl<'a, const O: u8> LSECSSF_W<'a, O> {
    #[doc = "No failure detected on the external 32 KHz oscillator"]
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut W {
        self.variant(LSECSSFR_A::NoFailure)
    }
    #[doc = "A failure is detected on the external 32 kHz oscillator"]
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(LSECSSFR_A::Failure)
    }
}
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<CSSFR_A>;
#[doc = "Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSFR_A {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NotInterupted = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Interupted = 1,
}
impl From<CSSFR_A> for bool {
    #[inline(always)]
    fn from(variant: CSSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSFR_A {
        match self.bits {
            false => CSSFR_A::NotInterupted,
            true => CSSFR_A::Interupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterupted`"]
    #[inline(always)]
    pub fn is_not_interupted(&self) -> bool {
        *self == CSSFR_A::NotInterupted
    }
    #[doc = "Checks if the value of the field is `Interupted`"]
    #[inline(always)]
    pub fn is_interupted(&self) -> bool {
        *self == CSSFR_A::Interupted
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE_A>;
#[doc = "LSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::Disabled,
            true => LSIRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE_A::Enabled
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSIRDYIE_A, O>;
impl<'a, const O: u8> LSIRDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Enabled)
    }
}
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub use LSIRDYIE_R as LSERDYIE_R;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub use LSIRDYIE_R as HSIRDYIE_R;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub use LSIRDYIE_R as HSERDYIE_R;
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub use LSIRDYIE_R as PLLRDYIE_R;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub use LSIRDYIE_R as MSIRDYIE_R;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub use LSIRDYIE_W as LSERDYIE_W;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub use LSIRDYIE_W as HSIRDYIE_W;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub use LSIRDYIE_W as HSERDYIE_W;
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub use LSIRDYIE_W as PLLRDYIE_W;
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub use LSIRDYIE_W as MSIRDYIE_W;
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
pub type LSECSSIE_R = crate::BitReader<LSECSSIE_A>;
#[doc = "LSE clock security system interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE_A {
    #[doc = "0: LSE CSS interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LSE CSS interrupt enabled"]
    Enabled = 1,
}
impl From<LSECSSIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSIE_A {
        match self.bits {
            false => LSECSSIE_A::Disabled,
            true => LSECSSIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSIE_A::Enabled
    }
}
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable"]
pub type LSECSSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSECSSIE_A, O>;
impl<'a, const O: u8> LSECSSIE_W<'a, O> {
    #[doc = "LSE CSS interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSECSSIE_A::Disabled)
    }
    #[doc = "LSE CSS interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSECSSIE_A::Enabled)
    }
}
#[doc = "LSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW_AW {
    #[doc = "1: Clear interrupt"]
    Clear = 1,
}
impl From<LSIRDYCW_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSIRDYCW_AW, O>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW_AW::Clear)
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub use LSIRDYC_W as LSERDYC_W;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub use LSIRDYC_W as HSIRDYC_W;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub use LSIRDYC_W as HSERDYC_W;
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub use LSIRDYC_W as PLLRDYC_W;
#[doc = "Field `MSIRDYC` writer - MSI ready interrupt clear"]
pub use LSIRDYC_W as MSIRDYC_W;
#[doc = "LSE Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSCW_AW {
    #[doc = "1: Clear interrupt"]
    Clear = 1,
}
impl From<LSECSSCW_AW> for bool {
    #[inline(always)]
    fn from(variant: LSECSSCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear"]
pub type LSECSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSECSSCW_AW, O>;
impl<'a, const O: u8> LSECSSC_W<'a, O> {
    #[doc = "Clear interrupt"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSECSSCW_AW::Clear)
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub use LSECSSC_W as CSSC_W;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssf(&mut self) -> LSECSSF_W<6> {
        LSECSSF_W::new(self)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<8> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<9> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<10> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<11> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<12> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 13 - MSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<13> {
        MSIRDYIE_W::new(self)
    }
    #[doc = "Bit 14 - LSE clock security system interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<14> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<16> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 17 - LSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<17> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<18> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<19> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 20 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<20> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 21 - MSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<21> {
        MSIRDYC_W::new(self)
    }
    #[doc = "Bit 22 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<22> {
        LSECSSC_W::new(self)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<23> {
        CSSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](index.html) module"]
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir::R](R) reader structure"]
impl crate::Readable for CIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir::W](W) writer structure"]
impl crate::Writable for CIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
