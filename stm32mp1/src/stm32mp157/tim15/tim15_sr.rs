#[doc = "Register `TIM15_SR` reader"]
pub struct R(crate::R<TIM15_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_SR` writer"]
pub struct W(crate::W<TIM15_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_SR_SPEC>;
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
impl From<crate::W<TIM15_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIF` reader - UIF"]
pub type UIF_R = crate::BitReader<bool>;
#[doc = "Field `UIF` writer - UIF"]
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
#[doc = "Field `CC1IF` reader - CC1IF"]
pub type CC1IF_R = crate::BitReader<bool>;
#[doc = "Field `CC1IF` writer - CC1IF"]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
#[doc = "Field `CC2IF` reader - CC2IF"]
pub type CC2IF_R = crate::BitReader<bool>;
#[doc = "Field `CC2IF` writer - CC2IF"]
pub type CC2IF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
#[doc = "Field `COMIF` reader - COMIF"]
pub type COMIF_R = crate::BitReader<bool>;
#[doc = "Field `COMIF` writer - COMIF"]
pub type COMIF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
#[doc = "Field `TIF` reader - TIF"]
pub type TIF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` writer - TIF"]
pub type TIF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
#[doc = "Field `BIF` reader - BIF"]
pub type BIF_R = crate::BitReader<bool>;
#[doc = "Field `BIF` writer - BIF"]
pub type BIF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
#[doc = "Field `CC1OF` reader - CC1OF"]
pub type CC1OF_R = crate::BitReader<bool>;
#[doc = "Field `CC1OF` writer - CC1OF"]
pub type CC1OF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
#[doc = "Field `CC2OF` reader - CC2OF"]
pub type CC2OF_R = crate::BitReader<bool>;
#[doc = "Field `CC2OF` writer - CC2OF"]
pub type CC2OF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM15_SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1IF"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2IF"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - COMIF"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIF"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2OF"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    #[must_use]
    pub fn uif(&mut self) -> UIF_W<0> {
        UIF_W::new(self)
    }
    #[doc = "Bit 1 - CC1IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<1> {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 2 - CC2IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<2> {
        CC2IF_W::new(self)
    }
    #[doc = "Bit 5 - COMIF"]
    #[inline(always)]
    #[must_use]
    pub fn comif(&mut self) -> COMIF_W<5> {
        COMIF_W::new(self)
    }
    #[doc = "Bit 6 - TIF"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<6> {
        TIF_W::new(self)
    }
    #[doc = "Bit 7 - BIF"]
    #[inline(always)]
    #[must_use]
    pub fn bif(&mut self) -> BIF_W<7> {
        BIF_W::new(self)
    }
    #[doc = "Bit 9 - CC1OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc1of(&mut self) -> CC1OF_W<9> {
        CC1OF_W::new(self)
    }
    #[doc = "Bit 10 - CC2OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc2of(&mut self) -> CC2OF_W<10> {
        CC2OF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_sr](index.html) module"]
pub struct TIM15_SR_SPEC;
impl crate::RegisterSpec for TIM15_SR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim15_sr::R](R) reader structure"]
impl crate::Readable for TIM15_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_sr::W](W) writer structure"]
impl crate::Writable for TIM15_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM15_SR to value 0"]
impl crate::Resettable for TIM15_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
