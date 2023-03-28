#[doc = "Register `APB3RSTR` reader"]
pub struct R(crate::R<APB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB3RSTR` writer"]
pub struct W(crate::W<APB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB3RSTR_SPEC>;
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
impl From<crate::W<APB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBSRST` reader - SBS block reset Set and reset by software."]
pub type SBSRST_R = crate::BitReader<bool>;
#[doc = "Field `SBSRST` writer - SBS block reset Set and reset by software."]
pub type SBSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset Set and reset by software."]
pub type LPUART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `I3C2RST` reader - I3C2RST block reset Set and reset by software."]
pub type I3C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I3C2RST` writer - I3C2RST block reset Set and reset by software."]
pub type I3C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 block reset Set and reset by software."]
pub type LPTIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 block reset Set and reset by software."]
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
#[doc = "Field `VREFRST` reader - VREF block reset Set and reset by software."]
pub type VREFRST_R = crate::BitReader<bool>;
#[doc = "Field `VREFRST` writer - VREF block reset Set and reset by software."]
pub type VREFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB3RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    pub fn sbsrst(&self) -> SBSRST_R {
        SBSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - I3C2RST block reset Set and reset by software."]
    #[inline(always)]
    pub fn i3c2rst(&self) -> I3C2RST_R {
        I3C2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SBS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn sbsrst(&mut self) -> SBSRST_W<1> {
        SBSRST_W::new(self)
    }
    #[doc = "Bit 6 - LPUART1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<6> {
        LPUART1RST_W::new(self)
    }
    #[doc = "Bit 9 - I3C2RST block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn i3c2rst(&mut self) -> I3C2RST_W<9> {
        I3C2RST_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM1 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<11> {
        LPTIM1RST_W::new(self)
    }
    #[doc = "Bit 20 - VREF block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<20> {
        VREFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB3 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3rstr](index.html) module"]
pub struct APB3RSTR_SPEC;
impl crate::RegisterSpec for APB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb3rstr::R](R) reader structure"]
impl crate::Readable for APB3RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb3rstr::W](W) writer structure"]
impl crate::Writable for APB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
