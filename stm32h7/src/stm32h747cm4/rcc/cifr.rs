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
#[doc = "Field `LSIRDYF` reader - LSI ready Interrupt Flag"]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` reader - LSE ready Interrupt Flag"]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` reader - HSI ready Interrupt Flag"]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` reader - HSE ready Interrupt Flag"]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `CSIRDY` reader - CSI ready Interrupt Flag"]
pub type CSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `HSI48RDYF` reader - RC48 ready Interrupt Flag"]
pub type HSI48RDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLL1RDYF` reader - PLL1 ready Interrupt Flag"]
pub type PLL1RDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLL2RDYF` reader - PLL2 ready Interrupt Flag"]
pub type PLL2RDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLL3RDYF` reader - PLL3 ready Interrupt Flag"]
pub type PLL3RDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSF` reader - LSE clock security system Interrupt Flag"]
pub type LSECSSF_R = crate::BitReader<bool>;
#[doc = "Field `HSECSSF` reader - HSE clock security system Interrupt Flag"]
pub type HSECSSF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> PLL1RDYF_R {
        PLL1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> PLL2RDYF_R {
        PLL2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> PLL3RDYF_R {
        PLL3RDYF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "RCC Clock Source Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](index.html) module"]
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
