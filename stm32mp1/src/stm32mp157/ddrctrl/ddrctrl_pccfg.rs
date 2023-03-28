#[doc = "Register `DDRCTRL_PCCFG` reader"]
pub struct R(crate::R<DDRCTRL_PCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_PCCFG` writer"]
pub struct W(crate::W<DDRCTRL_PCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCCFG_SPEC>;
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
impl From<crate::W<DDRCTRL_PCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GO2CRITICAL_EN` reader - GO2CRITICAL_EN"]
pub type GO2CRITICAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `GO2CRITICAL_EN` writer - GO2CRITICAL_EN"]
pub type GO2CRITICAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_PCCFG_SPEC, bool, O>;
#[doc = "Field `PAGEMATCH_LIMIT` reader - PAGEMATCH_LIMIT"]
pub type PAGEMATCH_LIMIT_R = crate::BitReader<bool>;
#[doc = "Field `PAGEMATCH_LIMIT` writer - PAGEMATCH_LIMIT"]
pub type PAGEMATCH_LIMIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PCCFG_SPEC, bool, O>;
#[doc = "Field `BL_EXP_MODE` reader - BL_EXP_MODE"]
pub type BL_EXP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `BL_EXP_MODE` writer - BL_EXP_MODE"]
pub type BL_EXP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_PCCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    pub fn go2critical_en(&self) -> GO2CRITICAL_EN_R {
        GO2CRITICAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    pub fn pagematch_limit(&self) -> PAGEMATCH_LIMIT_R {
        PAGEMATCH_LIMIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    pub fn bl_exp_mode(&self) -> BL_EXP_MODE_R {
        BL_EXP_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GO2CRITICAL_EN"]
    #[inline(always)]
    #[must_use]
    pub fn go2critical_en(&mut self) -> GO2CRITICAL_EN_W<0> {
        GO2CRITICAL_EN_W::new(self)
    }
    #[doc = "Bit 4 - PAGEMATCH_LIMIT"]
    #[inline(always)]
    #[must_use]
    pub fn pagematch_limit(&mut self) -> PAGEMATCH_LIMIT_W<4> {
        PAGEMATCH_LIMIT_W::new(self)
    }
    #[doc = "Bit 8 - BL_EXP_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn bl_exp_mode(&mut self) -> BL_EXP_MODE_W<8> {
        BL_EXP_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL port common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_pccfg](index.html) module"]
pub struct DDRCTRL_PCCFG_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_pccfg::R](R) reader structure"]
impl crate::Readable for DDRCTRL_PCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_pccfg::W](W) writer structure"]
impl crate::Writable for DDRCTRL_PCCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_PCCFG to value 0"]
impl crate::Resettable for DDRCTRL_PCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
