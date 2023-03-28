#[doc = "Register `GICV_ABPR` reader"]
pub struct R(crate::R<GICV_ABPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_ABPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_ABPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_ABPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICV_ABPR` writer"]
pub struct W(crate::W<GICV_ABPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICV_ABPR_SPEC>;
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
impl From<crate::W<GICV_ABPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICV_ABPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BINARY_POINT` reader - BINARY_POINT"]
pub type BINARY_POINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BINARY_POINT` writer - BINARY_POINT"]
pub type BINARY_POINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICV_ABPR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    #[must_use]
    pub fn binary_point(&mut self) -> BINARY_POINT_W<0> {
        BINARY_POINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICV VM aliased binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_abpr](index.html) module"]
pub struct GICV_ABPR_SPEC;
impl crate::RegisterSpec for GICV_ABPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_abpr::R](R) reader structure"]
impl crate::Readable for GICV_ABPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicv_abpr::W](W) writer structure"]
impl crate::Writable for GICV_ABPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICV_ABPR to value 0x03"]
impl crate::Resettable for GICV_ABPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
