#[doc = "Register `DCMI_CWSTRT` reader"]
pub struct R(crate::R<DCMI_CWSTRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_CWSTRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_CWSTRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_CWSTRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_CWSTRT` writer"]
pub struct W(crate::W<DCMI_CWSTRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_CWSTRT_SPEC>;
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
impl From<crate::W<DCMI_CWSTRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_CWSTRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOFFCNT` reader - HOFFCNT"]
pub type HOFFCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HOFFCNT` writer - HOFFCNT"]
pub type HOFFCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCMI_CWSTRT_SPEC, u16, u16, 14, O>;
#[doc = "Field `VST` reader - VST"]
pub type VST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VST` writer - VST"]
pub type VST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCMI_CWSTRT_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:13 - HOFFCNT"]
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - VST"]
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - HOFFCNT"]
    #[inline(always)]
    #[must_use]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W<0> {
        HOFFCNT_W::new(self)
    }
    #[doc = "Bits 16:28 - VST"]
    #[inline(always)]
    #[must_use]
    pub fn vst(&mut self) -> VST_W<16> {
        VST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCMI crop window start\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_cwstrt](index.html) module"]
pub struct DCMI_CWSTRT_SPEC;
impl crate::RegisterSpec for DCMI_CWSTRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_cwstrt::R](R) reader structure"]
impl crate::Readable for DCMI_CWSTRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_cwstrt::W](W) writer structure"]
impl crate::Writable for DCMI_CWSTRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCMI_CWSTRT to value 0"]
impl crate::Resettable for DCMI_CWSTRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
