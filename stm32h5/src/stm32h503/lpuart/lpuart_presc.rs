#[doc = "Register `LPUART_PRESC` reader"]
pub struct R(crate::R<LPUART_PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPUART_PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPUART_PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPUART_PRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPUART_PRESC` writer"]
pub struct W(crate::W<LPUART_PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPUART_PRESC_SPEC>;
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
impl From<crate::W<LPUART_PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPUART_PRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALER` writer - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPUART_PRESC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is equal to 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpuart_presc](index.html) module"]
pub struct LPUART_PRESC_SPEC;
impl crate::RegisterSpec for LPUART_PRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpuart_presc::R](R) reader structure"]
impl crate::Readable for LPUART_PRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpuart_presc::W](W) writer structure"]
impl crate::Writable for LPUART_PRESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPUART_PRESC to value 0"]
impl crate::Resettable for LPUART_PRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
