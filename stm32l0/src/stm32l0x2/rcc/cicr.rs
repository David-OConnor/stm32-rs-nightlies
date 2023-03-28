#[doc = "Register `CICR` writer"]
pub struct W(crate::W<CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICR_SPEC>;
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
impl From<crate::W<CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LSI ready Interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYCW_AW {
    #[doc = "1: Clear interrupt flag"]
    Clear = 1,
}
impl From<LSIRDYCW_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready Interrupt clear"]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSIRDYCW_AW, O>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYCW_AW::Clear)
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready Interrupt clear"]
pub use LSIRDYC_W as LSERDYC_W;
#[doc = "Field `HSI16RDYC` writer - HSI16 ready Interrupt clear"]
pub use LSIRDYC_W as HSI16RDYC_W;
#[doc = "Field `HSERDYC` writer - HSE ready Interrupt clear"]
pub use LSIRDYC_W as HSERDYC_W;
#[doc = "Field `PLLRDYC` writer - PLL ready Interrupt clear"]
pub use LSIRDYC_W as PLLRDYC_W;
#[doc = "Field `MSIRDYC` writer - MSI ready Interrupt clear"]
pub use LSIRDYC_W as MSIRDYC_W;
#[doc = "Field `HSI48RDYC` writer - HSI48 ready Interrupt clear"]
pub use LSIRDYC_W as HSI48RDYC_W;
#[doc = "Field `CSSLSEC` writer - LSE Clock Security System Interrupt clear"]
pub use LSIRDYC_W as CSSLSEC_W;
#[doc = "Field `CSSHSEC` writer - Clock Security System Interrupt clear"]
pub use LSIRDYC_W as CSSHSEC_W;
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<0> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<1> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 2 - HSI16 ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16rdyc(&mut self) -> HSI16RDYC_W<2> {
        HSI16RDYC_W::new(self)
    }
    #[doc = "Bit 3 - HSE ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<3> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 4 - PLL ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<4> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 5 - MSI ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<5> {
        MSIRDYC_W::new(self)
    }
    #[doc = "Bit 6 - HSI48 ready Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<6> {
        HSI48RDYC_W::new(self)
    }
    #[doc = "Bit 7 - LSE Clock Security System Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn csslsec(&mut self) -> CSSLSEC_W<7> {
        CSSLSEC_W::new(self)
    }
    #[doc = "Bit 8 - Clock Security System Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn csshsec(&mut self) -> CSSHSEC_W<8> {
        CSSHSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicr](index.html) module"]
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cicr::W](W) writer structure"]
impl crate::Writable for CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
