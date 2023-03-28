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
    #[doc = "0: LSI ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LSI ready interrupt enabled"]
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
    #[doc = "LSI ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Disabled)
    }
    #[doc = "LSI ready interrupt enabled"]
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
    #[doc = "0: LSE ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: LSE ready interrupt enabled"]
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
    #[doc = "LSE ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSERDYIE_A::Disabled)
    }
    #[doc = "LSE ready interrupt enabled"]
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
    #[doc = "0: MSI ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: MSI ready interrupt enabled"]
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
    #[doc = "MSI ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSIRDYIE_A::Disabled)
    }
    #[doc = "MSI ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSIRDYIE_A::Enabled)
    }
}
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader<HSIRDYIE_A>;
#[doc = "HSI ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYIE_A {
    #[doc = "0: HSI16 ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HSI16 ready interrupt enabled"]
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
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, HSIRDYIE_A, O>;
impl<'a, const O: u8> HSIRDYIE_W<'a, O> {
    #[doc = "HSI16 ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::Disabled)
    }
    #[doc = "HSI16 ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::Enabled)
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader<HSERDYIE_A>;
#[doc = "HSE ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYIE_A {
    #[doc = "0: HSE ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HSE ready interrupt enabled"]
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
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, HSERDYIE_A, O>;
impl<'a, const O: u8> HSERDYIE_W<'a, O> {
    #[doc = "HSE ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSERDYIE_A::Disabled)
    }
    #[doc = "HSE ready interrupt enabled"]
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
    #[doc = "0: PLL lock interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PLL lock interrupt enabled"]
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
    #[doc = "PLL lock interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::Disabled)
    }
    #[doc = "PLL lock interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLRDYIE_A::Enabled)
    }
}
#[doc = "Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_R = crate::BitReader<PLLSAI1RDYIE_A>;
#[doc = "PLLSAI1 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYIE_A {
    #[doc = "0: PLLSAI1 lock interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PLLSAI1 lock interrupt enabled"]
    Enabled = 1,
}
impl From<PLLSAI1RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1RDYIE_A {
        match self.bits {
            false => PLLSAI1RDYIE_A::Disabled,
            true => PLLSAI1RDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1RDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1RDYIE_A::Enabled
    }
}
#[doc = "Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, PLLSAI1RDYIE_A, O>;
impl<'a, const O: u8> PLLSAI1RDYIE_W<'a, O> {
    #[doc = "PLLSAI1 lock interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI1RDYIE_A::Disabled)
    }
    #[doc = "PLLSAI1 lock interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI1RDYIE_A::Enabled)
    }
}
#[doc = "Field `PLLSAI2RDYIE` reader - PLLSAI2 ready interrupt enable"]
pub type PLLSAI2RDYIE_R = crate::BitReader<PLLSAI2RDYIE_A>;
#[doc = "PLLSAI2 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYIE_A {
    #[doc = "0: PLLSAI2 lock interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PLLSAI2 lock interrupt enabled"]
    Enabled = 1,
}
impl From<PLLSAI2RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2RDYIE_A {
        match self.bits {
            false => PLLSAI2RDYIE_A::Disabled,
            true => PLLSAI2RDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2RDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2RDYIE_A::Enabled
    }
}
#[doc = "Field `PLLSAI2RDYIE` writer - PLLSAI2 ready interrupt enable"]
pub type PLLSAI2RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, PLLSAI2RDYIE_A, O>;
impl<'a, const O: u8> PLLSAI2RDYIE_W<'a, O> {
    #[doc = "PLLSAI2 lock interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2RDYIE_A::Disabled)
    }
    #[doc = "PLLSAI2 lock interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2RDYIE_A::Enabled)
    }
}
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
pub type LSECSSIE_R = crate::BitReader<LSECSSIE_A>;
#[doc = "LSE clock security system interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSIE_A {
    #[doc = "0: Clock security interrupt caused by LSE clock failure disabled"]
    Disabled = 0,
    #[doc = "1: Clock security interrupt caused by LSE clock failure enabled"]
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
    #[doc = "Clock security interrupt caused by LSE clock failure disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSECSSIE_A::Disabled)
    }
    #[doc = "Clock security interrupt caused by LSE clock failure enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSECSSIE_A::Enabled)
    }
}
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_R = crate::BitReader<HSI48RDYIE_A>;
#[doc = "HSI48 ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYIE_A {
    #[doc = "0: HSI48 ready interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HSI48 ready interrupt enabled"]
    Enabled = 1,
}
impl From<HSI48RDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI48RDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSI48RDYIE_A {
        match self.bits {
            false => HSI48RDYIE_A::Disabled,
            true => HSI48RDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI48RDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI48RDYIE_A::Enabled
    }
}
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, HSI48RDYIE_A, O>;
impl<'a, const O: u8> HSI48RDYIE_W<'a, O> {
    #[doc = "HSI48 ready interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI48RDYIE_A::Disabled)
    }
    #[doc = "HSI48 ready interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI48RDYIE_A::Enabled)
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
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&self) -> PLLSAI2RDYIE_R {
        PLLSAI2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<3> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
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
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W<6> {
        PLLSAI1RDYIE_W::new(self)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2rdyie(&mut self) -> PLLSAI2RDYIE_W<7> {
        PLLSAI2RDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<9> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<10> {
        HSI48RDYIE_W::new(self)
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
