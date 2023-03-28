#[doc = "Register `IER1` reader"]
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER1` writer"]
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOHSTXIE` reader - Timeout high"]
pub type TOHSTXIE_R = crate::BitReader<bool>;
#[doc = "Field `TOHSTXIE` writer - Timeout high"]
pub type TOHSTXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TOLPRXIE` reader - Timeout low"]
pub type TOLPRXIE_R = crate::BitReader<bool>;
#[doc = "Field `TOLPRXIE` writer - Timeout low"]
pub type TOLPRXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `ECCSEIE` reader - ECC single"]
pub type ECCSEIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCSEIE` writer - ECC single"]
pub type ECCSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `ECCMEIE` reader - ECC multi"]
pub type ECCMEIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCMEIE` writer - ECC multi"]
pub type ECCMEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `CRCEIE` reader - CRC error interrupt enable"]
pub type CRCEIE_R = crate::BitReader<bool>;
#[doc = "Field `CRCEIE` writer - CRC error interrupt enable"]
pub type CRCEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `PSEIE` reader - Packet size error interrupt enable"]
pub type PSEIE_R = crate::BitReader<bool>;
#[doc = "Field `PSEIE` writer - Packet size error interrupt enable"]
pub type PSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `EOTPEIE` reader - EoTp error interrupt enable"]
pub type EOTPEIE_R = crate::BitReader<bool>;
#[doc = "Field `EOTPEIE` writer - EoTp error interrupt enable"]
pub type EOTPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `LPWREIE` reader - LTDC payload write error interrupt enable"]
pub type LPWREIE_R = crate::BitReader<bool>;
#[doc = "Field `LPWREIE` writer - LTDC payload write error interrupt enable"]
pub type LPWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GCWREIE` reader - Generic command write error interrupt enable"]
pub type GCWREIE_R = crate::BitReader<bool>;
#[doc = "Field `GCWREIE` writer - Generic command write error interrupt enable"]
pub type GCWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GPWREIE` reader - Generic payload write error interrupt enable"]
pub type GPWREIE_R = crate::BitReader<bool>;
#[doc = "Field `GPWREIE` writer - Generic payload write error interrupt enable"]
pub type GPWREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GPTXEIE` reader - Generic payload transmit error interrupt enable"]
pub type GPTXEIE_R = crate::BitReader<bool>;
#[doc = "Field `GPTXEIE` writer - Generic payload transmit error interrupt enable"]
pub type GPTXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GPRDEIE` reader - Generic payload read error interrupt enable"]
pub type GPRDEIE_R = crate::BitReader<bool>;
#[doc = "Field `GPRDEIE` writer - Generic payload read error interrupt enable"]
pub type GPRDEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `GPRXEIE` reader - Generic payload receive error interrupt enable"]
pub type GPRXEIE_R = crate::BitReader<bool>;
#[doc = "Field `GPRXEIE` writer - Generic payload receive error interrupt enable"]
pub type GPRXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error interrupt enable"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Packet size error interrupt enable"]
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EoTp error interrupt enable"]
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LTDC payload write error interrupt enable"]
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic command write error interrupt enable"]
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic payload write error interrupt enable"]
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic payload transmit error interrupt enable"]
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generic payload read error interrupt enable"]
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Generic payload receive error interrupt enable"]
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    #[must_use]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W<0> {
        TOHSTXIE_W::new(self)
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    #[must_use]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W<1> {
        TOLPRXIE_W::new(self)
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    #[must_use]
    pub fn eccseie(&mut self) -> ECCSEIE_W<2> {
        ECCSEIE_W::new(self)
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    #[must_use]
    pub fn eccmeie(&mut self) -> ECCMEIE_W<3> {
        ECCMEIE_W::new(self)
    }
    #[doc = "Bit 4 - CRC error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crceie(&mut self) -> CRCEIE_W<4> {
        CRCEIE_W::new(self)
    }
    #[doc = "Bit 5 - Packet size error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pseie(&mut self) -> PSEIE_W<5> {
        PSEIE_W::new(self)
    }
    #[doc = "Bit 6 - EoTp error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eotpeie(&mut self) -> EOTPEIE_W<6> {
        EOTPEIE_W::new(self)
    }
    #[doc = "Bit 7 - LTDC payload write error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpwreie(&mut self) -> LPWREIE_W<7> {
        LPWREIE_W::new(self)
    }
    #[doc = "Bit 8 - Generic command write error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcwreie(&mut self) -> GCWREIE_W<8> {
        GCWREIE_W::new(self)
    }
    #[doc = "Bit 9 - Generic payload write error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwreie(&mut self) -> GPWREIE_W<9> {
        GPWREIE_W::new(self)
    }
    #[doc = "Bit 10 - Generic payload transmit error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gptxeie(&mut self) -> GPTXEIE_W<10> {
        GPTXEIE_W::new(self)
    }
    #[doc = "Bit 11 - Generic payload read error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gprdeie(&mut self) -> GPRDEIE_W<11> {
        GPRDEIE_W::new(self)
    }
    #[doc = "Bit 12 - Generic payload receive error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn gprxeie(&mut self) -> GPRXEIE_W<12> {
        GPRXEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](index.html) module"]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier1::R](R) reader structure"]
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier1::W](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER1 to value 0"]
impl crate::Resettable for IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
