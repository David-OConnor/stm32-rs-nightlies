#[doc = "Register `ALRM%sSSR` reader"]
pub struct R(crate::R<ALRMSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRM%sSSR` writer"]
pub struct W(crate::W<ALRMSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMSSR_SPEC>;
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
impl From<crate::W<ALRMSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SS` reader - SS"]
pub type SS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SS` writer - SS"]
pub type SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMSSR_SPEC, u16, u16, 15, O>;
#[doc = "Field `MASKSS` reader - MASKSS"]
pub type MASKSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASKSS` writer - MASKSS"]
pub type MASKSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMSSR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:14 - SS"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - MASKSS"]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - SS"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<0> {
        SS_W::new(self)
    }
    #[doc = "Bits 24:27 - MASKSS"]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<24> {
        MASKSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm sub-second register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmssr](index.html) module"]
pub struct ALRMSSR_SPEC;
impl crate::RegisterSpec for ALRMSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrmssr::R](R) reader structure"]
impl crate::Readable for ALRMSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrmssr::W](W) writer structure"]
impl crate::Writable for ALRMSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRM%sSSR to value 0"]
impl crate::Resettable for ALRMSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
