#[doc = "Register `WRP1BR` reader"]
pub struct R(crate::R<WRP1BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP1BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP1BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP1BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRP1BR` writer"]
pub struct W(crate::W<WRP1BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP1BR_SPEC>;
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
impl From<crate::W<WRP1BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP1BR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP1B_PSTRT` reader - WRP1B_PSTRT"]
pub type WRP1B_PSTRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRP1B_PSTRT` writer - WRP1B_PSTRT"]
pub type WRP1B_PSTRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP1BR_SPEC, u8, u8, 7, O>;
#[doc = "Field `WRP1B_PEND` reader - WRP1B_PEND"]
pub type WRP1B_PEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRP1B_PEND` writer - WRP1B_PEND"]
pub type WRP1B_PEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP1BR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - WRP1B_PSTRT"]
    #[inline(always)]
    pub fn wrp1b_pstrt(&self) -> WRP1B_PSTRT_R {
        WRP1B_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP1B_PEND"]
    #[inline(always)]
    pub fn wrp1b_pend(&self) -> WRP1B_PEND_R {
        WRP1B_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP1B_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_pstrt(&mut self) -> WRP1B_PSTRT_W<0> {
        WRP1B_PSTRT_W::new(self)
    }
    #[doc = "Bits 16:22 - WRP1B_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1b_pend(&mut self) -> WRP1B_PEND_W<16> {
        WRP1B_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 WRP area B address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1br](index.html) module"]
pub struct WRP1BR_SPEC;
impl crate::RegisterSpec for WRP1BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp1br::R](R) reader structure"]
impl crate::Readable for WRP1BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrp1br::W](W) writer structure"]
impl crate::Writable for WRP1BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRP1BR to value 0xff00_ff00"]
impl crate::Resettable for WRP1BR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff00;
}
