#[doc = "Register `TTGTP` reader"]
pub struct R(crate::R<TTGTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTGTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTGTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTGTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTGTP` writer"]
pub struct W(crate::W<TTGTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTGTP_SPEC>;
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
impl From<crate::W<TTGTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTGTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCL` reader - Time Preset"]
pub type NCL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NCL` writer - Time Preset"]
pub type NCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTGTP_SPEC, u16, u16, 16, O>;
#[doc = "Field `CTP` reader - Cycle Time Target Phase"]
pub type CTP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTP` writer - Cycle Time Target Phase"]
pub type CTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTGTP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    #[must_use]
    pub fn ncl(&mut self) -> NCL_W<0> {
        NCL_W::new(self)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CTP_W<16> {
        CTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Global Time Preset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttgtp](index.html) module"]
pub struct TTGTP_SPEC;
impl crate::RegisterSpec for TTGTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttgtp::R](R) reader structure"]
impl crate::Readable for TTGTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttgtp::W](W) writer structure"]
impl crate::Writable for TTGTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTGTP to value 0"]
impl crate::Resettable for TTGTP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
