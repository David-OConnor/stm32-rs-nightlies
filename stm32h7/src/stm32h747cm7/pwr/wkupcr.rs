#[doc = "Register `WKUPCR` reader"]
pub struct R(crate::R<WKUPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUPCR` writer"]
pub struct W(crate::W<WKUPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUPCR_SPEC>;
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
impl From<crate::W<WKUPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPC` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPC` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WKUPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WKUPCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc(&self) -> WKUPC_R {
        WKUPC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn wkupc(&mut self) -> WKUPC_W<0> {
        WKUPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupcr](index.html) module"]
pub struct WKUPCR_SPEC;
impl crate::RegisterSpec for WKUPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkupcr::R](R) reader structure"]
impl crate::Readable for WKUPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkupcr::W](W) writer structure"]
impl crate::Writable for WKUPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKUPCR to value 0"]
impl crate::Resettable for WKUPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
