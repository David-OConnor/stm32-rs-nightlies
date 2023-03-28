#[doc = "Register `SAES_IER` reader"]
pub struct R(crate::R<SAES_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAES_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAES_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAES_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAES_IER` writer"]
pub struct W(crate::W<SAES_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAES_IER_SPEC>;
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
impl From<crate::W<SAES_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAES_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCFIE` reader - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set."]
pub type CCFIE_R = crate::BitReader<bool>;
#[doc = "Field `CCFIE` writer - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set."]
pub type CCFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAES_IER_SPEC, bool, O>;
#[doc = "Field `RWEIE` reader - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set."]
pub type RWEIE_R = crate::BitReader<bool>;
#[doc = "Field `RWEIE` writer - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set."]
pub type RWEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAES_IER_SPEC, bool, O>;
#[doc = "Field `KEIE` reader - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set."]
pub type KEIE_R = crate::BitReader<bool>;
#[doc = "Field `KEIE` writer - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set."]
pub type KEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAES_IER_SPEC, bool, O>;
#[doc = "Field `RNGEIE` reader - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set."]
pub type RNGEIE_R = crate::BitReader<bool>;
#[doc = "Field `RNGEIE` writer - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set."]
pub type RNGEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAES_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set."]
    #[inline(always)]
    pub fn ccfie(&self) -> CCFIE_R {
        CCFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set."]
    #[inline(always)]
    pub fn rweie(&self) -> RWEIE_R {
        RWEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set."]
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set."]
    #[inline(always)]
    pub fn rngeie(&self) -> RNGEIE_R {
        RNGEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Computation complete flag interrupt enable This bit enables or disables (masks) the SAES interrupt generation when CCF (computation complete flag) is set."]
    #[inline(always)]
    #[must_use]
    pub fn ccfie(&mut self) -> CCFIE_W<0> {
        CCFIE_W::new(self)
    }
    #[doc = "Bit 1 - Read or write error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RWEIF (read and/or write error flag) is set."]
    #[inline(always)]
    #[must_use]
    pub fn rweie(&mut self) -> RWEIE_W<1> {
        RWEIE_W::new(self)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when KEIF (key error flag) is set."]
    #[inline(always)]
    #[must_use]
    pub fn keie(&mut self) -> KEIE_W<2> {
        KEIE_W::new(self)
    }
    #[doc = "Bit 3 - RNG error interrupt enable This bit enables or disables (masks) the SAES interrupt generation when RNGEIF (RNG error flag) is set."]
    #[inline(always)]
    #[must_use]
    pub fn rngeie(&mut self) -> RNGEIE_W<3> {
        RNGEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAES interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saes_ier](index.html) module"]
pub struct SAES_IER_SPEC;
impl crate::RegisterSpec for SAES_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saes_ier::R](R) reader structure"]
impl crate::Readable for SAES_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saes_ier::W](W) writer structure"]
impl crate::Writable for SAES_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAES_IER to value 0"]
impl crate::Resettable for SAES_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
