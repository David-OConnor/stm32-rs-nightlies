#[doc = "Register `TIM8_SR` reader"]
pub struct R(crate::R<TIM8_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM8_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM8_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM8_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM8_SR` writer"]
pub struct W(crate::W<TIM8_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM8_SR_SPEC>;
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
impl From<crate::W<TIM8_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM8_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIF` reader - UIF"]
pub type UIF_R = crate::BitReader<bool>;
#[doc = "Field `UIF` writer - UIF"]
pub type UIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC1IF` reader - CC1IF"]
pub type CC1IF_R = crate::BitReader<bool>;
#[doc = "Field `CC1IF` writer - CC1IF"]
pub type CC1IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC2IF` reader - CC2IF"]
pub type CC2IF_R = crate::BitReader<bool>;
#[doc = "Field `CC2IF` writer - CC2IF"]
pub type CC2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC3IF` reader - CC3IF"]
pub type CC3IF_R = crate::BitReader<bool>;
#[doc = "Field `CC3IF` writer - CC3IF"]
pub type CC3IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC4IF` reader - CC4IF"]
pub type CC4IF_R = crate::BitReader<bool>;
#[doc = "Field `CC4IF` writer - CC4IF"]
pub type CC4IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `COMIF` reader - COMIF"]
pub type COMIF_R = crate::BitReader<bool>;
#[doc = "Field `COMIF` writer - COMIF"]
pub type COMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `TIF` reader - TIF"]
pub type TIF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` writer - TIF"]
pub type TIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `BIF` reader - BIF"]
pub type BIF_R = crate::BitReader<bool>;
#[doc = "Field `BIF` writer - BIF"]
pub type BIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `B2IF` reader - B2IF"]
pub type B2IF_R = crate::BitReader<bool>;
#[doc = "Field `B2IF` writer - B2IF"]
pub type B2IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC1OF` reader - CC1OF"]
pub type CC1OF_R = crate::BitReader<bool>;
#[doc = "Field `CC1OF` writer - CC1OF"]
pub type CC1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC2OF` reader - CC2OF"]
pub type CC2OF_R = crate::BitReader<bool>;
#[doc = "Field `CC2OF` writer - CC2OF"]
pub type CC2OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC3OF` reader - CC3OF"]
pub type CC3OF_R = crate::BitReader<bool>;
#[doc = "Field `CC3OF` writer - CC3OF"]
pub type CC3OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC4OF` reader - CC4OF"]
pub type CC4OF_R = crate::BitReader<bool>;
#[doc = "Field `CC4OF` writer - CC4OF"]
pub type CC4OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `SBIF` reader - SBIF"]
pub type SBIF_R = crate::BitReader<bool>;
#[doc = "Field `SBIF` writer - SBIF"]
pub type SBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC5IF` reader - CC5IF"]
pub type CC5IF_R = crate::BitReader<bool>;
#[doc = "Field `CC5IF` writer - CC5IF"]
pub type CC5IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
#[doc = "Field `CC6IF` reader - CC6IF"]
pub type CC6IF_R = crate::BitReader<bool>;
#[doc = "Field `CC6IF` writer - CC6IF"]
pub type CC6IF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SR_SPEC, bool, O>;
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
    #[doc = "Bit 3 - CC3IF"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4IF"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 8 - B2IF"]
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 11 - CC3OF"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4OF"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SBIF"]
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CC5IF"]
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC6IF"]
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 3 - CC3IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc3if(&mut self) -> CC3IF_W<3> {
        CC3IF_W::new(self)
    }
    #[doc = "Bit 4 - CC4IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc4if(&mut self) -> CC4IF_W<4> {
        CC4IF_W::new(self)
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
    #[doc = "Bit 8 - B2IF"]
    #[inline(always)]
    #[must_use]
    pub fn b2if(&mut self) -> B2IF_W<8> {
        B2IF_W::new(self)
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
    #[doc = "Bit 11 - CC3OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc3of(&mut self) -> CC3OF_W<11> {
        CC3OF_W::new(self)
    }
    #[doc = "Bit 12 - CC4OF"]
    #[inline(always)]
    #[must_use]
    pub fn cc4of(&mut self) -> CC4OF_W<12> {
        CC4OF_W::new(self)
    }
    #[doc = "Bit 13 - SBIF"]
    #[inline(always)]
    #[must_use]
    pub fn sbif(&mut self) -> SBIF_W<13> {
        SBIF_W::new(self)
    }
    #[doc = "Bit 16 - CC5IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc5if(&mut self) -> CC5IF_W<16> {
        CC5IF_W::new(self)
    }
    #[doc = "Bit 17 - CC6IF"]
    #[inline(always)]
    #[must_use]
    pub fn cc6if(&mut self) -> CC6IF_W<17> {
        CC6IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM8 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_sr](index.html) module"]
pub struct TIM8_SR_SPEC;
impl crate::RegisterSpec for TIM8_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim8_sr::R](R) reader structure"]
impl crate::Readable for TIM8_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim8_sr::W](W) writer structure"]
impl crate::Writable for TIM8_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM8_SR to value 0"]
impl crate::Resettable for TIM8_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
