#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGSEC` reader - SYSCFG clock control security"]
pub type SYSCFGSEC_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGSEC` writer - SYSCFG clock control security"]
pub type SYSCFGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `CLASSBSEC` reader - ClassB security"]
pub type CLASSBSEC_R = crate::BitReader<bool>;
#[doc = "Field `CLASSBSEC` writer - ClassB security"]
pub type CLASSBSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `SRAM2SEC` reader - SRAM2 security"]
pub type SRAM2SEC_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2SEC` writer - SRAM2 security"]
pub type SRAM2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
#[doc = "Field `FPUSEC` reader - FPUSEC"]
pub type FPUSEC_R = crate::BitReader<bool>;
#[doc = "Field `FPUSEC` writer - FPUSEC"]
pub type FPUSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    pub fn syscfgsec(&self) -> SYSCFGSEC_R {
        SYSCFGSEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    pub fn sram2sec(&self) -> SRAM2SEC_R {
        SRAM2SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG clock control security"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgsec(&mut self) -> SYSCFGSEC_W<0> {
        SYSCFGSEC_W::new(self)
    }
    #[doc = "Bit 1 - ClassB security"]
    #[inline(always)]
    #[must_use]
    pub fn classbsec(&mut self) -> CLASSBSEC_W<1> {
        CLASSBSEC_W::new(self)
    }
    #[doc = "Bit 2 - SRAM2 security"]
    #[inline(always)]
    #[must_use]
    pub fn sram2sec(&mut self) -> SRAM2SEC_W<2> {
        SRAM2SEC_W::new(self)
    }
    #[doc = "Bit 3 - FPUSEC"]
    #[inline(always)]
    #[must_use]
    pub fn fpusec(&mut self) -> FPUSEC_W<3> {
        FPUSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
