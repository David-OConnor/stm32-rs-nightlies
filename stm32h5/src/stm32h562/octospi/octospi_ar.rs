#[doc = "Register `OCTOSPI_AR` reader"]
pub struct R(crate::R<OCTOSPI_AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_AR` writer"]
pub struct W(crate::W<OCTOSPI_AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_AR_SPEC>;
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
impl From<crate::W<OCTOSPI_AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - Address Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 1. Writes to this field are ignored when BUSY = 1 or when FMODE = 11 (Memory-mapped mode)."]
pub type ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS` writer - Address Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 1. Writes to this field are ignored when BUSY = 1 or when FMODE = 11 (Memory-mapped mode)."]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_AR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 1. Writes to this field are ignored when BUSY = 1 or when FMODE = 11 (Memory-mapped mode)."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address Address to be sent to the external device. In HyperBus protocol, this field must be even as this protocol is 16-bit word oriented. In dual-memory configuration, AR\\[0\\]
is forced to 1. Writes to this field are ignored when BUSY = 1 or when FMODE = 11 (Memory-mapped mode)."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_ar](index.html) module"]
pub struct OCTOSPI_AR_SPEC;
impl crate::RegisterSpec for OCTOSPI_AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_ar::R](R) reader structure"]
impl crate::Readable for OCTOSPI_AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_ar::W](W) writer structure"]
impl crate::Writable for OCTOSPI_AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_AR to value 0"]
impl crate::Resettable for OCTOSPI_AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
