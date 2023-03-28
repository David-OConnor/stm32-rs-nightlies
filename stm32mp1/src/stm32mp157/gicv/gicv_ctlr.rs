#[doc = "Register `GICV_CTLR` reader"]
pub struct R(crate::R<GICV_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICV_CTLR` writer"]
pub struct W(crate::W<GICV_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICV_CTLR_SPEC>;
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
impl From<crate::W<GICV_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICV_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLEGRP0` reader - ENABLEGRP0"]
pub type ENABLEGRP0_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEGRP0` writer - ENABLEGRP0"]
pub type ENABLEGRP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICV_CTLR_SPEC, bool, O>;
#[doc = "Field `ENABLEGRP1` reader - ENABLEGRP1"]
pub type ENABLEGRP1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLEGRP1` writer - ENABLEGRP1"]
pub type ENABLEGRP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICV_CTLR_SPEC, bool, O>;
#[doc = "Field `ACKCTL` reader - ACKCTL"]
pub type ACKCTL_R = crate::BitReader<bool>;
#[doc = "Field `ACKCTL` writer - ACKCTL"]
pub type ACKCTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICV_CTLR_SPEC, bool, O>;
#[doc = "Field `FIQEN` reader - FIQEN"]
pub type FIQEN_R = crate::BitReader<bool>;
#[doc = "Field `FIQEN` writer - FIQEN"]
pub type FIQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICV_CTLR_SPEC, bool, O>;
#[doc = "Field `CBPR` reader - CBPR"]
pub type CBPR_R = crate::BitReader<bool>;
#[doc = "Field `CBPR` writer - CBPR"]
pub type CBPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICV_CTLR_SPEC, bool, O>;
#[doc = "Field `EOIMODE` reader - EOIMODE"]
pub type EOIMODE_R = crate::BitReader<bool>;
#[doc = "Field `EOIMODE` writer - EOIMODE"]
pub type EOIMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICV_CTLR_SPEC, bool, O>;
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
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    pub fn ackctl(&self) -> ACKCTL_R {
        ACKCTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    pub fn fiqen(&self) -> FIQEN_R {
        FIQEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    pub fn cbpr(&self) -> CBPR_R {
        CBPR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - EOIMODE"]
    #[inline(always)]
    pub fn eoimode(&self) -> EOIMODE_R {
        EOIMODE_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    #[must_use]
    pub fn ackctl(&mut self) -> ACKCTL_W<2> {
        ACKCTL_W::new(self)
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    #[must_use]
    pub fn fiqen(&mut self) -> FIQEN_W<3> {
        FIQEN_W::new(self)
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    #[must_use]
    pub fn cbpr(&mut self) -> CBPR_W<4> {
        CBPR_W::new(self)
    }
    #[doc = "Bit 9 - EOIMODE"]
    #[inline(always)]
    #[must_use]
    pub fn eoimode(&mut self) -> EOIMODE_W<9> {
        EOIMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICV virtual machine control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_ctlr](index.html) module"]
pub struct GICV_CTLR_SPEC;
impl crate::RegisterSpec for GICV_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_ctlr::R](R) reader structure"]
impl crate::Readable for GICV_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicv_ctlr::W](W) writer structure"]
impl crate::Writable for GICV_CTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICV_CTLR to value 0"]
impl crate::Resettable for GICV_CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
