#[doc = "Register `CEC_IER` reader"]
pub struct R(crate::R<CEC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_IER` writer"]
pub struct W(crate::W<CEC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_IER_SPEC>;
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
impl From<crate::W<CEC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXBRIE` reader - RXBRIE"]
pub type RXBRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXBRIE` writer - RXBRIE"]
pub type RXBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `RXENDIE` reader - RXENDIE"]
pub type RXENDIE_R = crate::BitReader<bool>;
#[doc = "Field `RXENDIE` writer - RXENDIE"]
pub type RXENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `RXOVRIE` reader - RXOVRIE"]
pub type RXOVRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVRIE` writer - RXOVRIE"]
pub type RXOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `BREIE` reader - BREIE"]
pub type BREIE_R = crate::BitReader<bool>;
#[doc = "Field `BREIE` writer - BREIE"]
pub type BREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `SBPEIE` reader - SBPEIE"]
pub type SBPEIE_R = crate::BitReader<bool>;
#[doc = "Field `SBPEIE` writer - SBPEIE"]
pub type SBPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `LBPEIE` reader - LBPEIE"]
pub type LBPEIE_R = crate::BitReader<bool>;
#[doc = "Field `LBPEIE` writer - LBPEIE"]
pub type LBPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `RXACKIE` reader - RXACKIE"]
pub type RXACKIE_R = crate::BitReader<bool>;
#[doc = "Field `RXACKIE` writer - RXACKIE"]
pub type RXACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `ARBLSTIE` reader - ARBLSTIE"]
pub type ARBLSTIE_R = crate::BitReader<bool>;
#[doc = "Field `ARBLSTIE` writer - ARBLSTIE"]
pub type ARBLSTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXBRIE` reader - TXBRIE"]
pub type TXBRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXBRIE` writer - TXBRIE"]
pub type TXBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXENDIE` reader - TXENDIE"]
pub type TXENDIE_R = crate::BitReader<bool>;
#[doc = "Field `TXENDIE` writer - TXENDIE"]
pub type TXENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXUDRIE` reader - TXUDRIE"]
pub type TXUDRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUDRIE` writer - TXUDRIE"]
pub type TXUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXERRIE` reader - TXERRIE"]
pub type TXERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXERRIE` writer - TXERRIE"]
pub type TXERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXACKIE` reader - TXACKIE"]
pub type TXACKIE_R = crate::BitReader<bool>;
#[doc = "Field `TXACKIE` writer - TXACKIE"]
pub type TXACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXBRIE"]
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXENDIE"]
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXOVRIE"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BREIE"]
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SBPEIE"]
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LBPEIE"]
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXACKIE"]
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARBLSTIE"]
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXBRIE"]
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXENDIE"]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUDRIE"]
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXERRIE"]
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACKIE"]
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXBRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbrie(&mut self) -> RXBRIE_W<0> {
        RXBRIE_W::new(self)
    }
    #[doc = "Bit 1 - RXENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxendie(&mut self) -> RXENDIE_W<1> {
        RXENDIE_W::new(self)
    }
    #[doc = "Bit 2 - RXOVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<2> {
        RXOVRIE_W::new(self)
    }
    #[doc = "Bit 3 - BREIE"]
    #[inline(always)]
    #[must_use]
    pub fn breie(&mut self) -> BREIE_W<3> {
        BREIE_W::new(self)
    }
    #[doc = "Bit 4 - SBPEIE"]
    #[inline(always)]
    #[must_use]
    pub fn sbpeie(&mut self) -> SBPEIE_W<4> {
        SBPEIE_W::new(self)
    }
    #[doc = "Bit 5 - LBPEIE"]
    #[inline(always)]
    #[must_use]
    pub fn lbpeie(&mut self) -> LBPEIE_W<5> {
        LBPEIE_W::new(self)
    }
    #[doc = "Bit 6 - RXACKIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxackie(&mut self) -> RXACKIE_W<6> {
        RXACKIE_W::new(self)
    }
    #[doc = "Bit 7 - ARBLSTIE"]
    #[inline(always)]
    #[must_use]
    pub fn arblstie(&mut self) -> ARBLSTIE_W<7> {
        ARBLSTIE_W::new(self)
    }
    #[doc = "Bit 8 - TXBRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbrie(&mut self) -> TXBRIE_W<8> {
        TXBRIE_W::new(self)
    }
    #[doc = "Bit 9 - TXENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn txendie(&mut self) -> TXENDIE_W<9> {
        TXENDIE_W::new(self)
    }
    #[doc = "Bit 10 - TXUDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txudrie(&mut self) -> TXUDRIE_W<10> {
        TXUDRIE_W::new(self)
    }
    #[doc = "Bit 11 - TXERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txerrie(&mut self) -> TXERRIE_W<11> {
        TXERRIE_W::new(self)
    }
    #[doc = "Bit 12 - TXACKIE"]
    #[inline(always)]
    #[must_use]
    pub fn txackie(&mut self) -> TXACKIE_W<12> {
        TXACKIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_ier](index.html) module"]
pub struct CEC_IER_SPEC;
impl crate::RegisterSpec for CEC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_ier::R](R) reader structure"]
impl crate::Readable for CEC_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_ier::W](W) writer structure"]
impl crate::Writable for CEC_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEC_IER to value 0"]
impl crate::Resettable for CEC_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
