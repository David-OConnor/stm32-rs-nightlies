#[doc = "Register `WRP1AR` reader"]
pub struct R(crate::R<WRP1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP1AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRP1AR` writer"]
pub struct W(crate::W<WRP1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP1AR_SPEC>;
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
impl From<crate::W<WRP1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP1AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP1A_STRT` reader - Bank 1 WRP first area start offset"]
pub type WRP1A_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRP1A_STRT` writer - Bank 1 WRP first area start offset"]
pub type WRP1A_STRT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WRP1AR_SPEC, u8, u8, 7, O>;
#[doc = "Field `WRP1A_END` reader - Bank 1 WRP first area A end offset"]
pub type WRP1A_END_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRP1A_END` writer - Bank 1 WRP first area A end offset"]
pub type WRP1A_END_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WRP1AR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    pub fn wrp1a_strt(&self) -> WRP1A_STRT_R {
        WRP1A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    pub fn wrp1a_end(&self) -> WRP1A_END_R {
        WRP1A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bank 1 WRP first area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_strt(&mut self) -> WRP1A_STRT_W<0> {
        WRP1A_STRT_W::new(self)
    }
    #[doc = "Bits 16:22 - Bank 1 WRP first area A end offset"]
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_end(&mut self) -> WRP1A_END_W<16> {
        WRP1A_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash WRP area A address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrp1ar](index.html) module"]
pub struct WRP1AR_SPEC;
impl crate::RegisterSpec for WRP1AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrp1ar::R](R) reader structure"]
impl crate::Readable for WRP1AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrp1ar::W](W) writer structure"]
impl crate::Writable for WRP1AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRP1AR to value 0xff80_ffff"]
impl crate::Resettable for WRP1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff80_ffff;
}
