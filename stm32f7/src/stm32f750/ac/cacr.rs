#[doc = "Register `CACR` reader"]
pub struct R(crate::R<CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACR` writer"]
pub struct W(crate::W<CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACR_SPEC>;
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
impl From<crate::W<CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIWT` reader - SIWT"]
pub type SIWT_R = crate::BitReader<bool>;
#[doc = "Field `SIWT` writer - SIWT"]
pub type SIWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACR_SPEC, bool, O>;
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACR_SPEC, bool, O>;
#[doc = "Field `FORCEWT` reader - FORCEWT"]
pub type FORCEWT_R = crate::BitReader<bool>;
#[doc = "Field `FORCEWT` writer - FORCEWT"]
pub type FORCEWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SIWT"]
    #[inline(always)]
    pub fn siwt(&self) -> SIWT_R {
        SIWT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FORCEWT"]
    #[inline(always)]
    pub fn forcewt(&self) -> FORCEWT_R {
        FORCEWT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SIWT"]
    #[inline(always)]
    #[must_use]
    pub fn siwt(&mut self) -> SIWT_W<0> {
        SIWT_W::new(self)
    }
    #[doc = "Bit 1 - ECCEN"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<1> {
        ECCEN_W::new(self)
    }
    #[doc = "Bit 2 - FORCEWT"]
    #[inline(always)]
    #[must_use]
    pub fn forcewt(&mut self) -> FORCEWT_W<2> {
        FORCEWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Cache Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](index.html) module"]
pub struct CACR_SPEC;
impl crate::RegisterSpec for CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacr::R](R) reader structure"]
impl crate::Readable for CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacr::W](W) writer structure"]
impl crate::Writable for CACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR to value 0"]
impl crate::Resettable for CACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
