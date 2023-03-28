#[doc = "Register `BMCMPR6` reader"]
pub struct R(crate::R<BMCMPR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCMPR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCMPR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCMPR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMCMPR6` writer"]
pub struct W(crate::W<BMCMPR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCMPR6_SPEC>;
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
impl From<crate::W<BMCMPR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCMPR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMCMP` reader - BMCMP"]
pub type BMCMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BMCMP` writer - BMCMP"]
pub type BMCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCMPR6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    #[must_use]
    pub fn bmcmp(&mut self) -> BMCMP_W<0> {
        BMCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMCMPR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcmpr6](index.html) module"]
pub struct BMCMPR6_SPEC;
impl crate::RegisterSpec for BMCMPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmcmpr6::R](R) reader structure"]
impl crate::Readable for BMCMPR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmcmpr6::W](W) writer structure"]
impl crate::Writable for BMCMPR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMCMPR6 to value 0"]
impl crate::Resettable for BMCMPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
