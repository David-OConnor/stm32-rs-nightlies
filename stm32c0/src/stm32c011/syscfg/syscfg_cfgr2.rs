#[doc = "Register `SYSCFG_CFGR2` reader"]
pub struct R(crate::R<SYSCFG_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CFGR2` writer"]
pub struct W(crate::W<SYSCFG_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CFGR2_SPEC>;
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
impl From<crate::W<SYSCFG_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKUP_LOCK` reader - Cortex<Superscript>�<Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript>�<Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
pub type LOCKUP_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP_LOCK` writer - Cortex<Superscript>�<Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript>�<Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
pub type LOCKUP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Cortex<Superscript>�<Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript>�<Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex<Superscript>�<Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript>�<Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<0> {
        LOCKUP_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cfgr2](index.html) module"]
pub struct SYSCFG_CFGR2_SPEC;
impl crate::RegisterSpec for SYSCFG_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cfgr2::R](R) reader structure"]
impl crate::Readable for SYSCFG_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cfgr2::W](W) writer structure"]
impl crate::Writable for SYSCFG_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR2 to value 0"]
impl crate::Resettable for SYSCFG_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
