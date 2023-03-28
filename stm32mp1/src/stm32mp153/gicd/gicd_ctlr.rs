#[doc = "Register `GICD_CTLR` reader"]
pub struct R(crate::R<GICD_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_CTLR` writer"]
pub struct W(crate::W<GICD_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_CTLR_SPEC>;
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
impl From<crate::W<GICD_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLEGRP0` reader - ENABLEGRP0"]
pub type ENABLEGRP0_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEGRP0` writer - ENABLEGRP0"]
pub type ENABLEGRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_CTLR_SPEC, bool, O>;
#[doc = "Field `ENABLEGRP1` reader - ENABLEGRP1"]
pub type ENABLEGRP1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEGRP1` writer - ENABLEGRP1"]
pub type ENABLEGRP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_CTLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    pub fn enablegrp0(&self) -> ENABLEGRP0_R {
        ENABLEGRP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    pub fn enablegrp1(&self) -> ENABLEGRP1_R {
        ENABLEGRP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp0(&mut self) -> ENABLEGRP0_W<0> {
        ENABLEGRP0_W::new(self)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    #[must_use]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W<1> {
        ENABLEGRP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ctlr](index.html) module"]
pub struct GICD_CTLR_SPEC;
impl crate::RegisterSpec for GICD_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ctlr::R](R) reader structure"]
impl crate::Readable for GICD_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ctlr::W](W) writer structure"]
impl crate::Writable for GICD_CTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_CTLR to value 0"]
impl crate::Resettable for GICD_CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
