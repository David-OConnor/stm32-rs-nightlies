#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXMSGDISCCF` reader - TXMSGDISCCF"]
pub type TXMSGDISCCF_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGDISCCF` writer - TXMSGDISCCF"]
pub type TXMSGDISCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXMSGSENTCF` reader - TXMSGSENTCF"]
pub type TXMSGSENTCF_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGSENTCF` writer - TXMSGSENTCF"]
pub type TXMSGSENTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXMSGABTCF` reader - TXMSGABTCF"]
pub type TXMSGABTCF_R = crate::BitReader<bool>;
#[doc = "Field `TXMSGABTCF` writer - TXMSGABTCF"]
pub type TXMSGABTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `HRSTDISCCF` reader - HRSTDISCCF"]
pub type HRSTDISCCF_R = crate::BitReader<bool>;
#[doc = "Field `HRSTDISCCF` writer - HRSTDISCCF"]
pub type HRSTDISCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `HRSTSENTCF` reader - HRSTSENTCF"]
pub type HRSTSENTCF_R = crate::BitReader<bool>;
#[doc = "Field `HRSTSENTCF` writer - HRSTSENTCF"]
pub type HRSTSENTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXUNDCF` reader - TXUNDCF"]
pub type TXUNDCF_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDCF` writer - TXUNDCF"]
pub type TXUNDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXORDDETCF` reader - RXORDDETCF"]
pub type RXORDDETCF_R = crate::BitReader<bool>;
#[doc = "Field `RXORDDETCF` writer - RXORDDETCF"]
pub type RXORDDETCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXHRSTDETCF` reader - RXHRSTDETCF"]
pub type RXHRSTDETCF_R = crate::BitReader<bool>;
#[doc = "Field `RXHRSTDETCF` writer - RXHRSTDETCF"]
pub type RXHRSTDETCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXOVRCF` reader - RXOVRCF"]
pub type RXOVRCF_R = crate::BitReader<bool>;
#[doc = "Field `RXOVRCF` writer - RXOVRCF"]
pub type RXOVRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RXMSGENDCF` reader - RXMSGENDCF"]
pub type RXMSGENDCF_R = crate::BitReader<bool>;
#[doc = "Field `RXMSGENDCF` writer - RXMSGENDCF"]
pub type RXMSGENDCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TYPECEVT1CF` reader - TYPECEVT1CF"]
pub type TYPECEVT1CF_R = crate::BitReader<bool>;
#[doc = "Field `TYPECEVT1CF` writer - TYPECEVT1CF"]
pub type TYPECEVT1CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TYPECEVT2CF` reader - TYPECEVT2CF"]
pub type TYPECEVT2CF_R = crate::BitReader<bool>;
#[doc = "Field `TYPECEVT2CF` writer - TYPECEVT2CF"]
pub type TYPECEVT2CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FRSEVTCF` reader - FRSEVTCF"]
pub type FRSEVTCF_R = crate::BitReader<bool>;
#[doc = "Field `FRSEVTCF` writer - FRSEVTCF"]
pub type FRSEVTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&self) -> TXMSGDISCCF_R {
        TXMSGDISCCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&self) -> TXMSGSENTCF_R {
        TXMSGSENTCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&self) -> TXMSGABTCF_R {
        TXMSGABTCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&self) -> HRSTDISCCF_R {
        HRSTDISCCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&self) -> HRSTSENTCF_R {
        HRSTSENTCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&self) -> TXUNDCF_R {
        TXUNDCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&self) -> RXORDDETCF_R {
        RXORDDETCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&self) -> RXHRSTDETCF_R {
        RXHRSTDETCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&self) -> RXOVRCF_R {
        RXOVRCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&self) -> RXMSGENDCF_R {
        RXMSGENDCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&self) -> TYPECEVT1CF_R {
        TYPECEVT1CF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&self) -> TYPECEVT2CF_R {
        TYPECEVT2CF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&self) -> FRSEVTCF_R {
        FRSEVTCF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W<1> {
        TXMSGDISCCF_W::new(self)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W<2> {
        TXMSGSENTCF_W::new(self)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    #[must_use]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W<3> {
        TXMSGABTCF_W::new(self)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    #[must_use]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W<4> {
        HRSTDISCCF_W::new(self)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    #[must_use]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W<5> {
        HRSTSENTCF_W::new(self)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    #[must_use]
    pub fn txundcf(&mut self) -> TXUNDCF_W<6> {
        TXUNDCF_W::new(self)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W<9> {
        RXORDDETCF_W::new(self)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W<10> {
        RXHRSTDETCF_W::new(self)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W<11> {
        RXOVRCF_W::new(self)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    #[must_use]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W<12> {
        RXMSGENDCF_W::new(self)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W<14> {
        TYPECEVT1CF_W::new(self)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    #[must_use]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W<15> {
        TYPECEVT2CF_W::new(self)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    #[must_use]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W<20> {
        FRSEVTCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
