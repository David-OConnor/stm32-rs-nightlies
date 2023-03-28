#[doc = "Register `COMP_CFGR2` reader"]
pub struct R(crate::R<COMP_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_CFGR2` writer"]
pub struct W(crate::W<COMP_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_CFGR2_SPEC>;
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
impl From<crate::W<COMP_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPSEL0` reader - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL0_R = crate::BitReader<bool>;
#[doc = "Field `INPSEL0` writer - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR2_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn inpsel0(&self) -> INPSEL0_R {
        INPSEL0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - COMP non-inverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inpsel0(&mut self) -> INPSEL0_W<4> {
        INPSEL0_W::new(self)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR2\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_cfgr2](index.html) module"]
pub struct COMP_CFGR2_SPEC;
impl crate::RegisterSpec for COMP_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_cfgr2::R](R) reader structure"]
impl crate::Readable for COMP_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_cfgr2::W](W) writer structure"]
impl crate::Writable for COMP_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP_CFGR2 to value 0"]
impl crate::Resettable for COMP_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
