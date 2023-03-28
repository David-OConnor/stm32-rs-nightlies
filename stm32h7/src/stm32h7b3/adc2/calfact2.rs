#[doc = "Register `CALFACT2` reader"]
pub struct R(crate::R<CALFACT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFACT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFACT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFACT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALFACT2` writer"]
pub struct W(crate::W<CALFACT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFACT2_SPEC>;
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
impl From<crate::W<CALFACT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFACT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINCALFACT` reader - Linearity Calibration Factor"]
pub type LINCALFACT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LINCALFACT` writer - Linearity Calibration Factor"]
pub type LINCALFACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CALFACT2_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&self) -> LINCALFACT_R {
        LINCALFACT_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    #[must_use]
    pub fn lincalfact(&mut self) -> LINCALFACT_W<0> {
        LINCALFACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Calibration Factor register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfact2](index.html) module"]
pub struct CALFACT2_SPEC;
impl crate::RegisterSpec for CALFACT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calfact2::R](R) reader structure"]
impl crate::Readable for CALFACT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calfact2::W](W) writer structure"]
impl crate::Writable for CALFACT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALFACT2 to value 0"]
impl crate::Resettable for CALFACT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
