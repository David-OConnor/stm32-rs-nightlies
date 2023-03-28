#[doc = "Register `WRP2AR` reader"]
pub struct R(crate::R<WRP2AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP2AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP2AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP2AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRP2AR` writer"]
pub struct W(crate::W<WRP2AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP2AR_SPEC>;
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
impl From<crate::W<WRP2AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP2AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP2A_PSTRT` reader - WRP2A_PSTRT"]
pub type WRP2A_PSTRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRP2A_PSTRT` writer - WRP2A_PSTRT"]
pub type WRP2A_PSTRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP2AR_SPEC, u8, u8, 7, O>;
#[doc = "Field `WRP2A_PEND` reader - WRP2A_PEND"]
pub type WRP2A_PEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRP2A_PEND` writer - WRP2A_PEND"]
pub type WRP2A_PEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRP2AR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - WRP2A_PSTRT"]
    #[inline(always)]
    pub fn wrp2a_pstrt(&self) -> WRP2A_PSTRT_R {
        WRP2A_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2A_PEND"]
    #[inline(always)]
    pub fn wrp2a_pend(&self) -> WRP2A_PEND_R {
        WRP2A_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2A_PSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_pstrt(&mut self) -> WRP2A_PSTRT_W<0> {
        WRP2A_PSTRT_W::new(self)
    }
    #[doc = "Bits 16:22 - WRP2A_PEND"]
    #[inline(always)]
    #[must_use]
    pub fn wrp2a_pend(&mut self) -> WRP2A_PEND_W<16> {
        WRP2A_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash WPR2 area A address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp2ar](index.html) module"]
pub struct WRP2AR_SPEC;
impl crate::RegisterSpec for WRP2AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp2ar::R](R) reader structure"]
impl crate::Readable for WRP2AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrp2ar::W](W) writer structure"]
impl crate::Writable for WRP2AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRP2AR to value 0xff00_ff00"]
impl crate::Resettable for WRP2AR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff00;
}
