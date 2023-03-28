#[doc = "Register `RAMCFG_M3IER` reader"]
pub struct R(crate::R<RAMCFG_M3IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCFG_M3IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCFG_M3IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCFG_M3IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMCFG_M3IER` writer"]
pub struct W(crate::W<RAMCFG_M3IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMCFG_M3IER_SPEC>;
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
impl From<crate::W<RAMCFG_M3IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMCFG_M3IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEIE` reader - ECC single error interrupt enable"]
pub type SEIE_R = crate::BitReader<bool>;
#[doc = "Field `SEIE` writer - ECC single error interrupt enable"]
pub type SEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M3IER_SPEC, bool, O>;
#[doc = "Field `DEIE` reader - ECC double error interrupt enable"]
pub type DEIE_R = crate::BitReader<bool>;
#[doc = "Field `DEIE` writer - ECC double error interrupt enable"]
pub type DEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M3IER_SPEC, bool, O>;
#[doc = "Field `ECCNMI` reader - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
pub type ECCNMI_R = crate::BitReader<bool>;
#[doc = "Field `ECCNMI` writer - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
pub type ECCNMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMCFG_M3IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
    #[inline(always)]
    pub fn eccnmi(&self) -> ECCNMI_R {
        ECCNMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<0> {
        SEIE_W::new(self)
    }
    #[doc = "Bit 1 - ECC double error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn deie(&mut self) -> DEIE_W<1> {
        DEIE_W::new(self)
    }
    #[doc = "Bit 3 - Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
    #[inline(always)]
    #[must_use]
    pub fn eccnmi(&mut self) -> ECCNMI_W<3> {
        ECCNMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMCFG memory 3 interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramcfg_m3ier](index.html) module"]
pub struct RAMCFG_M3IER_SPEC;
impl crate::RegisterSpec for RAMCFG_M3IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramcfg_m3ier::R](R) reader structure"]
impl crate::Readable for RAMCFG_M3IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramcfg_m3ier::W](W) writer structure"]
impl crate::Writable for RAMCFG_M3IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMCFG_M3IER to value 0"]
impl crate::Resettable for RAMCFG_M3IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
