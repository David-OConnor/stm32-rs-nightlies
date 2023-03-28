#[doc = "Register `RAMCFG_M3ICR` reader"]
pub struct R(crate::R<RAMCFG_M3ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCFG_M3ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCFG_M3ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCFG_M3ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMCFG_M3ICR` writer"]
pub struct W(crate::W<RAMCFG_M3ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMCFG_M3ICR_SPEC>;
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
impl From<crate::W<RAMCFG_M3ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMCFG_M3ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSEDC` reader - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
pub type CSEDC_R = crate::BitReader<bool>;
#[doc = "Field `CSEDC` writer - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
pub type CSEDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M3ICR_SPEC, bool, O>;
#[doc = "Field `CDED` reader - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
pub type CDED_R = crate::BitReader<bool>;
#[doc = "Field `CDED` writer - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
pub type CDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M3ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
    #[inline(always)]
    pub fn csedc(&self) -> CSEDC_R {
        CSEDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
    #[inline(always)]
    pub fn cded(&self) -> CDED_R {
        CDED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
    #[inline(always)]
    #[must_use]
    pub fn csedc(&mut self) -> CSEDC_W<0> {
        CSEDC_W::new(self)
    }
    #[doc = "Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
    #[inline(always)]
    #[must_use]
    pub fn cded(&mut self) -> CDED_W<1> {
        CDED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMCFG memory 3 interrupt clear register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg_m3icr](index.html) module"]
pub struct RAMCFG_M3ICR_SPEC;
impl crate::RegisterSpec for RAMCFG_M3ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramcfg_m3icr::R](R) reader structure"]
impl crate::Readable for RAMCFG_M3ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramcfg_m3icr::W](W) writer structure"]
impl crate::Writable for RAMCFG_M3ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMCFG_M3ICR to value 0"]
impl crate::Resettable for RAMCFG_M3ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
