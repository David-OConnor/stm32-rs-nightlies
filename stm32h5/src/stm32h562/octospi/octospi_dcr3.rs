#[doc = "Register `OCTOSPI_DCR3` reader"]
pub struct R(crate::R<OCTOSPI_DCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTOSPI_DCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTOSPI_DCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTOSPI_DCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTOSPI_DCR3` writer"]
pub struct W(crate::W<OCTOSPI_DCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTOSPI_DCR3_SPEC>;
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
impl From<crate::W<OCTOSPI_DCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTOSPI_DCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSBOUND` reader - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
pub type CSBOUND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSBOUND` writer - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
pub type CSBOUND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCTOSPI_DCR3_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 16:20 - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
    #[inline(always)]
    pub fn csbound(&self) -> CSBOUND_R {
        CSBOUND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - NCS boundary This field enables the transaction boundary feature. When active, a minimum value of 3 is recommended. The NCS is released on each boundary of 2CSBOUND bytes. others: NCS boundary set to 2CSBOUND bytes"]
    #[inline(always)]
    #[must_use]
    pub fn csbound(&mut self) -> CSBOUND_W<16> {
        CSBOUND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OCTOSPI device configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octospi_dcr3](index.html) module"]
pub struct OCTOSPI_DCR3_SPEC;
impl crate::RegisterSpec for OCTOSPI_DCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octospi_dcr3::R](R) reader structure"]
impl crate::Readable for OCTOSPI_DCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octospi_dcr3::W](W) writer structure"]
impl crate::Writable for OCTOSPI_DCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTOSPI_DCR3 to value 0"]
impl crate::Resettable for OCTOSPI_DCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
