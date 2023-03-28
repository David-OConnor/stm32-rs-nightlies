#[doc = "Register `PCROP1BSR` reader"]
pub struct R(crate::R<PCROP1BSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1BSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1BSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1BSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP1BSR` writer"]
pub struct W(crate::W<PCROP1BSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1BSR_SPEC>;
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
impl From<crate::W<PCROP1BSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1BSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1B_STRT` reader - PCROP1B area start offset"]
pub type PCROP1B_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCROP1B_STRT` writer - PCROP1B area start offset"]
pub type PCROP1B_STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP1BSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PCROP1B area start offset"]
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCROP1B area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1b_strt(&mut self) -> PCROP1B_STRT_W<0> {
        PCROP1B_STRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash PCROP zone B Start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1bsr](index.html) module"]
pub struct PCROP1BSR_SPEC;
impl crate::RegisterSpec for PCROP1BSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1bsr::R](R) reader structure"]
impl crate::Readable for PCROP1BSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop1bsr::W](W) writer structure"]
impl crate::Writable for PCROP1BSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCROP1BSR to value 0xffff_ffff"]
impl crate::Resettable for PCROP1BSR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
