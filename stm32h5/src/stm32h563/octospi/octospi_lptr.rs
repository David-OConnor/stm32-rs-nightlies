#[doc = "Register `OCTOSPI_LPTR` reader"]
pub struct R(crate::R<OCTOSPI_LPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_LPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_LPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_LPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_LPTR` writer"]
pub struct W(crate::W<OCTOSPI_LPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_LPTR_SPEC>;
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
impl From<crate::W<OCTOSPI_LPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_LPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the OCTOSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the OCTOSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
pub type TIMEOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEOUT` writer - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the OCTOSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the OCTOSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OCTOSPI_LPTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the OCTOSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the OCTOSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15: 0\\]: Timeout period After each access in Memory-mapped mode, the OCTOSPI prefetches the subsequent bytes and hold them in the FIFO. This field indicates how many CLK cycles the OCTOSPI waits after the clock becomes inactive and until it raises the NCS, putting the external device in a lower-consumption state."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI low-power timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_lptr](index.html) module"]
pub struct OCTOSPI_LPTR_SPEC;
impl crate::RegisterSpec for OCTOSPI_LPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_lptr::R](R) reader structure"]
impl crate::Readable for OCTOSPI_LPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_lptr::W](W) writer structure"]
impl crate::Writable for OCTOSPI_LPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_LPTR to value 0"]
impl crate::Resettable for OCTOSPI_LPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
