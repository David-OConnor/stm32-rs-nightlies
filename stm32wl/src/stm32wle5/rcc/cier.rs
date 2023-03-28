#[doc = "Register `CIER` reader"]
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIER` writer"]
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
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
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, LSIRDYIE_A, O>;
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
pub type LSERDYIE_R = crate::BitReader<LSERDYIE_A>;
#[doc = "LSE ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<LSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYIE_A {
        match self.bits {
            false => LSERDYIE_A::Disabled,
            true => LSERDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSERDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSERDYIE_A::Enabled
    }
}
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, LSERDYIE_A, O>;
impl<'a, const O: u8> LSERDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSERDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSERDYIE_A::Enabled)
    }
}
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub type MSIRDYIE_R = crate::BitReader<MSIRDYIE_A>;
#[doc = "MSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<MSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRDYIE_A {
        match self.bits {
            false => MSIRDYIE_A::Disabled,
            true => MSIRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSIRDYIE_A::Enabled
    }
}
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub type MSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, MSIRDYIE_A, O>;
impl<'a, const O: u8> MSIRDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSIRDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSIRDYIE_A::Enabled)
    }
}
#[doc = "Field `HSIRDYIE` reader - HSI16 ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader<HSIRDYIE_A>;
#[doc = "HSI16 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<HSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYIE_A {
        match self.bits {
            false => HSIRDYIE_A::Disabled,
            true => HSIRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIRDYIE_A::Enabled
    }
}
#[doc = "Field `HSIRDYIE` writer - HSI16 ready interrupt enable"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, HSIRDYIE_A, O>;
impl<'a, const O: u8> HSIRDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::Enabled)
    }
}
#[doc = "Field `HSERDYIE` reader - HSE32 ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader<HSERDYIE_A>;
#[doc = "HSE32 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<HSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDYIE_A {
        match self.bits {
            false => HSERDYIE_A::Disabled,
            true => HSERDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSERDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSERDYIE_A::Enabled
    }
}
#[doc = "Field `HSERDYIE` writer - HSE32 ready interrupt enable"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, HSERDYIE_A, O>;
impl<'a, const O: u8> HSERDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSERDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSERDYIE_A::Enabled)
    }
}
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub type PLLRDYIE_R = crate::BitReader<PLLRDYIE_A>;
#[doc = "PLL ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<PLLRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYIE_A {
        match self.bits {
            false => PLLRDYIE_A::Disabled,
            true => PLLRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLRDYIE_A::Enabled
    }
}
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, PLLRDYIE_A, O>;
impl<'a, const O: u8> PLLRDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::Enabled)
    }
}
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
pub type LSECSSIE_R = crate::BitReader<LSECSSIE_A>;
#[doc = "LSE clock security system interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
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
pub type LSECSSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, LSECSSIE_A, O>;
impl<'a, const O: u8> LSECSSIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSECSSIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSECSSIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE32 ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<2> {
        MSIRDYIE_W::new(self)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<3> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 4 - HSE32 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<4> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<5> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<9> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](index.html) module"]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cier::R](R) reader structure"]
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cier::W](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
