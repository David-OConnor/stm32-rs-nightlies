#[doc = "Register `CEC_ISR` reader"]
pub struct R(crate::R<CEC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_ISR` writer"]
pub struct W(crate::W<CEC_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_ISR_SPEC>;
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
impl From<crate::W<CEC_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXBR` reader - RXBR"]
pub type RXBR_R = crate::BitReader<bool>;
#[doc = "Field `RXBR` writer - RXBR"]
pub type RXBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `RXEND` reader - RXEND"]
pub type RXEND_R = crate::BitReader<bool>;
#[doc = "Field `RXEND` writer - RXEND"]
pub type RXEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `RXOVR` reader - RXOVR"]
pub type RXOVR_R = crate::BitReader<bool>;
#[doc = "Field `RXOVR` writer - RXOVR"]
pub type RXOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `BRE` reader - BRE"]
pub type BRE_R = crate::BitReader<bool>;
#[doc = "Field `BRE` writer - BRE"]
pub type BRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `SBPE` reader - SBPE"]
pub type SBPE_R = crate::BitReader<bool>;
#[doc = "Field `SBPE` writer - SBPE"]
pub type SBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `LBPE` reader - LBPE"]
pub type LBPE_R = crate::BitReader<bool>;
#[doc = "Field `LBPE` writer - LBPE"]
pub type LBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `RXACKE` reader - RXACKE"]
pub type RXACKE_R = crate::BitReader<bool>;
#[doc = "Field `RXACKE` writer - RXACKE"]
pub type RXACKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `ARBLST` reader - ARBLST"]
pub type ARBLST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLST` writer - ARBLST"]
pub type ARBLST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXBR` reader - TXBR"]
pub type TXBR_R = crate::BitReader<bool>;
#[doc = "Field `TXBR` writer - TXBR"]
pub type TXBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXEND` reader - TXEND"]
pub type TXEND_R = crate::BitReader<bool>;
#[doc = "Field `TXEND` writer - TXEND"]
pub type TXEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXUDR` reader - TXUDR"]
pub type TXUDR_R = crate::BitReader<bool>;
#[doc = "Field `TXUDR` writer - TXUDR"]
pub type TXUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXERR` reader - TXERR"]
pub type TXERR_R = crate::BitReader<bool>;
#[doc = "Field `TXERR` writer - TXERR"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXACKE` reader - TXACKE"]
pub type TXACKE_R = crate::BitReader<bool>;
#[doc = "Field `TXACKE` writer - TXACKE"]
pub type TXACKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXBR"]
    #[inline(always)]
    pub fn rxbr(&self) -> RXBR_R {
        RXBR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXEND"]
    #[inline(always)]
    pub fn rxend(&self) -> RXEND_R {
        RXEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRE"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SBPE"]
    #[inline(always)]
    pub fn sbpe(&self) -> SBPE_R {
        SBPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LBPE"]
    #[inline(always)]
    pub fn lbpe(&self) -> LBPE_R {
        LBPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXACKE"]
    #[inline(always)]
    pub fn rxacke(&self) -> RXACKE_R {
        RXACKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARBLST"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TXBR"]
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEND"]
    #[inline(always)]
    pub fn txend(&self) -> TXEND_R {
        TXEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TXUDR"]
    #[inline(always)]
    pub fn txudr(&self) -> TXUDR_R {
        TXUDR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TXERR"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACKE"]
    #[inline(always)]
    pub fn txacke(&self) -> TXACKE_R {
        TXACKE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXBR"]
    #[inline(always)]
    #[must_use]
    pub fn rxbr(&mut self) -> RXBR_W<0> {
        RXBR_W::new(self)
    }
    #[doc = "Bit 1 - RXEND"]
    #[inline(always)]
    #[must_use]
    pub fn rxend(&mut self) -> RXEND_W<1> {
        RXEND_W::new(self)
    }
    #[doc = "Bit 2 - RXOVR"]
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RXOVR_W<2> {
        RXOVR_W::new(self)
    }
    #[doc = "Bit 3 - BRE"]
    #[inline(always)]
    #[must_use]
    pub fn bre(&mut self) -> BRE_W<3> {
        BRE_W::new(self)
    }
    #[doc = "Bit 4 - SBPE"]
    #[inline(always)]
    #[must_use]
    pub fn sbpe(&mut self) -> SBPE_W<4> {
        SBPE_W::new(self)
    }
    #[doc = "Bit 5 - LBPE"]
    #[inline(always)]
    #[must_use]
    pub fn lbpe(&mut self) -> LBPE_W<5> {
        LBPE_W::new(self)
    }
    #[doc = "Bit 6 - RXACKE"]
    #[inline(always)]
    #[must_use]
    pub fn rxacke(&mut self) -> RXACKE_W<6> {
        RXACKE_W::new(self)
    }
    #[doc = "Bit 7 - ARBLST"]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ARBLST_W<7> {
        ARBLST_W::new(self)
    }
    #[doc = "Bit 8 - TXBR"]
    #[inline(always)]
    #[must_use]
    pub fn txbr(&mut self) -> TXBR_W<8> {
        TXBR_W::new(self)
    }
    #[doc = "Bit 9 - TXEND"]
    #[inline(always)]
    #[must_use]
    pub fn txend(&mut self) -> TXEND_W<9> {
        TXEND_W::new(self)
    }
    #[doc = "Bit 10 - TXUDR"]
    #[inline(always)]
    #[must_use]
    pub fn txudr(&mut self) -> TXUDR_W<10> {
        TXUDR_W::new(self)
    }
    #[doc = "Bit 11 - TXERR"]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<11> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 12 - TXACKE"]
    #[inline(always)]
    #[must_use]
    pub fn txacke(&mut self) -> TXACKE_W<12> {
        TXACKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC Interrupt and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_isr](index.html) module"]
pub struct CEC_ISR_SPEC;
impl crate::RegisterSpec for CEC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_isr::R](R) reader structure"]
impl crate::Readable for CEC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_isr::W](W) writer structure"]
impl crate::Writable for CEC_ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CEC_ISR to value 0"]
impl crate::Resettable for CEC_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
