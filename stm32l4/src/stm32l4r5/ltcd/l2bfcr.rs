#[doc = "Register `L2BFCR` reader"]
pub struct R(crate::R<L2BFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2BFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2BFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2BFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L2BFCR` writer"]
pub struct W(crate::W<L2BFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2BFCR_SPEC>;
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
impl From<crate::W<L2BFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2BFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BF2` reader - Blending Factor 2"]
pub type BF2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BF2` writer - Blending Factor 2"]
pub type BF2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2BFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BF1` reader - Blending Factor 1"]
pub type BF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BF1` writer - Blending Factor 1"]
pub type BF1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2BFCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    #[must_use]
    pub fn bf2(&mut self) -> BF2_W<0> {
        BF2_W::new(self)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    #[must_use]
    pub fn bf1(&mut self) -> BF1_W<8> {
        BF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Blending Factors Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2bfcr](index.html) module"]
pub struct L2BFCR_SPEC;
impl crate::RegisterSpec for L2BFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2bfcr::R](R) reader structure"]
impl crate::Readable for L2BFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2bfcr::W](W) writer structure"]
impl crate::Writable for L2BFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2BFCR to value 0"]
impl crate::Resettable for L2BFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
