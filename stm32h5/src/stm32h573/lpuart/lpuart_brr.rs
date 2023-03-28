#[doc = "Register `LPUART_BRR` reader"]
pub struct R(crate::R<LPUART_BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPUART_BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPUART_BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPUART_BRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPUART_BRR` writer"]
pub struct W(crate::W<LPUART_BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPUART_BRR_SPEC>;
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
impl From<crate::W<LPUART_BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPUART_BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRR` reader - LPUART baud rate division (LPUARTDIV)"]
pub type BRR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BRR` writer - LPUART baud rate division (LPUARTDIV)"]
pub type BRR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPUART_BRR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - LPUART baud rate division (LPUARTDIV)"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - LPUART baud rate division (LPUARTDIV)"]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BRR_W<0> {
        BRR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART baud rate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpuart_brr](index.html) module"]
pub struct LPUART_BRR_SPEC;
impl crate::RegisterSpec for LPUART_BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpuart_brr::R](R) reader structure"]
impl crate::Readable for LPUART_BRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpuart_brr::W](W) writer structure"]
impl crate::Writable for LPUART_BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPUART_BRR to value 0"]
impl crate::Resettable for LPUART_BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
