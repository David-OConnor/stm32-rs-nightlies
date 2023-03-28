#[doc = "Register `CIFR` reader"]
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<LSIRDYF_A>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::NotInterrupted,
            true => LSIRDYF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYF_A::Interrupted
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub type LSERDYF_R = crate::BitReader<LSERDYF_A>;
#[doc = "LSE ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<LSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYF_A {
        match self.bits {
            false => LSERDYF_A::NotInterrupted,
            true => LSERDYF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSERDYF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSERDYF_A::Interrupted
    }
}
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub type MSIRDYF_R = crate::BitReader<MSIRDYF_A>;
#[doc = "MSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<MSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIRDYF_A {
        match self.bits {
            false => MSIRDYF_A::NotInterrupted,
            true => MSIRDYF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == MSIRDYF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == MSIRDYF_A::Interrupted
    }
}
#[doc = "Field `HSIRDYF` reader - HSI16 ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader<HSIRDYF_A>;
#[doc = "HSI16 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<HSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYF_A {
        match self.bits {
            false => HSIRDYF_A::NotInterrupted,
            true => HSIRDYF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HSIRDYF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HSIRDYF_A::Interrupted
    }
}
#[doc = "Field `HSERDYF` reader - HSE32 ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader<HSERDYF_A>;
#[doc = "HSE32 ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<HSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDYF_A {
        match self.bits {
            false => HSERDYF_A::NotInterrupted,
            true => HSERDYF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == HSERDYF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == HSERDYF_A::Interrupted
    }
}
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub type PLLRDYF_R = crate::BitReader<PLLRDYF_A>;
#[doc = "PLL ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<PLLRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYF_A {
        match self.bits {
            false => PLLRDYF_A::NotInterrupted,
            true => PLLRDYF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == PLLRDYF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == PLLRDYF_A::Interrupted
    }
}
#[doc = "Field `CSSF` reader - HSE32 Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<CSSF_A>;
#[doc = "HSE32 Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<CSSF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSF_A {
        match self.bits {
            false => CSSF_A::NotInterrupted,
            true => CSSF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSF_A::Interrupted
    }
}
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag"]
pub type LSECSSF_R = crate::BitReader<LSECSSF_A>;
#[doc = "LSE Clock security system interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSF_A {
    #[doc = "0: Not interrupted"]
    NotInterrupted = 0,
    #[doc = "1: Interrupted"]
    Interrupted = 1,
}
impl From<LSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSF_A {
        match self.bits {
            false => LSECSSF_A::NotInterrupted,
            true => LSECSSF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSECSSF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSECSSF_A::Interrupted
    }
}
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
    #[doc = "Bit 2 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE32 ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - HSE32 Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Clock interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](index.html) module"]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cifr::R](R) reader structure"]
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
