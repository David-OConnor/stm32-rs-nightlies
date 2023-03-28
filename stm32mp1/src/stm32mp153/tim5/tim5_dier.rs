#[doc = "Register `TIM5_DIER` reader"]
pub struct R(crate::R<TIM5_DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_DIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_DIER` writer"]
pub struct W(crate::W<TIM5_DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_DIER_SPEC>;
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
impl From<crate::W<TIM5_DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_DIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIE` reader - UIE"]
pub type UIE_R = crate::BitReader<bool>;
#[doc = "Field `UIE` writer - UIE"]
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC1IE` reader - CC1IE"]
pub type CC1IE_R = crate::BitReader<bool>;
#[doc = "Field `CC1IE` writer - CC1IE"]
pub type CC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC2IE` reader - CC2IE"]
pub type CC2IE_R = crate::BitReader<bool>;
#[doc = "Field `CC2IE` writer - CC2IE"]
pub type CC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC3IE` reader - CC3IE"]
pub type CC3IE_R = crate::BitReader<bool>;
#[doc = "Field `CC3IE` writer - CC3IE"]
pub type CC3IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC4IE` reader - CC4IE"]
pub type CC4IE_R = crate::BitReader<bool>;
#[doc = "Field `CC4IE` writer - CC4IE"]
pub type CC4IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `COMIE` reader - COMIE"]
pub type COMIE_R = crate::BitReader<bool>;
#[doc = "Field `COMIE` writer - COMIE"]
pub type COMIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `TIE` reader - TIE"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - TIE"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `BIE` reader - BIE"]
pub type BIE_R = crate::BitReader<bool>;
#[doc = "Field `BIE` writer - BIE"]
pub type BIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `UDE` reader - UDE"]
pub type UDE_R = crate::BitReader<bool>;
#[doc = "Field `UDE` writer - UDE"]
pub type UDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC1DE` reader - CC1DE"]
pub type CC1DE_R = crate::BitReader<bool>;
#[doc = "Field `CC1DE` writer - CC1DE"]
pub type CC1DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC2DE` reader - CC2DE"]
pub type CC2DE_R = crate::BitReader<bool>;
#[doc = "Field `CC2DE` writer - CC2DE"]
pub type CC2DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC3DE` reader - CC3DE"]
pub type CC3DE_R = crate::BitReader<bool>;
#[doc = "Field `CC3DE` writer - CC3DE"]
pub type CC3DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `CC4DE` reader - CC4DE"]
pub type CC4DE_R = crate::BitReader<bool>;
#[doc = "Field `CC4DE` writer - CC4DE"]
pub type CC4DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `COMDE` reader - COMDE"]
pub type COMDE_R = crate::BitReader<bool>;
#[doc = "Field `COMDE` writer - COMDE"]
pub type COMDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
#[doc = "Field `TDE` reader - TDE"]
pub type TDE_R = crate::BitReader<bool>;
#[doc = "Field `TDE` writer - TDE"]
pub type TDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM5_DIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1IE"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2IE"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC3IE"]
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4IE"]
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMIE"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDE"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1DE"]
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2DE"]
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3DE"]
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4DE"]
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COMDE"]
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TDE"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIE"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<0> {
        UIE_W::new(self)
    }
    #[doc = "Bit 1 - CC1IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<1> {
        CC1IE_W::new(self)
    }
    #[doc = "Bit 2 - CC2IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<2> {
        CC2IE_W::new(self)
    }
    #[doc = "Bit 3 - CC3IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ie(&mut self) -> CC3IE_W<3> {
        CC3IE_W::new(self)
    }
    #[doc = "Bit 4 - CC4IE"]
    #[inline(always)]
    #[must_use]
    pub fn cc4ie(&mut self) -> CC4IE_W<4> {
        CC4IE_W::new(self)
    }
    #[doc = "Bit 5 - COMIE"]
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> COMIE_W<5> {
        COMIE_W::new(self)
    }
    #[doc = "Bit 6 - TIE"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 7 - BIE"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<7> {
        BIE_W::new(self)
    }
    #[doc = "Bit 8 - UDE"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<8> {
        UDE_W::new(self)
    }
    #[doc = "Bit 9 - CC1DE"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<9> {
        CC1DE_W::new(self)
    }
    #[doc = "Bit 10 - CC2DE"]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CC2DE_W<10> {
        CC2DE_W::new(self)
    }
    #[doc = "Bit 11 - CC3DE"]
    #[inline(always)]
    #[must_use]
    pub fn cc3de(&mut self) -> CC3DE_W<11> {
        CC3DE_W::new(self)
    }
    #[doc = "Bit 12 - CC4DE"]
    #[inline(always)]
    #[must_use]
    pub fn cc4de(&mut self) -> CC4DE_W<12> {
        CC4DE_W::new(self)
    }
    #[doc = "Bit 13 - COMDE"]
    #[inline(always)]
    #[must_use]
    pub fn comde(&mut self) -> COMDE_W<13> {
        COMDE_W::new(self)
    }
    #[doc = "Bit 14 - TDE"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<14> {
        TDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_dier](index.html) module"]
pub struct TIM5_DIER_SPEC;
impl crate::RegisterSpec for TIM5_DIER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim5_dier::R](R) reader structure"]
impl crate::Readable for TIM5_DIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_dier::W](W) writer structure"]
impl crate::Writable for TIM5_DIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM5_DIER to value 0"]
impl crate::Resettable for TIM5_DIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
