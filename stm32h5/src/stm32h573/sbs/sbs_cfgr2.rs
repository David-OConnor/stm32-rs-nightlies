#[doc = "Register `SBS_CFGR2` reader"]
pub struct R(crate::R<SBS_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_CFGR2` writer"]
pub struct W(crate::W<SBS_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_CFGR2_SPEC>;
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
impl From<crate::W<SBS_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLL` reader - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1/8/15/16/17 break inputs."]
pub type CLL_R = crate::BitReader<bool>;
#[doc = "Field `CLL` writer - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1/8/15/16/17 break inputs."]
pub type CLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CFGR2_SPEC, bool, O>;
#[doc = "Field `SEL` reader - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1/8/15/16/17."]
pub type SEL_R = crate::BitReader<bool>;
#[doc = "Field `SEL` writer - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1/8/15/16/17."]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CFGR2_SPEC, bool, O>;
#[doc = "Field `PVDL` reader - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1/8/15/16/17 break inputs."]
pub type PVDL_R = crate::BitReader<bool>;
#[doc = "Field `PVDL` writer - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1/8/15/16/17 break inputs."]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CFGR2_SPEC, bool, O>;
#[doc = "Field `ECCL` reader - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1/8/15/6/17."]
pub type ECCL_R = crate::BitReader<bool>;
#[doc = "Field `ECCL` writer - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1/8/15/6/17."]
pub type ECCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1/8/15/16/17 break inputs."]
    #[inline(always)]
    pub fn cll(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1/8/15/16/17."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1/8/15/16/17 break inputs."]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1/8/15/6/17."]
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1/8/15/16/17 break inputs."]
    #[inline(always)]
    #[must_use]
    pub fn cll(&mut self) -> CLL_W<0> {
        CLL_W::new(self)
    }
    #[doc = "Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1/8/15/16/17."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<1> {
        SEL_W::new(self)
    }
    #[doc = "Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1/8/15/16/17 break inputs."]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1/8/15/6/17."]
    #[inline(always)]
    #[must_use]
    pub fn eccl(&mut self) -> ECCL_W<3> {
        ECCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS Class B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_cfgr2](index.html) module"]
pub struct SBS_CFGR2_SPEC;
impl crate::RegisterSpec for SBS_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_cfgr2::R](R) reader structure"]
impl crate::Readable for SBS_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_cfgr2::W](W) writer structure"]
impl crate::Writable for SBS_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_CFGR2 to value 0"]
impl crate::Resettable for SBS_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
