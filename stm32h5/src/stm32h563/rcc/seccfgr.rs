#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSISEC` reader - HSI clock configuration and status bits security Set and reset by software."]
pub type HSISEC_R = crate::BitReader<bool>;
#[doc = "Field `HSISEC` writer - HSI clock configuration and status bits security Set and reset by software."]
pub type HSISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `HSESEC` reader - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
pub type HSESEC_R = crate::BitReader<bool>;
#[doc = "Field `HSESEC` writer - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
pub type HSESEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `CSISEC` reader - CSI clock configuration and status bits security Set and reset by software."]
pub type CSISEC_R = crate::BitReader<bool>;
#[doc = "Field `CSISEC` writer - CSI clock configuration and status bits security Set and reset by software."]
pub type CSISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `LSISEC` reader - LSI clock configuration and status bits security Set and reset by software."]
pub type LSISEC_R = crate::BitReader<bool>;
#[doc = "Field `LSISEC` writer - LSI clock configuration and status bits security Set and reset by software."]
pub type LSISEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `LSESEC` reader - LSE clock configuration and status bits security Set and reset by software."]
pub type LSESEC_R = crate::BitReader<bool>;
#[doc = "Field `LSESEC` writer - LSE clock configuration and status bits security Set and reset by software."]
pub type LSESEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SYSCLKSEC` reader - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
pub type SYSCLKSEC_R = crate::BitReader<bool>;
#[doc = "Field `SYSCLKSEC` writer - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
pub type SYSCLKSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PRESCSEC` reader - AHBx/APBx prescaler configuration bits security Set and reset by software."]
pub type PRESCSEC_R = crate::BitReader<bool>;
#[doc = "Field `PRESCSEC` writer - AHBx/APBx prescaler configuration bits security Set and reset by software."]
pub type PRESCSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PLL1SEC` reader - PLL1 clock configuration and status bits security Set and reset by software."]
pub type PLL1SEC_R = crate::BitReader<bool>;
#[doc = "Field `PLL1SEC` writer - PLL1 clock configuration and status bits security Set and reset by software."]
pub type PLL1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PLL2SEC` reader - PLL2 clock configuration and status bits security Set and reset by software."]
pub type PLL2SEC_R = crate::BitReader<bool>;
#[doc = "Field `PLL2SEC` writer - PLL2 clock configuration and status bits security Set and reset by software."]
pub type PLL2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `PLL3SEC` reader - PLL3 clock configuration and status bits security Set and reset by software."]
pub type PLL3SEC_R = crate::BitReader<bool>;
#[doc = "Field `PLL3SEC` writer - PLL3 clock configuration and status bits security Set and reset by software."]
pub type PLL3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `HSI48SEC` reader - HSI48 clock configuration and status bits security Set and reset by software."]
pub type HSI48SEC_R = crate::BitReader<bool>;
#[doc = "Field `HSI48SEC` writer - HSI48 clock configuration and status bits security Set and reset by software."]
pub type HSI48SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `RMVFSEC` reader - Remove reset flag security Set and reset by software."]
pub type RMVFSEC_R = crate::BitReader<bool>;
#[doc = "Field `RMVFSEC` writer - Remove reset flag security Set and reset by software."]
pub type RMVFSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `CKPERSELSEC` reader - per_ck selection security Set and reset by software."]
pub type CKPERSELSEC_R = crate::BitReader<bool>;
#[doc = "Field `CKPERSELSEC` writer - per_ck selection security Set and reset by software."]
pub type CKPERSELSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn csisec(&self) -> CSISEC_R {
        CSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSE clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHBx/APBx prescaler configuration bits security Set and reset by software."]
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL1 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn pll1sec(&self) -> PLL1SEC_R {
        PLL1SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL2 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn pll2sec(&self) -> PLL2SEC_R {
        PLL2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL3 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn pll3sec(&self) -> PLL3SEC_R {
        PLL3SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI48 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Remove reset flag security Set and reset by software."]
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - per_ck selection security Set and reset by software."]
    #[inline(always)]
    pub fn ckperselsec(&self) -> CKPERSELSEC_R {
        CKPERSELSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsisec(&mut self) -> HSISEC_W<0> {
        HSISEC_W::new(self)
    }
    #[doc = "Bit 1 - HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsesec(&mut self) -> HSESEC_W<1> {
        HSESEC_W::new(self)
    }
    #[doc = "Bit 2 - CSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn csisec(&mut self) -> CSISEC_W<2> {
        CSISEC_W::new(self)
    }
    #[doc = "Bit 3 - LSI clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsisec(&mut self) -> LSISEC_W<3> {
        LSISEC_W::new(self)
    }
    #[doc = "Bit 4 - LSE clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsesec(&mut self) -> LSESEC_W<4> {
        LSESEC_W::new(self)
    }
    #[doc = "Bit 5 - SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<5> {
        SYSCLKSEC_W::new(self)
    }
    #[doc = "Bit 6 - AHBx/APBx prescaler configuration bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn prescsec(&mut self) -> PRESCSEC_W<6> {
        PRESCSEC_W::new(self)
    }
    #[doc = "Bit 7 - PLL1 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll1sec(&mut self) -> PLL1SEC_W<7> {
        PLL1SEC_W::new(self)
    }
    #[doc = "Bit 8 - PLL2 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll2sec(&mut self) -> PLL2SEC_W<8> {
        PLL2SEC_W::new(self)
    }
    #[doc = "Bit 9 - PLL3 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn pll3sec(&mut self) -> PLL3SEC_W<9> {
        PLL3SEC_W::new(self)
    }
    #[doc = "Bit 11 - HSI48 clock configuration and status bits security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<11> {
        HSI48SEC_W::new(self)
    }
    #[doc = "Bit 12 - Remove reset flag security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<12> {
        RMVFSEC_W::new(self)
    }
    #[doc = "Bit 13 - per_ck selection security Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ckperselsec(&mut self) -> CKPERSELSEC_W<13> {
        CKPERSELSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
