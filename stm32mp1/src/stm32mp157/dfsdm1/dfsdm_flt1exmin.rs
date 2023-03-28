#[doc = "Register `DFSDM_FLT1EXMIN` reader"]
pub struct R(crate::R<DFSDM_FLT1EXMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1EXMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1EXMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1EXMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_FLT1EXMIN` writer"]
pub struct W(crate::W<DFSDM_FLT1EXMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT1EXMIN_SPEC>;
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
impl From<crate::W<DFSDM_FLT1EXMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT1EXMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXMINCH` reader - EXMINCH"]
pub type EXMINCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXMIN` reader - EXMIN"]
pub type EXMIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXMIN` writer - EXMIN"]
pub type EXMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_FLT1EXMIN_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:2 - EXMINCH"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    #[must_use]
    pub fn exmin(&mut self) -> EXMIN_W<8> {
        EXMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM filter 1 extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1exmin](index.html) module"]
pub struct DFSDM_FLT1EXMIN_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1EXMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt1exmin::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT1EXMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1exmin::W](W) writer structure"]
impl crate::Writable for DFSDM_FLT1EXMIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT1EXMIN to value 0x7fff_ff00"]
impl crate::Resettable for DFSDM_FLT1EXMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_ff00;
}
