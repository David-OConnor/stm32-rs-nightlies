#[doc = "Register `LCCCR` reader"]
pub struct R(crate::R<LCCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCCCR` writer"]
pub struct W(crate::W<LCCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCCCR_SPEC>;
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
impl From<crate::W<LCCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLC` reader - Color coding"]
pub type COLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COLC` writer - Color coding"]
pub type COLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCCCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPE` reader - Loosely packed enable"]
pub type LPE_R = crate::BitReader<bool>;
#[doc = "Field `LPE` writer - Loosely packed enable"]
pub type LPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Color coding"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Loosely packed enable"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Color coding"]
    #[inline(always)]
    #[must_use]
    pub fn colc(&mut self) -> COLC_W<0> {
        COLC_W::new(self)
    }
    #[doc = "Bit 8 - Loosely packed enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpe(&mut self) -> LPE_W<8> {
        LPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host LTDC current color coding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcccr](index.html) module"]
pub struct LCCCR_SPEC;
impl crate::RegisterSpec for LCCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcccr::R](R) reader structure"]
impl crate::Readable for LCCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcccr::W](W) writer structure"]
impl crate::Writable for LCCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCCCR to value 0"]
impl crate::Resettable for LCCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
