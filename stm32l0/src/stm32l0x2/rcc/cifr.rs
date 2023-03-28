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
pub type LSIRDYF_R = crate::BitReader<LSIRDYFR_A>;
#[doc = "LSI ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYFR_A {
    #[doc = "0: No clock ready interrupt"]
    NotInterrupted = 0,
    #[doc = "1: Clock ready interrupt"]
    Interrupted = 1,
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
            false => LSIRDYFR_A::NotInterrupted,
            true => LSIRDYFR_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYFR_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYFR_A::Interrupted
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `HSI16RDYF` reader - HSI16 ready interrupt flag"]
pub use LSIRDYF_R as HSI16RDYF_R;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub use LSIRDYF_R as PLLRDYF_R;
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub use LSIRDYF_R as MSIRDYF_R;
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag"]
pub use LSIRDYF_R as HSI48RDYF_R;
#[doc = "Field `CSSLSEF` reader - LSE Clock Security System Interrupt flag"]
pub type CSSLSEF_R = crate::BitReader<CSSLSEF_A>;
#[doc = "LSE Clock Security System Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSEF_A {
    #[doc = "0: No failure detected on LSE clock failure"]
    NoFailure = 0,
    #[doc = "1: Failure detected on LSE clock failure"]
    Failure = 1,
}
impl From<CSSLSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSEF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSLSEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSLSEF_A {
        match self.bits {
            false => CSSLSEF_A::NoFailure,
            true => CSSLSEF_A::Failure,
        }
    }
    #[doc = "Checks if the value of the field is `NoFailure`"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CSSLSEF_A::NoFailure
    }
    #[doc = "Checks if the value of the field is `Failure`"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CSSLSEF_A::Failure
    }
}
#[doc = "Field `CSSHSEF` reader - Clock Security System Interrupt flag"]
pub type CSSHSEF_R = crate::BitReader<CSSHSEF_A>;
#[doc = "Clock Security System Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSHSEF_A {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NoClock = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Clock = 1,
}
impl From<CSSHSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSHSEF_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSHSEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSHSEF_A {
        match self.bits {
            false => CSSHSEF_A::NoClock,
            true => CSSHSEF_A::Clock,
        }
    }
    #[doc = "Checks if the value of the field is `NoClock`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == CSSHSEF_A::NoClock
    }
    #[doc = "Checks if the value of the field is `Clock`"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == CSSHSEF_A::Clock
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
    #[doc = "Bit 2 - HSI16 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bit 6 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csslsef(&self) -> CSSLSEF_R {
        CSSLSEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn csshsef(&self) -> CSSHSEF_R {
        CSSHSEF_R::new(((self.bits >> 8) & 1) != 0)
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
