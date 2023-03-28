#[doc = "Register `RCC_MC_CIER` reader"]
pub struct R(crate::R<RCC_MC_CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_CIER` writer"]
pub struct W(crate::W<RCC_MC_CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_CIER_SPEC>;
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
impl From<crate::W<RCC_MC_CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_CIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYIE` reader - LSIRDYIE"]
pub type LSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` writer - LSIRDYIE"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `LSERDYIE` reader - LSERDYIE"]
pub type LSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYIE` writer - LSERDYIE"]
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `HSIRDYIE` reader - HSIRDYIE"]
pub type HSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYIE` writer - HSIRDYIE"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `HSERDYIE` reader - HSERDYIE"]
pub type HSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYIE` writer - HSERDYIE"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `CSIRDYIE` reader - CSIRDYIE"]
pub type CSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `CSIRDYIE` writer - CSIRDYIE"]
pub type CSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `PLL1DYIE` reader - PLL1DYIE"]
pub type PLL1DYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLL1DYIE` writer - PLL1DYIE"]
pub type PLL1DYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `PLL2DYIE` reader - PLL2DYIE"]
pub type PLL2DYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLL2DYIE` writer - PLL2DYIE"]
pub type PLL2DYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `PLL3DYIE` reader - PLL3DYIE"]
pub type PLL3DYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLL3DYIE` writer - PLL3DYIE"]
pub type PLL3DYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `PLL4DYIE` reader - PLL4DYIE"]
pub type PLL4DYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLL4DYIE` writer - PLL4DYIE"]
pub type PLL4DYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `LSECSSIE` reader - LSECSSIE"]
pub type LSECSSIE_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSIE` writer - LSECSSIE"]
pub type LSECSSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
#[doc = "Field `WKUPIE` reader - WKUPIE"]
pub type WKUPIE_R = crate::BitReader<bool>;
#[doc = "Field `WKUPIE` writer - WKUPIE"]
pub type WKUPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_CIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    pub fn pll1dyie(&self) -> PLL1DYIE_R {
        PLL1DYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    pub fn pll2dyie(&self) -> PLL2DYIE_R {
        PLL2DYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    pub fn pll3dyie(&self) -> PLL3DYIE_R {
        PLL3DYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    pub fn pll4dyie(&self) -> PLL4DYIE_R {
        PLL4DYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<2> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<3> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    #[must_use]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W<4> {
        CSIRDYIE_W::new(self)
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll1dyie(&mut self) -> PLL1DYIE_W<8> {
        PLL1DYIE_W::new(self)
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll2dyie(&mut self) -> PLL2DYIE_W<9> {
        PLL2DYIE_W::new(self)
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll3dyie(&mut self) -> PLL3DYIE_W<10> {
        PLL3DYIE_W::new(self)
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    #[must_use]
    pub fn pll4dyie(&mut self) -> PLL4DYIE_W<11> {
        PLL4DYIE_W::new(self)
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<16> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    #[must_use]
    pub fn wkupie(&mut self) -> WKUPIE_W<20> {
        WKUPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_cier](index.html) module"]
pub struct RCC_MC_CIER_SPEC;
impl crate::RegisterSpec for RCC_MC_CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_cier::R](R) reader structure"]
impl crate::Readable for RCC_MC_CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_cier::W](W) writer structure"]
impl crate::Writable for RCC_MC_CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MC_CIER to value 0"]
impl crate::Resettable for RCC_MC_CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
