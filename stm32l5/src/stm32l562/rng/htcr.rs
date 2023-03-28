#[doc = "Register `HTCR` reader"]
pub struct R(crate::R<HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTCR` writer"]
pub struct W(crate::W<HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTCR_SPEC>;
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
impl From<crate::W<HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTCFG` reader - health test configuration"]
pub type HTCFG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTCFG` writer - health test configuration"]
pub type HTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HTCR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    pub fn htcfg(&self) -> HTCFG_R {
        HTCFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - health test configuration"]
    #[inline(always)]
    #[must_use]
    pub fn htcfg(&mut self) -> HTCFG_W<0> {
        HTCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htcr](index.html) module"]
pub struct HTCR_SPEC;
impl crate::RegisterSpec for HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htcr::R](R) reader structure"]
impl crate::Readable for HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htcr::W](W) writer structure"]
impl crate::Writable for HTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTCR to value 0x000c_aa74"]
impl crate::Resettable for HTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_aa74;
}
