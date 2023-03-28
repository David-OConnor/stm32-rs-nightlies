#[doc = "Register `DMACCARxDR` reader"]
pub struct R(crate::R<DMACCARX_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARX_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARX_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARX_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACCARxDR` writer"]
pub struct W(crate::W<DMACCARX_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCARX_DR_SPEC>;
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
impl From<crate::W<DMACCARX_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCARX_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURRDESAPTR` writer - Application Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACCARX_DR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn currdesaptr(&mut self) -> CURRDESAPTR_W<0> {
        CURRDESAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current application receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccarx_dr](index.html) module"]
pub struct DMACCARX_DR_SPEC;
impl crate::RegisterSpec for DMACCARX_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccarx_dr::R](R) reader structure"]
impl crate::Readable for DMACCARX_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaccarx_dr::W](W) writer structure"]
impl crate::Writable for DMACCARX_DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACCARxDR to value 0"]
impl crate::Resettable for DMACCARX_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
