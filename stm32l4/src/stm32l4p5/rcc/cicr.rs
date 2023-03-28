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
#[doc = "LSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYC_AW {
    #[doc = "1: Clear the LSIRDYF flag"]
    Clear = 1,
}
impl From<LSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSIRDYC_AW, O>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    #[doc = "Clear the LSIRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYC_AW::Clear)
    }
}
#[doc = "LSE ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYC_AW {
    #[doc = "1: Clear the LSERDYF flag"]
    Clear = 1,
}
impl From<LSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSERDYC_AW, O>;
impl<'a, const O: u8> LSERDYC_W<'a, O> {
    #[doc = "Clear the LSERDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSERDYC_AW::Clear)
    }
}
#[doc = "MSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYC_AW {
    #[doc = "1: Clear the MSIRDYF flag"]
    Clear = 1,
}
impl From<MSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDYC` writer - MSI ready interrupt clear"]
pub type MSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, MSIRDYC_AW, O>;
impl<'a, const O: u8> MSIRDYC_W<'a, O> {
    #[doc = "Clear the MSIRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MSIRDYC_AW::Clear)
    }
}
#[doc = "HSI ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYC_AW {
    #[doc = "1: Clear HSIRDYF flag"]
    Clear = 1,
}
impl From<HSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, HSIRDYC_AW, O>;
impl<'a, const O: u8> HSIRDYC_W<'a, O> {
    #[doc = "Clear HSIRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSIRDYC_AW::Clear)
    }
}
#[doc = "HSE ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDYC_AW {
    #[doc = "1: Clear HSERDYF flag"]
    Clear = 1,
}
impl From<HSERDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSERDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, HSERDYC_AW, O>;
impl<'a, const O: u8> HSERDYC_W<'a, O> {
    #[doc = "Clear HSERDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSERDYC_AW::Clear)
    }
}
#[doc = "PLL ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYC_AW {
    #[doc = "1: Clear PLLRDYF flag"]
    Clear = 1,
}
impl From<PLLRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDYC` writer - PLL ready interrupt clear"]
pub type PLLRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, PLLRDYC_AW, O>;
impl<'a, const O: u8> PLLRDYC_W<'a, O> {
    #[doc = "Clear PLLRDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLRDYC_AW::Clear)
    }
}
#[doc = "PLLSAI1 ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDYC_AW {
    #[doc = "1: Clear PLLSAI1RDYF flag"]
    Clear = 1,
}
impl From<PLLSAI1RDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear"]
pub type PLLSAI1RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, PLLSAI1RDYC_AW, O>;
impl<'a, const O: u8> PLLSAI1RDYC_W<'a, O> {
    #[doc = "Clear PLLSAI1RDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLSAI1RDYC_AW::Clear)
    }
}
#[doc = "PLLSAI2 ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDYC_AW {
    #[doc = "1: Clear PLLSAI2RDYF flag"]
    Clear = 1,
}
impl From<PLLSAI2RDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSAI2RDYC` writer - PLLSAI2 ready interrupt clear"]
pub type PLLSAI2RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, PLLSAI2RDYC_AW, O>;
impl<'a, const O: u8> PLLSAI2RDYC_W<'a, O> {
    #[doc = "Clear PLLSAI2RDYF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PLLSAI2RDYC_AW::Clear)
    }
}
#[doc = "Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSC_AW {
    #[doc = "1: Clear the CSSF flag"]
    Clear = 1,
}
impl From<CSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, CSSC_AW, O>;
impl<'a, const O: u8> CSSC_W<'a, O> {
    #[doc = "Clear the CSSF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSC_AW::Clear)
    }
}
#[doc = "LSE Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSC_AW {
    #[doc = "1: Clear the LSECSSF flag"]
    Clear = 1,
}
impl From<LSECSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSECSSC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear"]
pub type LSECSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, LSECSSC_AW, O>;
impl<'a, const O: u8> LSECSSC_W<'a, O> {
    #[doc = "Clear the LSECSSF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSECSSC_AW::Clear)
    }
}
#[doc = "HSI48 oscillator ready interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI48RDYC_AW {
    #[doc = "1: Clear the HSI48RDYC flag"]
    Clear = 1,
}
impl From<HSI48RDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: HSI48RDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI48RDYC` writer - HSI48 oscillator ready interrupt clear"]
pub type HSI48RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, HSI48RDYC_AW, O>;
impl<'a, const O: u8> HSI48RDYC_W<'a, O> {
    #[doc = "Clear the HSI48RDYC flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HSI48RDYC_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<0> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<1> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 2 - MSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<2> {
        MSIRDYC_W::new(self)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<3> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<4> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<5> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W<6> {
        PLLSAI1RDYC_W::new(self)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllsai2rdyc(&mut self) -> PLLSAI2RDYC_W<7> {
        PLLSAI2RDYC_W::new(self)
    }
    #[doc = "Bit 8 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<8> {
        CSSC_W::new(self)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<9> {
        LSECSSC_W::new(self)
    }
    #[doc = "Bit 10 - HSI48 oscillator ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<10> {
        HSI48RDYC_W::new(self)
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
